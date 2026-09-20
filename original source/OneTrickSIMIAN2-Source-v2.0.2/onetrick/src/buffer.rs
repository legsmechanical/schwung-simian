/*
   ┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱
   ┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱
   ┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
   ┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
   ┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
   ┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
    ━━━━━━━━━━━━━━━━━━━━━━━━━━

    Copyright (c) 2023 Punk Labs LLC

    This section is part of OneTrick

    OneTrick is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    OneTrick is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License along with
    OneTrick.  If not, see <http://www.gnu.org/licenses/>.
*/

use nih_plug::prelude::*;

/// A wrapper around Nih-plug's Buffer class to allow for easy allocation and resizing
#[derive(Default)]
pub struct ResizableBuffer {
    buffer_storage: Vec<Vec<f32>>,
    pub buffer: Buffer<'static>
}

impl ResizableBuffer {
    /// Resizes the internal Buffer
    pub fn resize(&mut self, channel_count: usize, sample_count: usize) {
        if self.buffer.channels() == channel_count && self.buffer.samples() == sample_count {
            return;
        }
        unsafe {
            self.buffer.set_slices(sample_count, |buffer_slices| {
                // Clear any previous pointers to our internal buffer storage
                buffer_slices.clear();
                // It's now safe to modify the internal buffer storage
                self.buffer_storage = vec![vec![0.0; sample_count]; channel_count];
                // Add new pointers to our internal buffer storage
                for channel_storage in self.buffer_storage.iter_mut()
                {
                    // SAFETY: buffer_storage should no longer be accessed directly after this:
                    buffer_slices.push(&mut *(channel_storage.as_mut_slice() as *mut [f32]));
                }
            });
        }
    }

    /// Sets the working slice of the internal buffer storage so we can ignore offsets
    pub fn set_working_block(&mut self, block_start: usize, block_end: usize) {
        unsafe {
            let block_length = block_end-block_start;
            self.buffer.set_slices(block_length, |buffer_slices| {
                // Swap out pointers to our internal buffer storage
                for (buffer_slice, channel_storage) in buffer_slices.iter_mut().zip(self.buffer_storage.iter_mut())
                {
                    *buffer_slice = &mut *(&mut channel_storage.as_mut_slice()[block_start..block_end] as *mut [f32]);
                }
            });
        }
    }

    /// Resets the working slice of the internal buffer
    pub fn reset_working_block(&mut self) {
        unsafe {
            self.buffer.set_slices(self.real_samples(), |buffer_slices| {
                // Swap out pointers to our internal buffer storage
                for (buffer_slice, channel_storage) in buffer_slices.iter_mut().zip(self.buffer_storage.iter_mut())
                {
                    *buffer_slice = &mut *(channel_storage.as_mut_slice() as *mut [f32]);
                }
            });
        }
    }

    /*
    pub fn as_slice(&self) -> &[&mut [f32]] {
        self.buffer.as_slice_immutable()
    }

    pub fn as_slice_mut(&mut self) -> &mut [&mut [f32]] {
        self.buffer.as_slice()
    }
    */

    /// Returns the channel count of the Buffer
    pub fn channels(&self) -> usize {
        self.buffer.channels()
    }

    /// Returns the samples/frame count of the Buffer
    pub fn samples(&self) -> usize {
        self.buffer.samples()
    }

    /// Returns the true samples/frame count of the internal buffer storage
    fn real_samples(&self) -> usize {
        if !self.buffer_storage.is_empty() { self.buffer_storage[0].len() } else { 0 }
    }
}

/// A buffer manager for supporting fixed-samplerate DSP
pub struct InternallyResampledBuffer {
    // /// Copy of External Buffer data
    // external_buffer_input: ResizableBuffer,

    /// Copy of External Buffer data
    external_buffer_output: ResizableBuffer,

    /// External Samplerate
    external_sample_rate: usize,

    // /// Buffer for Internally processing data
    // internal_buffer_input: ResizableBuffer,

    /// Buffer for Internally processing data
    internal_buffer_output: ResizableBuffer,

    /// Internal Samplerate
    internal_sample_rate: usize,

    /// Logical Playhead in internal samples before compute
    internal_playhead: f32,

