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

/// Adds the ability to process audio in various ways using a closure
pub struct ProcessEx {}

impl ProcessEx {
    /// Calls the process_audio closure once for each block
    fn process_audio<AudioFn>(
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        block_size: usize,
        block_start: usize,
        block_end: usize,
        process_audio: &mut AudioFn,
    ) where
        AudioFn: FnMut(&mut Buffer, &mut AuxiliaryBuffers, usize, usize), // block_start, block_end
    {
        let num_samples: usize = block_end;
        let mut block_start = block_start;
        let mut block_end = (block_start + block_size).min(num_samples);
        while block_start < num_samples {
            process_audio(buffer, aux, block_start, block_end);
            block_start = block_end;
            block_end = (block_start + block_size).min(num_samples);
        }
    }

    /// Calls the process_audio closure once for each block
    /// Blocks are further divided on NoteOn and NoteOff events for sample-accurate synths
    pub fn process_split_notes<
        MidiFn,
        AudioFn,
        P: Plugin + nih_plug::prelude::Plugin<SysExMessage = SysexMsg>,
        SysexMsg,
    >(
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        block_size: usize,
        context: &mut impl ProcessContext<P>,
        process_midi: MidiFn,
        mut process_audio: AudioFn,
    ) -> ProcessStatus
    where
        MidiFn: Fn(NoteEvent<SysexMsg>),
        AudioFn: FnMut(&mut Buffer, &mut AuxiliaryBuffers, usize, usize),
        SysexMsg: std::marker::Copy, // block_start, block_end
    {
        let num_samples = buffer.samples();
        let mut processed_checkpoint: usize = 0;
        while let Some(event) = context.next_event() {
            process_midi(event);
            let split_requested = matches!(
                event,
                NoteEvent::NoteOn {
                    timing: _,
                    voice_id: _,
                    channel: _,
                    note: _,
                    velocity: _,
                } | NoteEvent::NoteOff {
                    timing: _,
                    voice_id: _,
                    channel: _,
                    note: _,
                    velocity: _,
                }
            );
            let event_timing = event.timing() as usize;
            if split_requested && event_timing > processed_checkpoint {
                Self::process_audio(
                    buffer,
                    aux,
                    block_size,
                    processed_checkpoint,
                    event_timing,
                    &mut process_audio,
                );
                processed_checkpoint = event_timing; // Update Checkpoint
            }
        }

        if num_samples > processed_checkpoint {
            Self::process_audio(
                buffer,
                aux,
                block_size,
                processed_checkpoint,
                num_samples,
                &mut process_audio,
            );
        }

        ProcessStatus::Normal
    }
}
