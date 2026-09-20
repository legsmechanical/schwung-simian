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

use std::collections::HashMap;
use std::vec::Vec;

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;

use nih_plug::prelude::*;

use faust_types::{FaustDsp, ParamIndex, UI};

use default_boxed::DefaultBoxed;

use crate::buffer::{
    ResizableBuffer,
    BufferExtras,
};


/// Tracks a DSP to determin if it should be considered active or or inactive
pub struct SilenceTracker {
    threshold_samples: u32,
    counter: u32,
}

impl SilenceTracker {
    fn default() -> Self {
        Self {
            threshold_samples: 0,
            counter: 0,
        }
    }

    /// Initialize the SilenceTracker with a set samples threshold
    pub fn initialize(&mut self, threshold_samples: u32) {
        self.threshold_samples = threshold_samples;
        self.set_inactive();
    }

    /// Update the tracker's state by reading a number of frames
    pub fn update(
        &mut self,
        frames: usize,
        buffer: &[&mut [f32]], //mutable now for laziness because the buffers we're testing are already mutable
    ) {
        if self.threshold_samples > 0 && self.is_active() {
            let channel_count = buffer.len();
            let silence_frame_skip = 1; // Doesn't work yet...

            if channel_count == 1 {
                for i in 0..frames {
                    if (buffer[0][i]).abs() > 0.001 {
                        self.counter = 0;
                    } else {
                        self.counter += silence_frame_skip;
                    }
                }
            } else {
                let mut ch = 0;
                for i in 0..frames {
                    ch = (ch + 1) % channel_count;
                    if (buffer[ch][i]).abs() > 0.001
                    //Cycle Channels to coarsely estimate silence
                    {
                        self.counter = 0;
                    } else {
                        self.counter += silence_frame_skip;
                    }
                }
            }
        }
    }

    /// Force the tracker to be considered inactive
    pub fn set_inactive(&mut self) {
        self.counter = self.threshold_samples; // Ensure we skip any initial smoothing
    }

    /// Force the tracker to be considered active
    pub fn set_active(&mut self) -> bool {
        let just_activated = !self.is_active();
        self.counter = 0;
        just_activated
    }

    /// Returns true if this tracker is active
    pub fn is_active(&self) -> bool {
        self.threshold_samples == 0 || self.counter < self.threshold_samples
    }
}

pub struct OneTrickDSP<T>
where
    T: FaustDsp<T = f32>, // + 'static,
{
    dsp: Box<T>,
    params: ParamsBuilder,

    buffer: ResizableBuffer,
    sample_rate: usize,

    voice_id: Option<i32>,

    silence_tracker: SilenceTracker,
    compute_skipped: bool,
    computed_frames: usize,

    param_wake: Option<i32>,
    param_trigger: Option<i32>,
    param_transpose: Option<i32>,
    param_pitch_wheel: Option<i32>,
    param_mod_wheel: Option<i32>,
    param_sustain: Option<i32>,
    param_choke: Option<i32>,
    param_hold: Option<i32>,
    // Basic Faust Midi:
    param_gate: Option<i32>,
    param_gain: Option<i32>,
    param_key: Option<i32>,
    param_freq: Option<i32>,
    param_vumeter_left: Option<i32>,
    param_vumeter_right: Option<i32>,

    trigger_indicator: Arc<AtomicBool>,
    vu_meter_left: Arc<AtomicI32>,
    vu_meter_right: Arc<AtomicI32>,

    param_gate_counter: i32,
    last_note: Option<f32>,

    sustain_active: bool,
    sostenuto_active: bool,
}