    /// Size of internal samples relative to external samples
    internal_sample_rate_ratio: f32,
}

impl Default for InternallyResampledBuffer {
    fn default() -> Self {
        InternallyResampledBuffer {
            //external_buffer_input: ResizableBuffer::default(),
            external_buffer_output: ResizableBuffer::default(),
            external_sample_rate: 48000,
            //internal_buffer_input: ResizableBuffer::default(),
            internal_buffer_output: ResizableBuffer::default(),
            internal_sample_rate: 48000,
            internal_sample_rate_ratio: 1.0,
            internal_playhead: 0.0,
        }
    }
}

impl InternallyResampledBuffer {
    pub fn internal_sample_rate(&self) -> usize {
        self.internal_sample_rate
    }
    pub fn setup(&mut self, external_sample_rate: usize, internal_sample_rate: usize, input_channel_count: usize, output_channel_count: usize, sample_count: usize) {
        nih_debug_assert!(input_channel_count==0); // We don't support input yet
        self.external_sample_rate = external_sample_rate;
        self.internal_sample_rate = internal_sample_rate;
        self.internal_playhead = 0.0;
        self.internal_sample_rate_ratio = internal_sample_rate as f32 / external_sample_rate as f32;
        // I think the max theoretical samples required would be +2, but let's do +3 to be safe
        let required_internal_samples = (sample_count as f32 * self.internal_sample_rate_ratio) as usize + 3;
        let required_external_samples = sample_count;//(required_internal_samples as f32 / self.internal_sample_rate_ratio) as usize + 3;
        //self.external_buffer_input.resize(input_channel_count, required_external_samples);
        self.external_buffer_output.resize(output_channel_count, required_external_samples);
        //self.internal_buffer_input.resize(input_channel_count, required_internal_samples);
        self.internal_buffer_output.resize(output_channel_count, required_internal_samples);
    }
    pub fn process_samples<ProcessFn>(&mut self,
        sample_count: usize,
        _inputs: &[&[f32]],
        outputs: &mut[&mut[f32]],
        process_samples: &mut ProcessFn,
    )  where
    ProcessFn: FnMut(usize, &[&[f32]], &mut[&mut[f32]]), // Count, Input, Output
    {
        //let sample_count = buffer.samples();
        let requesting_external_samples = sample_count;
        let requesting_internal_samples = (sample_count as f32 * self.internal_sample_rate_ratio - self.internal_playhead) as usize + 1;
        /*
        // Append buffer to our External Buffer
        //buffer.write_to_buffer_range(&mut self.external_buffer.buffer, 0, sample_count, self.external_sample_count, self.external_sample_count+sample_count);
        let _has_input = if inputs.len() > 0 && inputs[0].len() > 0 {
            self.external_buffer_input.buffer.read_from_slice_range(inputs, self.external_sample_count, self.external_sample_count+sample_count, 0, sample_count);
            // TODO: Resample external_input -> internal_input
            // TODO: Real external sample count should be higher than requested to fill in the padded internal buffer
            self.external_buffer_input.buffer.copy_to_resampled(&mut self.internal_buffer_input.buffer, 0, requesting_external_samples, 0, requesting_internal_samples);
            true
        } else {
            false
        };
        */

        // TODO: Slice up self.internal_buffer_output to start/end at the correct samples
        let start_internal_sample = (self.internal_playhead.ceil()+0.1) as usize;
        let end_internal_sample = start_internal_sample+requesting_internal_samples;
        
        self.internal_buffer_output.set_working_block(start_internal_sample, end_internal_sample);
        process_samples(requesting_internal_samples, &[], self.internal_buffer_output.buffer.as_slice());
        self.internal_buffer_output.reset_working_block();

        let next_internal_playhead = self.internal_playhead + (requesting_external_samples as f32 * self.internal_sample_rate_ratio);

        // Copy from Internal buffer to External buffer
        self.external_buffer_output.buffer.fill_with_resampled(0, requesting_external_samples, &mut self.internal_buffer_output.buffer, self.internal_playhead, next_internal_playhead);
        // Write to the actual output buffer slice
        self.external_buffer_output.buffer.write_to_slice_frames(outputs, requesting_external_samples);

        self.internal_playhead = next_internal_playhead;
        // Remove all whole samples from internal_buffer_output since they've been used
        let truncate_internal_samples = self.internal_playhead as usize; // Round down
        if truncate_internal_samples > 0 {
            self.internal_buffer_output.buffer.move_samples_to_front(truncate_internal_samples, truncate_internal_samples+1);
            // Subtract truncated samples from internal playhead!
            self.internal_playhead -= truncate_internal_samples as f32;
        }
    }
}