/// Provides functions for accessing Faust DSP properties, compute, etc
pub trait OneTrickDSPGeneral {
    fn initialize(&mut self, sample_rate: usize) -> &mut dyn OneTrickDSPGeneral;
    fn reset(&mut self);
    fn panic(&mut self);
    fn set_voice_id(&mut self, voice_id: Option<i32>);
    fn get_voice_id(&self) -> Option<i32>;
    fn setup_common_params(&mut self);
    fn track_silence(&mut self, threshold_ms: u32) -> &mut dyn OneTrickDSPGeneral;
    fn resize_buffer(&mut self, max_frames: usize) -> &mut dyn OneTrickDSPGeneral;
    fn param_index(&self, path: &str) -> Option<i32>;
    fn get_param(&mut self, index: i32) -> Option<f32>;
    fn set_param(&mut self, index: i32, value: f32);
    fn get_param_by_name(&mut self, path: &str) -> Option<f32>;
    fn set_param_by_name(&mut self, path: &str, value: f32);
    fn compute(&mut self, count: usize, inputs: &[&[f32]]);
    fn compute_to(&mut self, count: usize, inputs: &[&[f32]], outputs: Option<&mut [&mut [f32]]>);
    fn write_to_buffer(&self, to_buffer: &mut Buffer);
    fn add_to_buffer(&self, to_buffer: &mut Buffer);
    fn write_to_slice(&self, to_buffer: &mut [&mut [f32]]);
    fn add_to_slice(&self, to_buffer: &mut [&mut [f32]]);
    fn burn_sample(&mut self);
    fn wake_up(&mut self);
    fn trigger(&mut self, velocity: f32);
    fn transpose(&mut self, value: f32);
    fn pitch_wheel(&mut self, value: f32);
    fn mod_wheel(&mut self, value: f32);
    fn update_sustain(&mut self);
    fn sustain(&mut self, value: bool);
    fn sostenuto(&mut self, value: bool);
    fn get_trigger_indicator(&mut self) -> Arc<AtomicBool>;
    fn get_vu_meter_left(&mut self) -> Arc<AtomicI32>;
    fn get_vu_meter_right(&mut self) -> Arc<AtomicI32>;
    fn note_on(&mut self, note_number: f32, velocity: f32);
    fn note_off(&mut self);
    fn choke(&mut self);
    fn hold(&mut self, toggle: bool);
    fn compute_ran(&self) -> bool;
    fn skip_compute(&mut self);
    fn is_active(&self) -> bool;
    fn params(&self) -> &ParamsBuilder;
    fn params_mut(&mut self) -> &mut ParamsBuilder;
    fn update_params(&mut self);
    fn get_last_note(&self) -> Option<f32>;
    fn get_last_note_u8(&self) -> Option<u8>;
}

impl<T> Default for OneTrickDSP<T>
where
    T: FaustDsp<T = f32> + 'static + DefaultBoxed,
{
    fn default() -> Self {
        Self::new()
    }
}

#[allow(unused)]
impl<T> OneTrickDSP<T>
where
    T: FaustDsp<T = f32> + 'static + DefaultBoxed,
{
    /// Creates a new DSP wrapper from a particular DSP type
    pub fn new() -> Self {
        //let (dsp, state) = DspHandle::<T>::new();
        let mut dsp = T::default_boxed();
        let params = ParamsBuilder::from_dsp(dsp.as_mut());

        //let sample_rate = buffer_config.sample_rate as i32;

        let buffer = ResizableBuffer::default();
        //let input_buffer = ResizableBuffer::default();

        let mut this = {
            Self {
                dsp,
                params,

                buffer,
                sample_rate: 1, // Invalid until initilized

                voice_id: None,

                silence_tracker: SilenceTracker::default(),
                compute_skipped: false,
                computed_frames: 0,

                param_wake: None,
                param_trigger: None,
                param_transpose: None,
                param_pitch_wheel: None,
                param_mod_wheel: None,
                param_sustain: None,
                param_choke: None,
                param_hold: None,

                // Basic Faust Midi:
                param_gate: None,
                param_gain: None,
                param_key: None,
                param_freq: None,
                param_vumeter_left: None,
                param_vumeter_right: None,

                trigger_indicator: Arc::new(AtomicBool::new(false)),
                vu_meter_left: Arc::new(AtomicI32::new(0)),
                vu_meter_right: Arc::new(AtomicI32::new(0)),
                param_gate_counter: 0,
                last_note: None,

                sustain_active: false,
                sostenuto_active: false,
            }
        };

        this.setup_common_params();

        this
    }

    fn pre_compute(&mut self) {
        self.update_params();
    }


    fn post_compute(&mut self) {
        self.untrigger();
        if let Some(index) = self.param_vumeter_left {
            if let Some(value) = self.get_param(index) {
                if value > 0.01 {
                    self.silence_tracker.set_active(); // Keep alive while vu meter is active
                }
                self.vu_meter_left.store((value * 1000.0) as i32, Ordering::Relaxed);
            }
        }
        if let Some(index) = self.param_vumeter_right {
            if let Some(value) = self.get_param(index) {
                if value > 0.01 {
                    self.silence_tracker.set_active(); // Keep alive while vu meter is active
                }
                self.vu_meter_right.store((value * 1000.0) as i32, Ordering::Relaxed);
            }
        }
    }

    fn note_to_freq(note_number: f32) -> f32 {
        440.0 * (2.0_f32).powf((note_number - 69.0) / 12.0)
    }

    fn untrigger(&mut self) {
        if let Some(index) = self.param_trigger {
            self.set_param(index, 0.0);
        }
    }

    fn unchoke(&mut self) {
        if let Some(index) = self.param_choke {
            self.set_param(index, 0.0);
        }
    }

    fn set_wake(&mut self, value: f32) {
        if let Some(index) = self.param_wake {
            self.set_param(index, value);
        }
    }
}