/// Adds slicing to the built-in Buffer struct
pub trait BufferSlicer {
    /// Returns an immutable N ch slice of buffer data
    fn slice_range_mut<const N: usize>(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; N];
    /// Returns an immutable 1ch slice of buffer data (convenience)
    fn slice1ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 1];
    /// Returns an immutable 2ch slice of buffer data (convenience)
    fn slice2ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 2];
    /// Returns an immutable 3ch slice of buffer data (convenience)
    fn slice3ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 3];
    /// Returns an immutable 4ch slice of buffer data (convenience)
    fn slice4ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 4];

    /// Returns an immutable N ch slice of buffer data
    fn slice<const N: usize>(&self) -> [&[f32]; N];
    /// Returns an immutable 1ch slice of buffer data (convenience)
    fn slice1ch(&self) -> [&[f32]; 1];
    /// Returns an immutable 2ch slice of buffer data (convenience)
    fn slice2ch(&self) -> [&[f32]; 2];
    /// Returns an immutable 3ch slice of buffer data (convenience)
    fn slice3ch(&self) -> [&[f32]; 3];
    /// Returns an immutable 4ch slice of buffer data (convenience)
    fn slice4ch(&self) -> [&[f32]; 4];
}

impl<'buffer> BufferSlicer for Buffer<'buffer> {
    fn slice_range_mut<const N: usize>(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; N] {
        let slice = self.as_slice();
        let mut iter = slice.iter_mut().map(move |ch| &mut ch[start_index..end_index]);
        let result: [_; N] = std::array::from_fn(|_| iter.next().unwrap());
        // Ensure that iterator finished
        nih_debug_assert!(matches!(iter.next(), None));
        result
    }
    fn slice1ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 1] {
        self.slice_range_mut(start_index, end_index)
    }
    fn slice2ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 2] {
        self.slice_range_mut(start_index, end_index)
    }
    fn slice3ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 3] {
        self.slice_range_mut(start_index, end_index)
    }
    fn slice4ch_range_mut(&mut self, start_index: usize, end_index: usize) -> [&mut [f32]; 4] {
        self.slice_range_mut(start_index, end_index)
    }

    fn slice<const N: usize>(&self) -> [&[f32]; N] {
        let slice = self.as_slice_actually_immutable();
        let mut iter = slice.iter();
        let result: [_; N] = std::array::from_fn(|_| *iter.next().unwrap());
        // Ensure that iterator finished
        nih_debug_assert!(matches!(iter.next(), None));
        result
    }
    fn slice1ch(&self) -> [&[f32]; 1] {
        self.slice()
    }
    fn slice2ch(&self) -> [&[f32]; 2] {
        self.slice()
    }
    fn slice3ch(&self) -> [&[f32]; 3] {
        self.slice()
    }
    fn slice4ch(&self) -> [&[f32]; 4] {
        self.slice()
    }

}


/// Provides mechanisms for resampling one Buffer into another
pub trait BufferResampler {
    /*
    /// Copies and resamples data from another Buffer
    fn copy_from_resampled(&mut self, from: &mut Buffer, from_start: usize, from_end: usize, to_start: usize, to_end: usize);

    /// Copies and resamples data into another Buffer
    fn copy_to_resampled(&mut self, to: &mut Buffer, from_start: usize, from_end: usize, to_start: usize, to_end: usize);
    */
    fn fill_with_resampled(&mut self, into_start: usize, into_end: usize, source: &mut Buffer, from_start: f32, from_end:f32);

    /// Downsamples the buffer into the specified number of frames by averaging in chunks of N
    /// ie. downsample(10, 2) would result in 5 frames averaged in pairs from the original 10
    fn downsample(&mut self, frames: usize, n: usize);

    /// Upsamples the buffer into the specified number of frames by averaging in chunks of N
    /// ie. upsample(5, 2) would result in 10 frames stretched out from the original 5
    fn upsample(&mut self, frames: usize, n: usize);
}

impl<'buffer> BufferResampler for Buffer<'buffer> {
    /*
    fn copy_from_resampled(&mut self, from: &mut Buffer, from_start: usize, from_end: usize, to_start: usize, to_end: usize) {
        // Nearest-neightbor for now
        let from_range = from_end - from_start;
        let to_range = to_end - to_start;
        let from_step = from_range as f32 / to_range as f32;
        for (from_channel, to_channel) in from.as_slice().iter().zip(self.as_slice().iter_mut()) {
            for i_to in to_start..to_end {
                let i_from = (((i_to-to_start) as f32) * from_step) as usize;
                to_channel[i_to] = from_channel[i_from];
            }
        }
    }

    fn copy_to_resampled(&mut self, to: &mut Buffer, from_start: usize, from_end: usize, to_start: usize, to_end: usize) {
        to.copy_from_resampled(self, from_start, from_end, to_start, to_end);
    }
    */

    fn fill_with_resampled(&mut self, start_index: usize, end_index: usize, source: &mut Buffer, from_start: f32, from_end:f32) {
        // Linear filtering for now... Quadratic would be better.
        let from_range = from_end - from_start;
        let index_range = end_index - start_index;
        let from_step = from_range / index_range as f32;
        for (from_channel, to_channel) in source.as_slice().iter().zip(self.as_slice().iter_mut()) {
            for i in 0..index_range {
                let index = start_index + i;
                let from_index_f = (i as f32) * from_step + from_start;
                let from_a = from_index_f as usize;
                let from_b = from_a+1;
                let from_t = from_index_f.fract();
                let value = (from_channel[from_a]*(from_t-1.0)) + (from_channel[from_b]*from_t);
                to_channel[index] = value;
            }
        }
    }


    fn downsample(&mut self, frames: usize, n: usize) {
        for channel in self.as_slice().iter_mut() {
            for chunk in channel.chunks_exact_mut(n).take(frames) {
                let sum = chunk.iter().sum::<f32>();
                let avg = sum / n as f32;
                chunk[0] = avg;
            }
            for i in 0..frames {
                channel[i] = channel[i*n];
            }
        }
    }

    fn upsample(&mut self, frames: usize, n: usize) {
        for channel in self.as_slice().iter_mut() {
            for i in 0..frames*n {
                channel[i] = channel[i/n];
            }
        }
    }
}

/// Extra misc functions for Buffers
pub trait BufferExtras {
    /// Clears all frames with 0.0
    fn move_samples_to_front(&mut self, start: usize, end: usize);

    /// Clears the specified number of frames with 0.0
    fn clear(&mut self);

    /// Moves samples to the front of the buffer (used for live resampling)
    fn clear_frames(&mut self, frames: usize);

    /// Adds this Buffer's data to the specified Buffer
    fn add_to_buffer(&self, to_buffer: &mut Buffer);

    /// Adds a number of frames from this Buffer's data to the specified Buffer slice
    fn add_to_buffer_frames(&self, to_buffer: &mut Buffer, frames: usize);

    /// Overwrites the specified Buffer with this Buffer's data
    fn write_to_buffer(&self, to_buffer: &mut Buffer);

    /// Overwrites a number of frames of the specified Buffer with this Buffer's data
    fn write_to_buffer_frames(&self, to_buffer: &mut Buffer, frames: usize);

    /// Overwrites a range of frames of the specified Buffer with this Buffer's data
    fn write_to_buffer_range(&self, to_buffer: &mut Buffer, from_min: usize, from_max: usize, to_min: usize, to_max: usize);

    /// Overwrites a range of frames of the specified Buffer with this Buffer's data
    fn read_from_slice_range(&mut self, from_slice: &[&[f32]], from_min: usize, from_max: usize, to_min: usize, to_max: usize);

    /// Adds a number of frames from this Buffer's data to the specified Buffer slice
    fn add_to_slice_frames(&self, to_buffer: &mut [&mut [f32]], frames: usize);