impl<T> OneTrickDSP<T>
where
    T: FaustDsp<T = f32> + 'static + DefaultBoxed,
{
    fn note_off_internal(&mut self) {
        if self.param_gate_counter == 0 {
            self.untrigger();
            if let Some(index) = self.param_gate {
                self.set_param(index, 0.0);
            }
        }
    }
}
impl<T> OneTrickDSPGeneral for OneTrickDSP<T>
where
    T: FaustDsp<T = f32> + 'static + DefaultBoxed,
{
    fn initialize(&mut self, sample_rate: usize) -> &mut dyn OneTrickDSPGeneral {
        if self.sample_rate != sample_rate{
            self.sample_rate = sample_rate;
            self.dsp.init(sample_rate as i32);
        }
        self
    }

    fn reset(&mut self) {
        // Should "reset buffers and envelopes" here without allocating
        self.dsp.instance_init(self.sample_rate as i32);
        self.panic();
    }

    fn panic(&mut self) {
        self.sustain(false);
        self.sostenuto(false);
        self.note_off();
        self.param_gate_counter = 0;
    }

    fn set_voice_id(&mut self, voice_id: Option<i32>) {
        self.voice_id = voice_id;
    }

    fn get_voice_id(&self) -> Option<i32> {
        self.voice_id
    }

    fn setup_common_params(&mut self) {
        self.param_gate = self.param_index("gate");
        self.param_gain = self.param_index("gain");
        self.param_key = self.param_index("key");
        self.param_freq = self.param_index("freq");

        self.param_wake = self.param_index("WakeUp");
        self.param_trigger = self.param_index("Trigger");
        self.param_transpose = self.param_index("Transpose");
        self.param_pitch_wheel = self.param_index("PitchWheel");
        self.param_mod_wheel = self.param_index("ModWheel");
        self.param_sustain = self.param_index("Sustain");
        self.param_choke = self.param_index("Choke");
        self.param_hold = self.param_index("Hold");
        self.param_vumeter_left = self.param_index("VuMeterLeft");
        self.param_vumeter_right = self.param_index("VuMeterRight");
    }

    fn resize_buffer(&mut self, max_frames: usize) -> &mut dyn OneTrickDSPGeneral {
        let channel_count = self.dsp.get_num_outputs() as usize;
        self.buffer.resize(channel_count, max_frames);
        self
    }

    fn track_silence(&mut self, threshold_ms: u32) -> &mut dyn OneTrickDSPGeneral {
        self.silence_tracker
            .initialize(threshold_ms * (self.sample_rate as u32) / 1000_u32);
        self
    }

    fn param_index(&self, name: &str) -> Option<i32> {
        for (idx, element) in self.params.params.iter().enumerate() {
            if element.name() == name {
                return Some(idx as i32);
            }
        }
        None
    }

    fn get_param(&mut self, index: i32) -> Option<f32> {
        self.dsp.get_param(ParamIndex(index))
    }
    fn set_param(&mut self, index: i32, value: f32) {
        if index > -1 {
            self.dsp.set_param(ParamIndex(index), value);
        }
    }

    fn get_param_by_name(&mut self, name: &str) -> Option<f32> {
        if let Some(index) = self.param_index(name) {
            self.dsp.get_param(ParamIndex(index))
        } else {
            None
        }
    }
    fn set_param_by_name(&mut self, name: &str, value: f32) {
        if let Some(index) = self.param_index(name) {
            self.set_param(index, value);
        }
    }

    fn compute(&mut self, count: usize, inputs: &[&[f32]]) {
        self.compute_to(count, inputs, None);
    }

    fn compute_to(&mut self, count: usize, inputs: &[&[f32]], outputs: Option<&mut [&mut [f32]]>) {
        if self.silence_tracker.is_active() {
            self.compute_skipped = false;
            self.computed_frames = count;
            self.pre_compute();

            // TODO: Should we fix denomals like rust-faust?
            if let Some(outputs) = outputs {
                self.dsp.compute(count as i32, inputs, outputs);
                self.silence_tracker.update(count, outputs);
            } else {
                let outputs = self.buffer.buffer.as_slice();
                self.dsp.compute(count as i32, inputs, outputs);
                self.silence_tracker.update(count, outputs);
            }
            
            self.post_compute();
        } else {
            self.compute_skipped = true;
            self.computed_frames = 0
        }
    }

    fn write_to_buffer(&self, to_buffer: &mut Buffer) //, to_offset: i32
    {
        if self.compute_skipped {
            to_buffer.clear();
        } else {
            self.buffer.buffer
                .write_to_buffer_frames(to_buffer, self.computed_frames);
        }
    }

    fn add_to_buffer(&self, to_buffer: &mut Buffer) //, to_offset: i32
    {
        if self.compute_skipped {
            return;
        }
        self.buffer.buffer
            .add_to_buffer_frames(to_buffer, self.computed_frames);
    }

    fn write_to_slice(&self, to_buffer: &mut [&mut [f32]]) //, to_offset: i32
    {
        if self.compute_skipped {
            for channel in to_buffer.iter_mut() {
                channel.fill(0.0);
            }
        } else {
            self.buffer.buffer
                .write_to_slice_frames(to_buffer, self.computed_frames);
        }
    }

    fn add_to_slice(&self, to_buffer: &mut [&mut [f32]]) //, to_offset: i32
    {
        if self.compute_skipped {
            return;
        }
        self.buffer.buffer
            .add_to_slice_frames(to_buffer, self.computed_frames);
    }

    fn burn_sample(&mut self) {
        self.compute(
            1,
            &[
                &[0.0], //[0.0,0.0,0.0,0.0] to allow for 4x supersampling...
                &[0.0],
                &[0.0],
                &[0.0],
                &[0.0],
                &[0.0],
                &[0.0],
                &[0.0],
            ],
        ); // 8-channel should be safe
    }

    fn wake_up(&mut self) {
        if self.silence_tracker.set_active() {
            // Hack: Process 1 sample with notes off to ensure they wake up properly in DSP
            self.note_off_internal();
            self.set_wake(0.0);
            self.burn_sample();
            self.set_wake(1.0);
        }
    }

    fn trigger(&mut self, velocity: f32) {
        self.trigger_indicator.store(true, Ordering::Relaxed);
        self.wake_up();
        self.unchoke();
        if let Some(index) = self.param_trigger {
            self.set_param(index, velocity);
        }
    }

    fn transpose(&mut self, value: f32) {
        if let Some(index) = self.param_transpose {
            self.set_param(index, value);
        }
    }

    fn pitch_wheel(&mut self, value: f32) {
        // Put into -1,1 range...
        //should we add dead zone for cruddy controllers?
        //*1.01).clamp(-1.0,1.0);
        let value = value * 2.0 - 1.0;
        if let Some(index) = self.param_pitch_wheel {
            self.set_param(index, value);
        }
    }
    
    fn mod_wheel(&mut self, value: f32) {
        if let Some(index) = self.param_mod_wheel {
            self.set_param(index, value);
        }
    }

    fn update_sustain(&mut self) {
        if let Some(index) = self.param_sustain {
            let value = self.sustain_active || self.sostenuto_active;
            self.set_param(index, if value {1.0} else {0.0});
        }
    }

    fn sustain(&mut self, value: bool) {
        self.sustain_active = value;
        self.update_sustain();
    }

    fn sostenuto(&mut self, value: bool) {
        self.sostenuto_active = value;
        self.update_sustain();
    }

    fn get_trigger_indicator(&mut self) -> Arc<AtomicBool> {
        self.trigger_indicator.clone()
    }
    fn get_vu_meter_left(&mut self) -> Arc<AtomicI32> {
        self.vu_meter_left.clone()
    }
    fn get_vu_meter_right(&mut self) -> Arc<AtomicI32> {
        self.vu_meter_right.clone()
    }

    fn note_on(&mut self, note_number: f32, velocity: f32) {
        self.param_gate_counter += 1;
        self.trigger(velocity);
        if let Some(index) = self.param_gate {
            self.set_param(index, 1.0);
        }
        if let Some(index) = self.param_gain {
            self.set_param(index, velocity);
        }
        if let Some(index) = self.param_key {
            self.set_param(index, note_number);
        }
        if let Some(index) = self.param_freq {
            self.set_param(index, Self::note_to_freq(note_number));
        }
        self.last_note = Some(note_number);
    }

    fn note_off(&mut self) {
        self.param_gate_counter = (self.param_gate_counter - 1).max(0);
        self.note_off_internal();
    }

    fn choke(&mut self) {
        if let Some(index) = self.param_choke {
            self.set_param(index, 1.0);
        }
        self.note_off();
    }

    fn hold(&mut self, toggle: bool) {
        if let Some(index) = self.param_hold {
            self.set_param(index, if toggle {1.0} else {0.0})
        }
    }

    fn compute_ran(&self) -> bool {
        !self.compute_skipped
    }

    fn skip_compute(&mut self) {
        self.compute_skipped = true;
    }

    fn is_active(&self) -> bool {
        self.silence_tracker.is_active()
    }

    fn params(&self) -> &ParamsBuilder {
        &self.params
    }

    fn params_mut(&mut self) -> &mut ParamsBuilder {
        &mut self.params
    }

    fn update_params(&mut self) {
        for (idx, param) in self.params.params.iter_mut().enumerate() {
            if let Some(plugin_param) = param.plugin_param {
                match plugin_param {
                    ParamPtr::BoolParam(p) => unsafe {
                        self.dsp
                            .set_param(ParamIndex(idx as i32), if (*p).value() { 1.0 } else { 0.0 })
                    },
                    ParamPtr::FloatParam(p) => unsafe {
                        self.dsp.set_param(ParamIndex(idx as i32), (*p).value())
                    },
                    ParamPtr::IntParam(p) => unsafe {
                        self.dsp
                            .set_param(ParamIndex(idx as i32), (*p).value() as f32)
                    },
                    //                        ParamPtr::BoolParam(p) => { self.set_param(*idx, if (*p).value() {1.0} else {0.0}); }
                    //                        ParamPtr::FloatParam(p) => { self.set_param(*idx, (*p).value()); }
                    //                        ParamPtr::IntParam(p) => { self.set_param(*idx, (*p).value() as f32); }
                    _ => {}
                }
            }
        }
    }

    fn get_last_note(&self) -> Option<f32> {
        self.last_note
    }

    fn get_last_note_u8(&self) -> Option<u8> {
        self.last_note.map(|last_note| (last_note + 0.5) as u8)
    }

}

/// Holds values for a Faust Slider or NumEntry
#[derive(Clone)]
pub struct ElementInput {
    init: f32,
    min: f32,
    max: f32,
    step: f32,
}
impl ElementInput {
    fn new(init: f32, min: f32, max: f32, step: f32) -> Self {
        ElementInput {
            init,
            min,
            max,
            step,
        }
    }
}

/// Holds values for a Faust Bargraph
#[derive(Clone)]
pub struct ElementOutput {
    min: f32,
    max: f32,
}
impl ElementOutput {
    fn new(min: f32, max: f32) -> Self {
        ElementOutput { min, max }
    }
}

/// Represents all the Faust Input and Output type specific data
#[derive(Clone)]
pub enum ElementType {
    Unknown,
    Button,
    Toggle,
    VerticalSlider(ElementInput),
    HorizontalSlider(ElementInput),
    NumEntry(ElementInput),
    HorizontalBargraph(ElementOutput),
    VerticalBargraph(ElementOutput),
}


/// Holds a Faust Element with Metadata, and links that with a Plugin Parameter
#[derive(Clone)]
pub struct Element {
    name: String,
    element_type: ElementType,
    metadata: HashMap<String, String>,
    plugin_param: Option<ParamPtr>,
}

impl Element {
    /// Returns the Element's name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the Element's type
    pub fn element_type(&self) -> &ElementType {
        &self.element_type
    }