    /// Overwrites a number of frames of the specified Buffer with this Buffer's data
    fn write_to_slice_frames(&self, to_buffer: &mut [&mut [f32]], frames: usize);

    fn as_slice_actually_immutable(&self) -> &[&[f32]];
}
impl<'buffer> BufferExtras for Buffer<'buffer> {
    
    fn clear(&mut self) {
        for channel in self.as_slice().iter_mut() {
            channel.fill(0.0);
        }
    }

    fn clear_frames(&mut self, frames: usize) {
        for channel in self.as_slice().iter_mut() {
            channel[0..frames].fill(0.0);
        }
    }

    fn move_samples_to_front(&mut self, start: usize, end: usize) {
        if start > 0 {
            //self.clear_frames(start);
            for channel in self.as_slice().iter_mut() {
                channel.copy_within(start..end, 0);
            }
        }

    }

    fn add_to_buffer(&self, to_buffer: &mut Buffer) {
        for (self_channel, to_channel) in self.as_slice_actually_immutable().iter().zip(to_buffer.as_slice().iter_mut()) {
            for (to, from) in to_channel.iter_mut().zip(self_channel.iter()) {
                *to += *from;
            }
        }
    }

    fn add_to_buffer_frames(&self, to_buffer: &mut Buffer, frames: usize) {
        for (self_channel, to_channel) in self.as_slice_actually_immutable().iter().zip(to_buffer.as_slice().iter_mut()) {
            for (to, from) in to_channel[0..frames].iter_mut().zip(self_channel.iter()) {
                *to += *from;
            }
        }
    }


    fn write_to_buffer(&self, to_buffer: &mut Buffer) {
        for (self_channel, to_channel) in self.as_slice_actually_immutable().iter().zip(to_buffer.as_slice().iter_mut()) {
            let len = to_channel.len().min(self_channel.len());
            to_channel[..len].copy_from_slice(&self_channel[..len]);
        }
    }

    fn write_to_buffer_frames(&self, to_buffer: &mut Buffer, frames: usize) {
        for (self_channel, to_channel) in self.as_slice_actually_immutable().iter().zip(to_buffer.as_slice().iter_mut()) {
            let len = to_channel.len().min(self_channel.len()).min(frames);
            to_channel[..len].copy_from_slice(
                &self_channel[..len],
            );
        }
    }

    fn write_to_buffer_range(&self, to_buffer: &mut Buffer, from_min: usize, from_max: usize, to_min: usize, to_max: usize) {
        for (self_channel, to_channel) in self.as_slice_actually_immutable().iter().zip(to_buffer.as_slice().iter_mut()) {
            to_channel[to_min..to_max].copy_from_slice(
                &self_channel[from_min..from_max],
            );
        }
    }
    fn read_from_slice_range(&mut self, from_slice: &[&[f32]], from_min: usize, from_max: usize, to_min: usize, to_max: usize) {
        for (self_channel, from_channel) in self.as_slice().iter_mut().zip(from_slice.iter()) {
            self_channel[to_min..to_max].copy_from_slice(
                &from_channel[from_min..from_max],
            );
        }
    }

    fn add_to_slice_frames(&self, to_buffer: &mut [&mut [f32]], frames: usize) {
        for (self_channel, to_channel) in self.as_slice_actually_immutable().iter().zip(to_buffer.iter_mut()) {
            for (to, from) in to_channel[0..frames].iter_mut().zip(self_channel.iter()) {
                *to += *from;
            }
        }
    }

    fn write_to_slice_frames(&self, to_buffer: &mut [&mut [f32]], frames: usize) {
        for (self_channel, to_channel) in self.as_slice_actually_immutable().iter().zip(to_buffer.iter_mut()) {
            let len = to_channel.len().min(self_channel.len()).min(frames);
            to_channel[..len].copy_from_slice(
                &self_channel[..len],
            );
        }
    }
    fn as_slice_actually_immutable(&self) -> &[&[f32]]
    {
        let data = self.as_slice_immutable();
        let data_immutable: &[&[f32]] = unsafe { std::mem::transmute(data) };
        data_immutable
    }
}