    /// Assigns a Plugin Parameter to this Element
    pub fn set_plugin_param(&mut self, plugin_param: ParamPtr) {
        self.plugin_param = Some(plugin_param);
    }

    /// Returns true if this is a Bargraph, or otherwise unknown
    pub fn is_passive(&self) -> bool {
        match &self.element_type {
            ElementType::Unknown => true,
            ElementType::Button => false,
            ElementType::Toggle => false,
            ElementType::VerticalSlider(_value) => false,
            ElementType::HorizontalSlider(_value) => false,
            ElementType::NumEntry(_value) => false,
            ElementType::HorizontalBargraph(_out) => true,
            ElementType::VerticalBargraph(_out) => true,
        }
    }

    /// Returns the Initial value of this Element
    pub fn init(&self) -> f32 {
        match &self.element_type {
            ElementType::VerticalSlider(value) => value.init,
            ElementType::HorizontalSlider(value) => value.init,
            ElementType::NumEntry(value) => value.init,
            _ => 0.0,
        }
    }

    /// Sets the Initial value of this Element
    pub fn set_init(&mut self, init: f32) {
        match &mut self.element_type {
            ElementType::VerticalSlider(value) => {
                value.init = init;
            }
            ElementType::HorizontalSlider(value) => {
                value.init = init;
            }
            ElementType::NumEntry(value) => {
                value.init = init;
            }
            _ => {}
        }
    }

    /// Returns the Minimum value of this Element
    pub fn min(&self) -> f32 {
        match &self.element_type {
            ElementType::VerticalSlider(value) => value.min,
            ElementType::HorizontalSlider(value) => value.min,
            ElementType::NumEntry(value) => value.min,
            ElementType::HorizontalBargraph(out) => out.min,
            ElementType::VerticalBargraph(out) => out.min,
            _ => 0.0,
        }
    }

    /// Returns the Maximum value of this Element
    pub fn max(&self) -> f32 {
        match &self.element_type {
            ElementType::VerticalSlider(value) => value.max,
            ElementType::HorizontalSlider(value) => value.max,
            ElementType::NumEntry(value) => value.max,
            ElementType::HorizontalBargraph(out) => out.max,
            ElementType::VerticalBargraph(out) => out.max,
            _ => 1.0,
        }
    }

    /// Returns the Step size of this Element
    pub fn step(&self) -> f32 {
        match &self.element_type {
            ElementType::VerticalSlider(value) => value.step,
            ElementType::HorizontalSlider(value) => value.step,
            ElementType::NumEntry(value) => value.step,
            _ => 1.0,
        }
    }

    /// Returns the Unit string of this Element, or "" if not found
    pub fn unit(&self) -> &str {
        let key = "unit";
        if self.metadata.contains_key(key) {
            &self.metadata[key]
        } else {
            ""
        }
    }

    /// Returns a Meta value of this Element, these can be anything
    pub fn meta(&self, key: &str) -> Option<&str> {
        if self.metadata.contains_key(key) {
            Some(&self.metadata[key])
        } else {
            None
        }
    }

    /// Make non-static unit strings work with NIH-Plug's Param traits
    pub fn unit_static(&self) -> &'static str {
        // Solution 1: Make static by using mem::transmute()
        // We can't be 100% certain Nih-plug/host will never access parameters after the DSP unloads
        // So maybe this solution should be avoided...
        // let result: &'static str = unsafe { std::mem::transmute(self.unit()) };
        // result

        // Solution 2: Make static by using a table of static strings:
        // This is ugly, but guaranteed to be safe.
        // let's just play it safe for now...
        match self.unit() {
            "%" => "%",
            "x" => "x",
            "dB" => "dB",
            "db" => "db",
            "Hz" => "Hz",
            "hz" => "hz",
            "kHz" => "kHz",
            "khz" => "khz",
            "mHz" => "mHz",
            "mhz" => "mhz",
            "rpm" => "rpm",
            "ns" => "ns",
            "ms" => "ms",
            "sec" => "sec",
            "c" => "c",
            "cents" => "cents",
            "s" => "s",
            "st" => "st",
            "semi" => "semi",
            "oct" => "oct",
            "m" => "m",
            "cm" => "cm",
            "mm" => "mm",
            "in" => "\"",
            "ft" => "'",
            "bit" => "bit",
            _ => "",
        }
    }
}

/// Interfaces with a Faust DSP to collect the UI elements like Sliders, and Bargraphs
pub struct ParamsBuilder {
    params: Vec<Element>, // TODO: Replace with Vec<Element> and just use index in place of idx
}

impl ParamsBuilder {
    fn new() -> Self {
        Self { params: Vec::new() }
    }
    fn from_dsp<D>(dsp: &mut D) -> Self
    where
        D: FaustDsp<T = f32>,
    {
        let mut this = Self::new();
        dsp.build_user_interface(&mut this);
        this
    }

    fn setup_element(&mut self, name: &str, idx: ParamIndex, element_type: ElementType) {
        let idx = idx.0 as usize;
        if self.params.len() <= idx {
            let element = Element {
                name: name.to_string(),
                element_type,
                metadata: HashMap::new(),
                plugin_param: None,
            };
            self.params.resize(idx + 1, element);
        } else {
            let element = self.params.get_mut(idx).unwrap();
            element.name = name.to_string();
            element.element_type = element_type;
        }
    }

    /// Returns an immutable list of param Elements
    pub fn params(&self) -> &Vec<Element> {
        &self.params
    }

    /// Returns a mutable list of param Elements
    pub fn params_mut(&mut self) -> &mut Vec<Element> {
        &mut self.params
    }
}

impl UI<f32> for ParamsBuilder {
    // Groups
    fn open_tab_box(&mut self, _label: &str) {}
    fn open_horizontal_box(&mut self, _label: &str) {}
    fn open_vertical_box(&mut self, _label: &str) {}
    fn close_box(&mut self) {}

    // UI Widgets
    fn add_button(&mut self, label: &str, param: ParamIndex) {
        self.setup_element(label, param, ElementType::Button);
    }
    fn add_check_button(&mut self, label: &str, param: ParamIndex) {
        self.setup_element(label, param, ElementType::Toggle);
    }
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        let widget_type = ElementType::VerticalSlider(ElementInput::new(init, min, max, step));
        self.setup_element(label, param, widget_type);
    }
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        let widget_type = ElementType::HorizontalSlider(ElementInput::new(init, min, max, step));
        self.setup_element(label, param, widget_type);
    }
    fn add_num_entry(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: f32,
        min: f32,
        max: f32,
        step: f32,
    ) {
        let widget_type = ElementType::NumEntry(ElementInput::new(init, min, max, step));
        self.setup_element(label, param, widget_type);
    }

    // Passive
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: f32, max: f32) {
        let widget_type = ElementType::HorizontalBargraph(ElementOutput::new(min, max));
        self.setup_element(label, param, widget_type);
    }
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: f32, max: f32) {
        let widget_type = ElementType::VerticalBargraph(ElementOutput::new(min, max));
        self.setup_element(label, param, widget_type);
    }

    // Metadata
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str) {
        if let Some(param_index) = param {
            if !self.params.len() > param_index.0 as usize {
                self.setup_element("Unknown", param_index, ElementType::Unknown)
            }
            if let Some(element) = self.params.get_mut(param_index.0 as usize) {
                element.metadata.insert(key.to_string(), value.to_string());
            }
        }
    }
}


pub struct OneTrickVoices<T>
where
    T: FaustDsp<T = f32>, // + 'static,
{
    active_voices: Vec<OneTrickDSP<T>>,
    inactive_voices: Vec<OneTrickDSP<T>>,
    sostenuto_notes: Vec<u8>,
    prioritize_retrigger: bool,
    voice_stealing: bool,
    reset_stolen: bool,
}
#[allow(unused)]
impl<T> OneTrickVoices<T>
where
    T: FaustDsp<T = f32> + 'static + DefaultBoxed,
{
    pub fn new(voice_count: usize) -> Self {

        let mut active_voices = Vec::with_capacity(voice_count);

        let mut inactive_voices = Vec::new();
        for i in 0..voice_count {
            let mut voice = OneTrickDSP::new();
            inactive_voices.push(voice);
        }

        let sostenuto_notes = Vec::with_capacity(voice_count);


        Self {
            active_voices,
            inactive_voices,
            sostenuto_notes,
            prioritize_retrigger: true,
            voice_stealing: true,
            reset_stolen: true,
        }
    }

    pub fn reset(&mut self) {
        while self.active_voice_count() > 0 {
            let voice = self.active_voices.remove(0);
            self.inactive_voices.push(voice);

        }
        self.for_each_voice(|voice| {
            voice.reset();
        });
    }

    pub fn panic(&mut self) {
        while self.active_voice_count() > 0 {
            let voice = self.active_voices.remove(0);
            self.inactive_voices.push(voice);

        }
        self.for_each_voice(|voice| {
            voice.panic();
        });
    }

    pub fn prioritize_retrigger(mut self, value: bool) -> Self {
        self.prioritize_retrigger = value;
        self
    }

    fn voice_index_by_note(&self, note: u8) -> Option<usize> {
        self.active_voices.iter().position(|voice| {
            voice.get_last_note_u8().unwrap_or(0)==note
        })
    }

    fn is_note_active(&self, note: u8) -> bool {
        self.voice_index_by_note(note).is_some()
    }

    pub fn active_voices(&self) -> &Vec<OneTrickDSP<T>> {
        &self.active_voices
    }

    pub fn voice_by_id(&mut self, voice_id: Option<i32>) -> Option<&mut OneTrickDSP<T>> {
        voice_id?;
        
        if let Some(index) = self.active_voices.iter().position(|voice| {
            voice.get_voice_id() == voice_id
        }) {
            return Some(&mut self.active_voices[index]);
        }
        None
    }

    pub fn note_on(&mut self, note: u8, velocity: f32, voice_id: Option<i32>) -> bool {
        // Ignore duplicate note triggers
        if self.is_note_active(note) {
            return false;
        }
        
        // Attempt to reuse a voice that was last used for this exact note
        if self.prioritize_retrigger {
            if let Some(index) = self.inactive_voices.iter().position(|voice| {
                voice.get_last_note_u8().unwrap_or(0)==note
            }) {
                let mut voice = self.inactive_voices.remove(index);
                voice.set_voice_id(voice_id);
                voice.note_on(note as f32, velocity);
                self.active_voices.push(voice);
                return true;
            }
        }

        // Attempt to reuse the oldest voice, preferably not one using sostenuto
        if !self.inactive_voices.is_empty() {
            // Search for oldest voice not using sostenuto
            let index = if let Some(index) = self.inactive_voices.iter().position(|voice| {
                let note = voice.get_last_note_u8().unwrap_or(0);
                !self.sostenuto_notes.contains(&note)
            }) {
                index
            } else { // Every voice using sostenuto!? Just take the oldest one...
                0
            };
            let mut voice = self.inactive_voices.remove(index);
            let sostenuto = self.sostenuto_notes.contains(&note);
            voice.set_voice_id(voice_id);
            voice.sostenuto(sostenuto); // Restore sostenuto
            voice.note_on(note as f32, velocity);
            self.active_voices.push(voice);
            return true;
        }  else if self.voice_stealing { // NoVoice Stealing
            // Make the oldest (first) note the newest (last)
            let mut voice = self.active_voices.remove(0);
            let sostenuto = self.sostenuto_notes.contains(&note);
            voice.sostenuto(sostenuto); // Restore sostenuto
            voice.note_off();
            if self.reset_stolen {
                voice.reset();
            }
            voice.burn_sample();
            voice.set_voice_id(voice_id);
            voice.note_on(note as f32, velocity);
            self.active_voices.push(voice);
            return true;
        }
        false
    }

    pub fn note_off(&mut self, note: u8, voice_id: Option<i32>) {
        // No need to do a while-let-Some because we ensure no duplicate active voices
        if let Some(index) = self.voice_index_by_note(note) {
            let mut voice = self.active_voices.remove(index);
            voice.note_off();
            voice.set_voice_id(None);
            self.inactive_voices.push(voice); // Push to the end of the queue

        }
    }

    pub fn hold(&mut self, toggle: bool) {
        self.for_each_voice(|voice|{
            voice.hold(toggle);
        });
    }

    pub fn for_each_voice<VoiceFn>(&mut self, mut voice_fn: VoiceFn)
        where VoiceFn: FnMut(&mut OneTrickDSP<T>)
    {
        for voice in &mut self.active_voices {
            voice_fn(voice);
        }
        for voice in &mut self.inactive_voices {
            voice_fn(voice);
        }
    }

    pub fn active_voice_count(&self) -> usize {
        self.active_voices.len()
    }

    pub fn transpose(&mut self, value: f32) {
        self.for_each_voice(|voice| {
            voice.transpose(value);
        });
    }

    pub fn pitch_wheel(&mut self, value: f32) {
        self.for_each_voice(|voice| {
            voice.pitch_wheel(value);
        });
    }

    pub fn mod_wheel(&mut self, value: f32) {
        self.for_each_voice(|voice| {
            voice.mod_wheel(value);
        });
    }

    pub fn sustain(&mut self, value: bool) {
        self.for_each_voice(|voice| {
            voice.sustain(value);
        });
    }
    pub fn sostenuto(&mut self, value: bool) {
        if value {
            for voice in &mut self.active_voices {
                voice.sostenuto(true);
                if let Some(note) = voice.get_last_note_u8() {
                    if !self.sostenuto_notes.contains(&note) {
                        self.sostenuto_notes.push(note);
                    }
                }
            }
        } else {
            self.for_each_voice(|voice| {
                voice.sostenuto(false);
            });
            self.sostenuto_notes.clear();
        }
    }
}