/*
   ┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱
   ┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱
   ┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
   ┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
   ┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
   ┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
    ━━━━━━━━━━━━━━━━━━━━━━━━━━

    Copyright (c) 2024 Punk Labs LLC

    This section is part of OneTrick SIMIAN

    OneTrick SIMIAN is free software: you can redistribute it and/or modify it
    under the terms of the GNU General Public License as published by the Free
    Software Foundation, either version 3 of the License, or (at your option)
    any later version.

    OneTrick SIMIAN is distributed in the hope that it will be useful, but WITHOUT
    ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
    FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
    more details.

    You should have received a copy of the GNU General Public License along with
    OneTrick SIMIAN.  If not, see <http://www.gnu.org/licenses/>.
*/

use std::cell::RefCell;
//use std::rc::Rc;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::sync::Arc;

use nih_plug::prelude::*;

// #[cfg(feature = "egui")]
// use egui_extras::image::RetainedImage;

#[cfg(feature = "egui")]
use nih_plug_egui::{
    create_egui_editor,
    egui::{
        epaint::{
            Shadow,
        },
        style,
        Margin,
        vec2,
        //Align,
        Align2,
        Area,
        CentralPanel,
        Color32,
        Context,
        FontDefinitions,
        FontFamily,
        FontId,
        Frame,
        Id,
        //LayerId,
        //Layout,
        Order,
        Pos2,
        Rect,
        Rounding,
        Sense,
        Stroke,
        Vec2,
    },
    EguiState,
};

mod dsp;
use onetrick::prelude::*;

//256 is 187.5Hz at 48kHz
//512 is 94Hz at 48kHz
//1024 is 47Hz at 48kHz
const MAX_BLOCK_SIZE: usize = 4096;
const NUM_CHANNELS: u32 = 2;
#[cfg(feature = "multi_out")]
const AUX_BUS_COUNT: usize = 10;

// NOTE: Should probably use an array of &dyn OneTrickGeneral, but couldn't make it work
macro_rules! for_each_voice_dsp {
    ($this:ident, $expression:expr) => {{
        $expression(&mut *$this.dsp_kick.borrow_mut());
        $expression(&mut *$this.dsp_snare.borrow_mut());
        $expression(&mut *$this.dsp_rimshot.borrow_mut());
        $expression(&mut *$this.dsp_clap.borrow_mut());
        $expression(&mut *$this.dsp_lowtom.borrow_mut());
        $expression(&mut *$this.dsp_midtom.borrow_mut());
        $expression(&mut *$this.dsp_hitom.borrow_mut());
        $expression(&mut *$this.dsp_hihat_closed.borrow_mut());
        $expression(&mut *$this.dsp_hihat_open.borrow_mut());
        $expression(&mut *$this.dsp_cymbal.borrow_mut());
    }};
}

macro_rules! for_each_dsp {
    ($this:ident, $expression:expr) => {{
        for_each_voice_dsp!($this, $expression);
        $expression(&mut *$this.dsp_output.borrow_mut());
    }};
}

/// Main Struct for OneTrick SIMIAN
pub struct OneTrickSimian2 {
    params: Arc<OneTrickPluginParams>,
    //processor: FixedProcessor,

    accum_buffer: ResizableBuffer,

    dsp_output: RefCell<OneTrickDSP<dsp::modules::DSP_Output>>,
    dsp_kick: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_snare: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_rimshot: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_clap: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_lowtom: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_midtom: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_hitom: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_hihat_closed: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_hihat_open: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    dsp_cymbal: RefCell<OneTrickDSP<dsp::modules::DSP_Drum>>,
    param_global_sensitivity: usize,
    param_global_transpose: usize,

    kick_indicator: Arc<AtomicBool>,
    snare_indicator: Arc<AtomicBool>,
    rimshot_indicator: Arc<AtomicBool>,
    clap_indicator: Arc<AtomicBool>,
    lowtom_indicator: Arc<AtomicBool>,
    midtom_indicator: Arc<AtomicBool>,
    hitom_indicator: Arc<AtomicBool>,
    hihat_closed_indicator: Arc<AtomicBool>,
    hihat_open_indicator: Arc<AtomicBool>,
    cymbal_indicator: Arc<AtomicBool>,

    vu_meter_left: Arc<AtomicI32>,
    vu_meter_right: Arc<AtomicI32>,

    sample_rate: usize,
}

impl Default for OneTrickSimian2 {
    fn default() -> Self {
        
        let mut params = OneTrickPluginParams::default();
        if let Some(preset) = Preset::from_string("Basic", include_str!("assets/presets/Basic.preset")) {
            params.set_defaults_preset(preset);
        }
        // Preset::from_string("Basic", include_str!("assets/presets/Basic.preset")).apply
        #[cfg(feature = "egui")]
        {
            params.editor_state = EguiState::from_size(1120, 550);
        }

        let mut dsp_output = OneTrickDSP::new();
        let mut dsp_kick = OneTrickDSP::new();
        let mut dsp_snare = OneTrickDSP::new();
        let mut dsp_rimshot = OneTrickDSP::new();
        let mut dsp_clap = OneTrickDSP::new();
        let mut dsp_lowtom = OneTrickDSP::new();
        let mut dsp_midtom = OneTrickDSP::new();
        let mut dsp_hitom = OneTrickDSP::new();
        let mut dsp_hihat_closed = OneTrickDSP::new();
        let mut dsp_hihat_open = OneTrickDSP::new();
        let mut dsp_cymbal = OneTrickDSP::new();

        // ==== OVERRIDES ====
        // KICK
        // for p in dsp_kick.params_mut().params_mut() {
        //     if p.name() == "Tuning" { p.set_init(50.0); }
        // }

        params.append_dsp(&mut dsp_output, "Global");

        let param_global_sensitivity = params.append_float(
            FloatParam::new(
                "Global Sensitivity",
                100.0,
                FloatRange::Linear {
                    min: 0.0,
                    max: 100.0,
                },
            )
            .with_unit("%")
            .with_value_to_string(Arc::new(|v| {
                format!("{value:.precision$}", precision = 1, value = v)
            })),
            "Global",
        );
        let param_global_transpose = params.append_float(
            FloatParam::new(
                "Global Transpose",
                0.0,
                FloatRange::Linear {
                    min: -12.0,
                    max: 12.0,
                },
            )
            .with_unit("st")
            .with_value_to_string(Arc::new(|v| {
                format!("{value:.precision$}", precision = 1, value = v)
            })),
            "Global",
        );

        params.append_dsp(&mut dsp_kick, "Kick");
        params.append_dsp(&mut dsp_snare, "Snare");
        params.append_dsp(&mut dsp_rimshot, "Rimshot");
        params.append_dsp(&mut dsp_clap, "Clap");
        params.append_dsp(&mut dsp_lowtom, "Low Tom");
        params.append_dsp(&mut dsp_midtom, "Mid Tom");
        params.append_dsp(&mut dsp_hitom, "High Tom");
        params.append_dsp(&mut dsp_hihat_closed, "Hihat Closed");
        params.append_dsp(&mut dsp_hihat_open, "Hihat Open");
        params.append_dsp(&mut dsp_cymbal, "Cymbal");

        //let param_hihat_closed = dsp_hihat.param_index("Closed").unwrap();

        params.append_field_int("tab_index", 0);
        params.append_field_string("selected_preset", "");

        let kick_indicator = dsp_kick.get_trigger_indicator();
        let snare_indicator = dsp_snare.get_trigger_indicator();
        let rimshot_indicator = dsp_rimshot.get_trigger_indicator();
        let clap_indicator = dsp_clap.get_trigger_indicator();
        let lowtom_indicator = dsp_lowtom.get_trigger_indicator();
        let midtom_indicator = dsp_midtom.get_trigger_indicator();
        let hitom_indicator = dsp_hitom.get_trigger_indicator();
        let hihat_closed_indicator = dsp_hihat_closed.get_trigger_indicator();
        let hihat_open_indicator = dsp_hihat_open.get_trigger_indicator();
        let cymbal_indicator = dsp_cymbal.get_trigger_indicator();

        let vu_meter_left = dsp_output.get_vu_meter_left();
        let vu_meter_right = dsp_output.get_vu_meter_right();

        Self {
            params: Arc::new(params),

            accum_buffer: ResizableBuffer::default(),

            dsp_output: RefCell::new(dsp_output),
            dsp_kick: RefCell::new(dsp_kick),
            dsp_snare: RefCell::new(dsp_snare),
            dsp_rimshot: RefCell::new(dsp_rimshot),
            dsp_clap: RefCell::new(dsp_clap),
            dsp_lowtom: RefCell::new(dsp_lowtom),
            dsp_midtom: RefCell::new(dsp_midtom),
            dsp_hitom: RefCell::new(dsp_hitom),
            dsp_hihat_closed: RefCell::new(dsp_hihat_closed),
            dsp_hihat_open: RefCell::new(dsp_hihat_open),
            dsp_cymbal: RefCell::new(dsp_cymbal),

            param_global_sensitivity,
            param_global_transpose,

            kick_indicator,
            snare_indicator,
            rimshot_indicator,
            clap_indicator,
            lowtom_indicator,
            midtom_indicator,
            hitom_indicator,
            hihat_closed_indicator,
            hihat_open_indicator,
            cymbal_indicator,

            vu_meter_left,
            vu_meter_right,

            sample_rate: 1, // Invalid until initilized
        }
    }
}

impl OneTrickSimian2 {
    /// Set up fonts and default styles for Egui
    #[cfg(feature = "egui")]
    fn setup_egui_style(ctx: &Context) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = FontDefinitions::default();

        BasicFonts::add_fonts(&mut fonts);
        Icons::add_fonts(&mut fonts);

        let mut style = style::Style::default();

        // Setup defaults for using generic controls:
        let font_id = FontId::new(17.0, FontFamily::Proportional);
        style.override_font_id = Some(font_id);

        // Set the base Style
        style.visuals = style::Visuals::dark();

        // Tooltips and Popup Windows:
        style.visuals.window_fill = Color32::DARK_GRAY;
        style.visuals.window_stroke = Stroke::default();
        style.visuals.popup_shadow = Shadow {offset: Vec2::new(0.0, 0.0), blur: 6.0, spread: 0.0, color: Color32::from_black_alpha(32)};
        style.visuals.override_text_color = Some(Color32::WHITE);
        
        style.visuals.selection.stroke = Stroke::new(2.0, Color32::WHITE);
        ctx.set_style(style);

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
    }

    fn midi_channel_ok(channel: u8) -> bool {
        channel <= 11
    }

    fn map_midi_note(note: u8, channel: u8) -> Option<GeneralMidiDrums> {
        match channel {
            1  => Some(GeneralMidiDrums::BassDrum),
            2  => Some(GeneralMidiDrums::AcousticSnare),
            3  => Some(GeneralMidiDrums::SideStick),
            4  => Some(GeneralMidiDrums::HandClap),
            5  => Some(GeneralMidiDrums::LowFloorTom),
            6  => Some(GeneralMidiDrums::LowTom),
            7  => Some(GeneralMidiDrums::HighMidTom),
            8  => Some(GeneralMidiDrums::ClosedHihat),
            10  => Some(GeneralMidiDrums::OpenHihat),
            11 => Some(GeneralMidiDrums::CrashCymbal),
            _ => GeneralMidiDrums::try_from(note).ok()
        }        
    }
    fn map_midi_chromatic(note: u8, channel: u8) -> Option<f32> { // Returns RELATIVE note in semitones
        match channel {
            1  => Some(note as f32 - GeneralMidiDrums::BassDrum as u8 as f32),
            2  => Some(note as f32 - GeneralMidiDrums::AcousticSnare as u8 as f32),
            3  => Some(note as f32 - GeneralMidiDrums::SideStick as u8 as f32),
            4  => Some(note as f32 - GeneralMidiDrums::HandClap as u8 as f32),
            5  => Some(note as f32 - GeneralMidiDrums::LowFloorTom as u8 as f32),
            6  => Some(note as f32 - GeneralMidiDrums::LowTom as u8 as f32),
            7  => Some(note as f32 - GeneralMidiDrums::HighMidTom as u8 as f32),
            8  => Some(note as f32 - GeneralMidiDrums::ClosedHihat as u8 as f32),
            10  => Some(note as f32 - GeneralMidiDrums::OpenHihat as u8 as f32),
            11 => Some(note as f32 - GeneralMidiDrums::CrashCymbal as u8 as f32),
            _ => None
        }        
    }

    fn set_internal_params(&mut self) {
        // Gain Adjustments:
        // self.dsp_kick.borrow_mut().set_param_by_name("GainAdjustment", 0.0);
    }
}


impl Plugin for OneTrickSimian2 {
    const NAME: &'static str = "OneTrick SIMIAN2";
    const VENDOR: &'static str = "Punk Labs";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "contact@punklabs.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        #[cfg(feature = "multi_out")]
        AudioIOLayout {
            // Multi-Out Stereo
            main_input_channels: NonZeroU32::new(0),
            main_output_channels: NonZeroU32::new(NUM_CHANNELS),
            aux_input_ports: &[],
            // Two to five of these ports will be used at a time
            aux_output_ports: &[new_nonzero_u32(NUM_CHANNELS); AUX_BUS_COUNT],

            names: PortNames {
                layout: Some("Multi-Out Stereo"),

                main_input: None,
                main_output: Some("Output"),
                aux_inputs: &[],
                aux_outputs: &[
                    "Kick",
                    "Snare",
                    "Rimshot",
                    "Clap",
                    "Low Tom",
                    "Mid Tom",
                    "Hi Tom",
                    "Hihat Closed",
                    "Hihat Open",
                    "Cymbal",
                ],
            },
        },
        AudioIOLayout {
            // Stereo
            main_input_channels: NonZeroU32::new(0),
            main_output_channels: NonZeroU32::new(NUM_CHANNELS),
            names: PortNames {
                layout: Some("Stereo"),

                main_input: None,
                main_output: Some("Output"),
                aux_inputs: &[],
                aux_outputs: &[],
            },
            ..AudioIOLayout::const_default()
        },
    ];

    const MIDI_INPUT: MidiConfig = MidiConfig::Basic; //Basic or MidiCCs
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = false; // Too bad this doesn't split on Midi NoteOn events

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        // Resize buffers and perform other potentially expensive initialization operations here.
        // The `reset()` function is always called right after this function. You can remove this
        // function if you do not need it.

        let sample_rate = buffer_config.sample_rate as usize;

        //self.processor.initialize(48000, sample_rate, 2, MAX_BLOCK_SIZE);

        self.accum_buffer.resize(3, MAX_BLOCK_SIZE);

        //nih_log!("Sample Rate: {}", sample_rate);
        self.sample_rate = sample_rate;
        const SILENCE_MS: u32 = 100;
        for_each_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
            dsp.initialize(sample_rate)
                .resize_buffer(MAX_BLOCK_SIZE)
                .track_silence(SILENCE_MS);
        });

        self.set_internal_params();

        true
    }

    fn reset(&mut self) {
        // Reset buffers and envelopes here. This can be called from the audio thread and may not
        // allocate. You can remove this function if you do not need it.
        for_each_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
            dsp.reset();
        });
        
        self.set_internal_params();
    }

    fn process(
        &mut self,
        buffer: &mut Buffer,
        aux: &mut AuxiliaryBuffers,
        context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        let _is_standalone = matches!(context.plugin_api(), PluginApi::Standalone);

        ProcessEx::process_split_notes(
            buffer,
            aux,
            MAX_BLOCK_SIZE,
            context,
            |event| {
                // MIDI
                match event {
                    NoteEvent::MidiPitchBend {
                        timing: _,
                        channel,
                        value,
                    } => {
                        if !Self::midi_channel_ok(channel) {
                            return;
                        }
                        // Pitch Shifting through synthesis:
                        // Artifact-free and Latency-free
                        for_each_voice_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
                            dsp.pitch_wheel(value);
                        });
                    }
                    NoteEvent::MidiCC {
                        timing: _,
                        channel,
                        cc,
                        value,
                    } => {
                        if !Self::midi_channel_ok(channel) {
                            return;
                        }
                        if cc == 1 {
                            for_each_voice_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
                                dsp.mod_wheel(value);
                            });
                        }
                    }
                    NoteEvent::NoteOn {
                        timing: _,
                        voice_id: _,
                        channel,
                        note,
                        velocity,
                    } => {
                        if !Self::midi_channel_ok(channel) {
                            return;
                        }
                        if let Some(midi_drum_note) = Self::map_midi_note(note, channel) {
                            let chromatic = Self::map_midi_chromatic(note, channel);
                            let velocity = if let Some(p) =
                                self.params.param_float_at(self.param_global_sensitivity)
                            {
                                let sensitivity = p.value() * 0.01;
                                velocity * sensitivity + (1.0 - sensitivity) * (80.0 / 127.0)
                            } else {
                                velocity
                            };

                            match midi_drum_note {
                                GeneralMidiDrums::BassDrum | GeneralMidiDrums::AcousticBassDrum => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_kick.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::AcousticSnare
                                | GeneralMidiDrums::ElectricSnare => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_snare.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::SideStick => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_rimshot.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::HandClap => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_clap.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::LowFloorTom | GeneralMidiDrums::HighFloorTom => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_lowtom.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::LowTom | GeneralMidiDrums::LowMidTom => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_midtom.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::HighMidTom | GeneralMidiDrums::HighTom => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_hitom.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::ClosedHihat => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_hihat_closed.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                    self.dsp_hihat_open.borrow_mut().choke();
                                }
                                GeneralMidiDrums::PedalHihat => {
                                    self.dsp_hihat_closed.borrow_mut().choke();
                                    self.dsp_hihat_open.borrow_mut().choke();
                                }
                                GeneralMidiDrums::OpenHihat => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_hihat_open.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                GeneralMidiDrums::CrashCymbal => {
                                    self.dsp_output.borrow_mut().wake_up();
                                    self.dsp_cymbal.borrow_mut().note_on(chromatic.unwrap_or(0.0), velocity);
                                }
                                |
                                GeneralMidiDrums::RideCymbal => {
                                    self.dsp_cymbal.borrow_mut().choke();
                                }
                                _ => {}
                            }
                        }
                    }
                    _ => (),
                }
            },
            #[allow(unused_variables)]
            &mut |buffer: &mut Buffer, aux: &mut AuxiliaryBuffers, block_start, block_end| // AUDIO
            {
                let frames = block_end - block_start;

                // Attempt to process audio in blocks split by events:
                let output = &mut buffer.slice2ch_range_mut(block_start, block_end);
                for channel in output.iter_mut() {
                    channel.fill(0.0);
                }

                self.accum_buffer.buffer.clear_frames(frames);

                let pitch_shift = if let Some(p) =
                    self.params.param_float_at(self.param_global_transpose)
                {
                    p.value()
                } else {
                    0.0
                };

                for_each_voice_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
                    // Compute Voice:
                    if dsp.is_active() {
                        dsp.transpose(pitch_shift);
                        dsp.compute(frames, &[]);
                    } else {
                        dsp.skip_compute();
                    }
                    // Accumulate Voice:
                    dsp.add_to_buffer(&mut self.accum_buffer.buffer);
                });

                // Process Main Output:
                {
                    let mut dsp = self.dsp_output.borrow_mut();
                    if dsp.is_active() {
                        dsp.compute_to(frames, self.accum_buffer.buffer.as_slice_actually_immutable(), Some(output));
                    } else {
                        dsp.skip_compute();
                    }
                }

                // Handle Aux Output:
                #[cfg(feature = "multi_out") ]
                {
                    // NIH Plug will zero out Aux buffers if it gets "weird data" from host...
                    // Ardour: Uses only the first Aux output..
                    let aux_supported: bool = !aux.outputs.is_empty()
                        && !aux.outputs[0].is_empty()
                        && !aux.outputs[0].as_slice()[0].is_empty();
                    if aux_supported {
                        let mut aux_outs = aux.outputs.iter_mut();
                        for_each_voice_dsp!(self, &mut |dsp: &mut dyn OneTrickDSPGeneral| {
                            if let Some(aux_out) = aux_outs.next() {
                                if !aux_out.is_empty() && !aux_out.as_slice()[0].is_empty() {
                                    let mut slice = aux_out.slice2ch_range_mut(block_start, block_end);
                                    dsp.write_to_slice(&mut slice);
                                }
                            }
                        });
                    }
                }
            },
        );

        ProcessStatus::Normal
    }

    #[cfg(feature = "egui")]
    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        //async_executor.execute_gui((||{nih_log!("Hello from the GUI")})());
        //async_executor.execute_background((||{nih_log!("Hello from a Background Thread")})());

        let params = self.params.clone();
        let mut preset_manager = PresetManager::new("OneTrick SIMIAN2")
            .with_active_field(params.field_string("selected_preset"));
        let default_preset = Preset::from_param_defaults("Basic", &params);
        preset_manager.add_factory(default_preset);
        preset_manager.add_factory_string("Alpacalypse", include_str!("assets/presets/Alpacalypse.preset"));
        preset_manager.add_factory_string("Aquatic Traffic", include_str!("assets/presets/Aquatic Traffic.preset"));
        preset_manager.add_factory_string("Bar Fight", include_str!("assets/presets/Bar Fight.preset"));
        //preset_manager.add_factory_string("Basic", include_str!("assets/presets/Basic.preset"));
        preset_manager.add_factory_string("Boomsticks", include_str!("assets/presets/Boomsticks.preset"));
        preset_manager.add_factory_string("Bubble Factory", include_str!("assets/presets/Bubble Factory.preset"));
        preset_manager.add_factory_string("Clique Beetle", include_str!("assets/presets/Clique Beetle.preset"));
        preset_manager.add_factory_string("Discopsychology", include_str!("assets/presets/Discopsychology.preset"));
        preset_manager.add_factory_string("Disco Sol", include_str!("assets/presets/Disco Sol.preset"));
        preset_manager.add_factory_string("Duh Duh Duh", include_str!("assets/presets/Duh Duh Duh.preset"));
        preset_manager.add_factory_string("Dustbin", include_str!("assets/presets/Dustbin.preset"));
        preset_manager.add_factory_string("Eticroquette", include_str!("assets/presets/Eticroquette.preset"));
        preset_manager.add_factory_string("Factorial Fiction", include_str!("assets/presets/Factorial Fiction.preset"));
        preset_manager.add_factory_string("Fuchsia Sunrise", include_str!("assets/presets/Fuchsia Sunrise.preset"));
        preset_manager.add_factory_string("Fuchsia Sunset", include_str!("assets/presets/Fuchsia Sunset.preset"));
        preset_manager.add_factory_string("Hootenanny", include_str!("assets/presets/Hootenanny.preset"));
        preset_manager.add_factory_string("Italo Disco", include_str!("assets/presets/Italo Disco.preset"));
        preset_manager.add_factory_string("Life Support", include_str!("assets/presets/Life Support.preset"));
        preset_manager.add_factory_string("Macrochip", include_str!("assets/presets/Macrochip.preset"));
        preset_manager.add_factory_string("No Problem", include_str!("assets/presets/No Problem.preset"));
        preset_manager.add_factory_string("Oh Shoot", include_str!("assets/presets/Oh Shoot.preset"));
        preset_manager.add_factory_string("Passive Aggressive", include_str!("assets/presets/Passive Aggressive.preset"));
        preset_manager.add_factory_string("Power Down", include_str!("assets/presets/Power Down.preset"));
        preset_manager.add_factory_string("Power Up", include_str!("assets/presets/Power Up.preset"));
        preset_manager.add_factory_string("Raptor Hatchling", include_str!("assets/presets/Raptor Hatchling.preset"));
        preset_manager.add_factory_string("Sandy Fuego", include_str!("assets/presets/Sandy Fuego.preset"));
        preset_manager.add_factory_string("Sasquelch", include_str!("assets/presets/Sasquelch.preset"));
        preset_manager.add_factory_string("Sequence", include_str!("assets/presets/Sequence.preset"));
        preset_manager.add_factory_string("Shhhhh", include_str!("assets/presets/Shhhhh.preset"));
        preset_manager.add_factory_string("Suburbanight", include_str!("assets/presets/Suburbanight.preset"));
        preset_manager.add_factory_string("Thrashcan", include_str!("assets/presets/Thrashcan.preset"));
        preset_manager.add_factory_string("Tommy Tom Tom", include_str!("assets/presets/Tommy Tom Tom.preset"));
        preset_manager.add_factory_string("Undeadhead", include_str!("assets/presets/Undeadhead.preset"));
        preset_manager.add_factory_string("Whatever", include_str!("assets/presets/Whatever.preset"));
        preset_manager.add_factory_string("Preset Init", include_str!("assets/presets/Preset Init.preset"));

        preset_manager.refresh();

        let tab_field: Arc<FieldI32> = params.field_int("tab_index").unwrap();


        let palette = Palette::new(8)
            .shades(6)
            .white_level(1.0)
            .black_level(0.15)
            .saturation(0.82) // 0.75
            .shift(0.49)
            .alt_hue_step(4.533)
            .dark_shift(0.0) // 0.1
            .dark_desaturation(0.2);
        //let palette_alt = palette.alternate(1.5);

        let main_gray = palette.white().brightness(0.2).to_color32();

        let mut knob_style = ParamKnobStyle {
            radius: 35.0,
            indicator_style: ParamKnobIndicatorStyle::Continuous,
            ..Default::default()
        };
        let column_width = knob_style.required_width();


        let mut heading_style = LabelStyle::default_heading();
        heading_style.width = Some(column_width);
        heading_style.color = palette.black().into();
        heading_style.bg_color = Color32::TRANSPARENT; //Color32::from_gray(128);
        heading_style.shadow = None;
        let mut subheading_style = heading_style.clone();
        subheading_style.color = palette.white().into();
        subheading_style.bg_color = Color32::TRANSPARENT;
        subheading_style.shadow = None;
        


        let separator_style = SeparatorStyle {
            width: 2.0,
            color: palette.white().into(),
            ..Default::default()
        };
        knob_style.knob_color = palette.grey().into();
        knob_style.label_style.bg_color = palette.black().alpha(0.0).into(); //palette.black().into();
        knob_style.label_style.color = palette.white().into();
        knob_style.label_style_hover.bg_color = palette.black().into();
        knob_style.label_style_hover.color = palette.color(0).into();
        knob_style.bg_color = palette.black().into();
        knob_style.indicator_fill_color = palette.grey().into();

        let mut knob_style_1 = knob_style.clone();
        knob_style_1.indicator_fill_color = palette.color_alt(0, 1).into();
        let mut knob_style_2 = knob_style.clone();
        knob_style_2.indicator_fill_color = palette.color(0).into();
        let mut knob_style_3 = knob_style.clone();
        knob_style_3.indicator_fill_color = palette.color(1).into();
        let mut knob_style_4 = knob_style.clone();
        knob_style_4.indicator_fill_color = palette.color(2).into();
        let mut knob_style_5 = knob_style.clone();
        knob_style_5.indicator_fill_color = palette.color(4).into();
        let mut knob_style_6 = knob_style.clone();
        knob_style_6.indicator_fill_color = palette.color(5).into();
        let mut knob_style_7 = knob_style.clone();
        knob_style_7.indicator_fill_color = palette.color(11).into();

        let mut slider_style = ParamSliderStyle {
            length: 70.0,
            ..Default::default()
        };
        slider_style.knob_color = knob_style.knob_color;
        slider_style.label_width = 70.0;
        slider_style.label_style.bg_color = knob_style.label_style.bg_color;
        slider_style.label_style.color = knob_style.label_style.color;
        slider_style.label_style_hover.bg_color = knob_style.label_style_hover.bg_color;
        slider_style.label_style_hover.color = knob_style.label_style.color;
        slider_style.bg_color = palette.black().into();
        slider_style.fill_color = Color32::LIGHT_GREEN;

        let mut slider_style_1 = slider_style.clone();
        slider_style_1.fill_color = knob_style_1.indicator_fill_color;
        let mut slider_style_2 = slider_style.clone();
        slider_style_2.fill_color = knob_style_2.indicator_fill_color;
        let mut slider_style_3 = slider_style.clone();
        slider_style_3.fill_color = knob_style_3.indicator_fill_color;
        let mut slider_style_4 = slider_style.clone();
        slider_style_4.fill_color = knob_style_4.indicator_fill_color;
        #[allow(clippy::redundant_clone)]
        let mut slider_style_5 = slider_style.clone();
        slider_style_5.fill_color = knob_style_5.indicator_fill_color;

        let show_credits = Arc::new(AtomicBool::new(false));
        let credits_anim_id = Id::new("credits_anim");
        let kick_anim_id = Id::new("kick_anim");
        let snare_anim_id = Id::new("snare_anim");
        let rimshot_anim_id = Id::new("rimshot_anim");
        let clap_anim_id = Id::new("clap_anim");
        let lowtom_anim_id = Id::new("lowtom_anim");
        let midtom_anim_id = Id::new("midtom_anim");
        let hitom_anim_id = Id::new("hitom_anim");
        let hihat_closed_anim_id = Id::new("hihat_closed_anim");
        let hihat_open_anim_id = Id::new("hihat_open_anim");
        let cymbal_anim_id = Id::new("cymbal_anim");

        let kick_indicator = self.kick_indicator.clone();
        let snare_indicator = self.snare_indicator.clone();
        let rimshot_indicator = self.rimshot_indicator.clone();
        let clap_indicator = self.clap_indicator.clone();
        let lowtom_indicator = self.lowtom_indicator.clone();
        let midtom_indicator = self.midtom_indicator.clone();
        let hitom_indicator = self.hitom_indicator.clone();
        let hihat_closed_indicator = self.hihat_closed_indicator.clone();
        let hihat_open_indicator = self.hihat_open_indicator.clone();
        let cymbal_indicator = self.cymbal_indicator.clone();

        let vu_meter_left = self.vu_meter_left.clone();
        let vu_meter_right = self.vu_meter_right.clone();

        create_egui_editor(
            self.params.editor_state.clone(),
            preset_manager,
            move |ctx, _| {
                // DPI:
                // egui_baseview needs to be updated to support DPI.
                // We can adjust egui's rendering, but not egui_baseview's resolution.
                //ctx.set_pixels_per_point(2.0);

                //nih_log!("Editor::Open()");
                Self::setup_egui_style(ctx);
            },
            move |ctx, setter, state| {
                let preset_manager = state;
                //ctx.set_debug_on_hover(true);

                //update()
                //let window_rect = ctx.available_rect();

                let zoom_amount = 0.2;
                let zoom_time = 0.150;
                let indicator_raw = kick_indicator.swap(false, Ordering::Relaxed);
                let kick_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        kick_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = snare_indicator.swap(false, Ordering::Relaxed);
                let snare_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        snare_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = rimshot_indicator.swap(false, Ordering::Relaxed);
                let rimshot_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        rimshot_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = clap_indicator.swap(false, Ordering::Relaxed);
                let clap_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        clap_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
            
                let indicator_raw = lowtom_indicator.swap(false, Ordering::Relaxed);
                let lowtom_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        lowtom_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = midtom_indicator.swap(false, Ordering::Relaxed);
                let midtom_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        midtom_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = hitom_indicator.swap(false, Ordering::Relaxed);
                let hitom_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        hitom_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = hihat_closed_indicator.swap(false, Ordering::Relaxed);
                let hihat_closed_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        hihat_closed_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = hihat_open_indicator.swap(false, Ordering::Relaxed);
                let hihat_open_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        hihat_open_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;
                let indicator_raw = cymbal_indicator.swap(false, Ordering::Relaxed);
                let cymbal_indicator_zoom = 1.0
                    + ctx.animate_bool_with_time(
                        cymbal_anim_id,
                        indicator_raw,
                        if indicator_raw { 0.0 } else { zoom_time },
                    ) * zoom_amount;

                let vu_meter_left_raw = vu_meter_left.load(Ordering::Relaxed) as f32 / 1000.0;
                let vu_meter_right_raw = vu_meter_right.load(Ordering::Relaxed) as f32 / 1000.0;

                let show_credits_amount =
                    ctx.animate_bool_with_time(credits_anim_id, show_credits.load(Ordering::Relaxed), 0.300);

                if show_credits_amount > 0.0 {
                    Area::new("credits_area".into())
                        .fixed_pos(Pos2::new(0.0, 0.0))
                        .order(Order::Foreground)
                        .show(ctx, |ui| {
                            Frame::none()
                                .outer_margin(Margin::same(0.0))
                                .inner_margin(Margin::same(10.0))
                                .fill(Color32::from_black_alpha(
                                    (200.0 * show_credits_amount) as u8,
                                ))
                                .show(ui, |ui| {
                                    let available_size =
                                        Vec2::new(ui.available_width(), ui.available_height());
                                    //let animated_offset = Vec2::new(0.0, available_size.y * (1.0-show_credits_amount));
                                    ui.painter().text(
                                        (available_size * 0.5).to_pos2() + Vec2::new(0.0, 15.0),
                                        Align2::CENTER_CENTER,
                                        format!(
                                            include_str!("CREDITS"),
                                            VERSION = env!("CARGO_PKG_VERSION")
                                        ),
                                        FontId::new(20.0, FontFamily::Name("Title".into())),
                                        Color32::from_white_alpha(
                                            (255.0 * show_credits_amount) as u8,
                                        ),
                                    );

                                    if ui
                                        .allocate_response(available_size, Sense::click())
                                        .clicked()
                                    {
                                        show_credits.store(false, Ordering::Relaxed);
                                    }
                                });
                        });
                }
                CentralPanel::default()
                    .frame(
                        Frame::none()
                            .outer_margin(Margin::same(0.0))
                            .inner_margin(Margin::same(0.0))
                            .fill(main_gray),
                    )
                    .show(ctx, |_ui| {
                        //ctx.set_debug_on_hover(true);

                        let header_height = 70.0;

                        // Separate foreground for interaction above other areas
                        Area::new("header_fg".into()) // Prevents layout being affected by Logo
                        .fixed_pos(Pos2::new(0.0, 0.0))
                        .order(Order::Foreground)
                        .show(ctx, |ui| {
                            let response = ui.allocate_rect(
                                Rect::from_min_size(Pos2::new(0.0, 0.0),
                                Vec2::new(240.0, 70.0)),
                                Sense::click());
                            if response.clicked() {
                                show_credits.store(true, Ordering::Relaxed);
                            }
                            ui.painter().one_trick_logo(
                                "SIMIAN",
                                Pos2::new(15.0, 35.0),
                                52.0,
                                palette.white().brightness(0.66).into(),
                                if response.hovered() {palette.color(5).into()} else {palette.white().into()},
                            );
                        });

                        // Separate background to go below the tabs as they expand
                        Area::new("header_bg".into()) // Prevents layout being affected by Logo
                        .fixed_pos(Pos2::new(0.0, 0.0))
                        .order(Order::Background)
                        .show(ctx, |ui| {
                            ui.painter().rect_filled(
                                Rect::from_min_max(Pos2::new(435.0, 0.0), Pos2::new(1400.0, header_height)), //415.0
                                Rounding::same(0.0),
                                palette.color(8).shade(1).to_color32(),
                                // palette.color(5).shade(1).hue(-0.15).to_color32(),
                            );
    
                            ui.painter().rect_filled(
                                Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(435.0, header_height)),
                                Rounding::same(0.0),
                                palette.black().to_color32());
                            ui.painter().parallelograms(
                                Rect::from_min_size(Pos2::new(325.0, 0.0), Vec2::new(220.0, header_height)),
                                1.0,
                                &[
                                    palette.color(5).shade(1).hue(0.05).to_color32(),
                                    palette.color(5).shade(1).hue(0.00).to_color32(),
                                    palette.color(5).shade(1).hue(-0.05).to_color32(),
                                    palette.color(5).shade(1).hue(-0.10).to_color32(),
                                ],
                            );

                        });

                        // Main area for the rest of the UI
                        Area::new("main_area".into()) // Prevents layout being affected by Logo
                        .fixed_pos(Pos2::new(0.0, 0.0))
                        .order(Order::Background)
                        .show(ctx, |ui| {

                            ui.set_width(ui.available_width());

                            let panel_rounding = Rounding::same(15.0);
                            let panel_shadow = Shadow {offset: Vec2::new(0.0, 0.0), blur: 12.0, spread: 0.0, color: Color32::from_black_alpha(12)};

                            // ============================================
                            // ============== MAIN 2 COLUMNS ==============
                            // ============================================
                            ui.horizontal(|ui| {
                                let default_item_spacing = ui.spacing().item_spacing;
                                ui.spacing_mut().item_spacing.x = 0.0;
                                // ==========================================
                                // ============== VOICES PANEL ==============
                                // ==========================================
                                Frame::none()
                                    .outer_margin(Margin::same(0.0))
                                    .inner_margin(Margin::same(0.0))
                                    .fill(Color32::TRANSPARENT)
                                    .show(ui, |ui| {
                                    ui.vertical(|ui| { // Voices
                                        ui.set_width(820.0);
                                        ui.set_height(660.0);
                                        ui.add_space(header_height);
                                        let tabs_style = IconTabsStyle{
                                            panel_rounding: 0.0.into(),
                                            panel_color: palette.white().brightness(0.9).into(),
                                            //bg_color_active: palette.color(3).into(),
                                            tab_color_active: main_gray,

                                            color: palette.black().into(),
                                            color_active: palette.white().into(),
                                            tab_rounding: Rounding{
                                                nw: 5.0,
                                                ne: 5.0,
                                                sw: 0.0,
                                                se: 0.0,
                                            },

                                            ..Default::default()
                                        };
                                        let regular_stroke = ui.style().visuals.selection.stroke;
                                        ui.style_mut().visuals.selection.stroke.color = main_gray;
                                        let mut tab_index: u32 = tab_field.get() as u32;
                                        ui.add(IconTabs::new(
                                            &mut tab_index,
                                            tabs_style,
                                            vec![
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::KickDrum)
                                                    .with_zoom(kick_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::SnareDrum)
                                                    .with_zoom(snare_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::Claves)
                                                    .with_zoom(rimshot_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::Clap)
                                                    .with_zoom(clap_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::TomDrumLarge)
                                                    .with_zoom(lowtom_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::TomDrumMedium)
                                                    .with_zoom(midtom_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::TomDrumSmall)
                                                    .with_zoom(hitom_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::Cymbal)
                                                    .with_zoom(hihat_closed_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::Hihat)
                                                    .with_zoom(hihat_open_indicator_zoom),
                                                IconTabStyle::default()
                                                    .with_dingbat(Dingbat::Cymbal)
                                                    .with_zoom(cymbal_indicator_zoom),
                                            ],
                                        ));
                                        tab_field.set(tab_index as i32);

                                        ui.style_mut().visuals.selection.stroke = regular_stroke;
                                        ui.add_space(15.0);
                                        let voices = [
                                            "Kick",
                                            "Snare",
                                            "Rimshot",
                                            "Clap",
                                            "Low Tom",
                                            "Mid Tom",
                                            "High Tom",
                                            "Hihat Closed",
                                            "Hihat Open",
                                            "Cymbal",
                                        ];
                                        let voice_name = voices[tab_index as usize];
                                        let center_offset_x = 39.0;
                                        let center_offset_y = 55.0;
                                        let spacing_y = 15.0;
                                        let justify_spacing_x = 10.0;
                                        let bend_group_offset = Vec2::new(75.0, -5.0);
                                        let bend_group_size = Vec2::new(80.0, 205.0);
                                        let bend_group_color = palette.white().alpha(0.025).to_color32();
                                        ui.horizontal(|ui| {
                                            ui.add_space(10.0);
                                            ui.spacing_mut().item_spacing = vec2(justify_spacing_x, spacing_y);
                                            ui.set_height(232.0);
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(Label::new("Tone").with_style(&subheading_style));
                                                });
                                                ui.painter().rect_filled( // Bend Group
                                                    Rect::from_min_size(ui.next_widget_position()+bend_group_offset, bend_group_size),
                                                    Rounding::same(10.0),
                                                    bend_group_color,
                                                );
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Osc Pitch", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Pitch")
                                                        .with_style(&knob_style_1),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Osc Env Bend", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Bend")
                                                        .with_style(&knob_style_1),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Osc Waveform", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Waveform")
                                                        .with_style(&knob_style_1),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Osc Env Bend Dynamics", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Velocity")
                                                        .with_style(&knob_style_1)
                                                        .with_indicator_style(ParamKnobIndicatorStyle::Dots),
                                                    );
                                                });
                                            });
                                            ui.add(Separator::new().with_style(&separator_style));
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(Label::new("Filter").with_style(&subheading_style));
                                                });
                                                ui.painter().rect_filled( // Bend Group
                                                    Rect::from_min_size(ui.next_widget_position()+bend_group_offset, bend_group_size),
                                                    Rounding::same(10.0),
                                                    bend_group_color,
                                                );
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Lowpass Freq", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Cutoff")
                                                        .with_style(&knob_style_2),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Lowpass Env Bend", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Bend")
                                                        .with_style(&knob_style_2),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Lowpass Q", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Resonance")
                                                        .with_style(&knob_style_2),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Lowpass Env Bend Dynamics", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Velocity")
                                                        .with_style(&knob_style_2)
                                                        .with_indicator_style(ParamKnobIndicatorStyle::Dots),
                                                    );
                                                });
                                            });
                                            ui.add(Separator::new().with_style(&separator_style));
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add(Label::new("Envelope").with_style(&subheading_style));
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Env Decay", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Decay")
                                                        .with_style(&knob_style_3),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Env Punch", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Punch")
                                                        .with_style(&knob_style_3),
                                                    );
                                                });
                                            });
                                            ui.add(Separator::new().with_style(&separator_style));
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(Label::new("Mix").with_style(&subheading_style));
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Pan", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Pan")
                                                        .with_style(&knob_style_4),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Send Reverb", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Room")
                                                        .with_style(&knob_style_4),
                                                    );
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Fade Tone Noise", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Tone Noise")
                                                        .with_style(&knob_style_4),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float(&format!("{} Fade Click", voice_name)),
                                                            setter,
                                                        )
                                                        .with_label("Transient")
                                                        .with_style(&knob_style_1),
                                                    );
                                                });
                                            });

                                            ui.add(Separator::new().with_style(&separator_style));

                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(Label::new("Amp").with_style(&subheading_style));
                                                });
                                                ui.painter().rect_filled( // Bend Group
                                                    Rect::from_min_size(ui.next_widget_position()+bend_group_offset, bend_group_size),
                                                    Rounding::same(10.0),
                                                    bend_group_color,
                                                );
                                                ui.horizontal(|ui| {
                                                    ui.vertical(|ui| {
                                                        ui.add_space(center_offset_y);
                                                        ui.add(
                                                            ParamKnob::for_param(
                                                                params.param_float(&format!("{} Saturation", voice_name)),
                                                                setter,
                                                            )
                                                            .with_label("Saturation")
                                                            .with_style(&knob_style_5),
                                                        );
                                                    });
                                                    ui.vertical(|ui| {
                                                        ui.add(
                                                            ParamKnob::for_param(
                                                                params.param_float(&format!("{} Gain", voice_name)),
                                                                setter,
                                                            )
                                                            .with_label("Gain")
                                                            .with_style(&knob_style_5),
                                                        );
                                                        ui.add(
                                                            ParamKnob::for_param(
                                                                params.param_float(&format!("{} Gain Dynamics", voice_name)),
                                                                setter,
                                                            )
                                                            .with_label("Velocity")
                                                            .with_style(&knob_style_5)
                                                            .with_indicator_style(ParamKnobIndicatorStyle::Dots),
                                                        );
                                                    });
                                                });
                                            });
                                        });
                                        ui.add_space(10.0);
                                        ui.painter().rect_filled(
                                            Rect::from_min_size(Pos2::new(0.0, ui.next_widget_position().y), Vec2::new(1400.0, 200.0)), //415.0
                                            Rounding::same(0.0),
                                            palette.color(4).brightness(0.8).to_color32(),
                                        );
                                        ui.add_space(10.0);
                                        ui.horizontal(|ui| {
                                            ui.spacing_mut().item_spacing = vec2(justify_spacing_x, spacing_y);
                                            ui.add_space(90.0);
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add(Label::new("Input").with_style(&subheading_style));
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Global Sensitivity"),
                                                            setter,
                                                        )
                                                        .with_label("Velocity")
                                                        .with_style(&knob_style_1)
                                                        .with_indicator_style(ParamKnobIndicatorStyle::Dots),
                                                    );
                                                });
                                            });
                                            ui.add(Separator::new().with_style(&separator_style));
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x);
                                                    ui.add(Label::new("Room Reverb").with_style(&subheading_style));
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Global Reverb Size"),
                                                            setter,
                                                        )
                                                        .with_label("Size")
                                                        .with_style(&knob_style_4),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Global Reverb Gate"),
                                                            setter,
                                                        )
                                                        .with_label("Gate")
                                                        .with_style(&knob_style_4),
                                                    );
                                                });
                                            });
                                            ui.add(Separator::new().with_style(&separator_style));
                                            ui.vertical(|ui| {
                                                ui.horizontal(|ui| {
                                                    ui.add_space(center_offset_x * 2.0);
                                                    ui.add(Label::new("Output").with_style(&subheading_style));
                                                });
                                                ui.horizontal(|ui| {
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Global Drive"),
                                                            setter,
                                                        )
                                                        .with_label("Drive")
                                                        .with_style(&knob_style_6),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Global Saturation"),
                                                            setter,
                                                        )
                                                        .with_label("Saturation")
                                                        .with_style(&knob_style_7),
                                                    );
                                                    ui.add(
                                                        ParamKnob::for_param(
                                                            params.param_float("Global Gain"),
                                                            setter,
                                                        )
                                                        .with_label("Volume")
                                                        .with_style(&knob_style_6),
                                                    );
                                                });
                                            });
                                    
                                        });
                                        // ===============================================
                                        // ===============================================
                                        // ===============================================
                                        // ===============================================
                                        // ===============================================
                                    });
                                });
                                
                                // ============================================
                                // ============== PRESETS PANELS ==============
                                // ============================================
                                Frame::none() // Preset Panel Wrapper
                                    .show(ui, |ui| {

                                    let frame_outer_margin = Margin {
                                        top:5.0,
                                        ..Default::default()
                                    };
                                    let frame_inner_margin = Margin{left: 0.0, right: 0.0, top: 5.0, bottom: 15.0};
                                    let subframe_inner_margin = Margin::same(10.0);
                                                                    
                                    ui.set_max_width(300.0);
                                    ui.vertical_centered(|ui| {
                                        // ===========================================
                                        // ============== PRESETS PANEL ==============
                                        // ===========================================
                                        ui.add_space(35.0);
                                        Frame::none()
                                            .outer_margin(frame_outer_margin)
                                            .inner_margin(frame_inner_margin)
                                            //.fill(palette.color(6).shade(1).into())
                                            .fill(palette.white().brightness(0.9).into())
                                            .rounding(panel_rounding)
                                            .shadow(panel_shadow)
                                            .show(ui, |ui| {
                                            ui.spacing_mut().item_spacing = default_item_spacing;
                                            ui.vertical_centered(|ui| {
                                                ui.add(
                                                    Label::new("Presets")
                                                    .with_style(&heading_style)
                                                );
                                            });

                                            Frame::none()
                                            .outer_margin(Margin::same(0.0))
                                            .inner_margin(subframe_inner_margin)
                                            .fill(palette.white().brightness(0.4).into())
                                            .show(ui, |ui| {
                                                // ============================================
                                                // ============== PRESETS PANEL ==============
                                                // ============================================
                                                let list_style = PresetListStyle {
                                                    bg_color: None,//Some(palette.dark_layer(1).into()),
                                                    color: palette.white().into(),
                                                    icon_style: IconButtonStyle{
                                                        color: palette.white().into(),
                                                        bg_color: Color32::TRANSPARENT,
                                                        color_hover: palette.white().into(),
                                                        bg_color_hover: palette.black().into(),
                                                        ..Default::default()
                                                    },
                                                    label_style: LabelStyle{
                                                        bg_color: palette.black().alpha(0.25).into(),
                                                        bg_color_hover: Some(palette.black().alpha(0.5).into()),
                                                        color: palette.white().brightness(0.95).into(),
                                                        color_hover: Some(palette.white().into()),
                                                        ..Default::default()
                                                    },
                                                    highlight_color: palette.color(0).into(),
                                                    highlight_bg_color: palette.black().alpha(0.9).into(),
                                                    search_bg_color: Some(palette.black().alpha(0.5).into()),
                                                    search_color: Some(palette.white().into()),
                                                    popup_offset_normalized: vec2(-1.0, 0.0),
                                                    ..Default::default()
                                                };

                                                ui.vertical_centered(|ui| {
                                                    ui.set_width(ui.available_width());
                                                    ui.set_height(430.0);
                                                    PresetList::new()
                                                        .with_style(&list_style)
                                                        .show(ui, preset_manager, &params, setter);
                                                });
                                            });
                                        });
                                    });
                                });
                            });
                            ui.painter().vu_meter_stereo(
                                Rect::from_min_size(Pos2::new(622.0, 410.0), Vec2::new(40.0, 125.0)),
                                vu_meter_left_raw,
                                vu_meter_right_raw,
                                &VuMeterStyle {
                                    dot_count: 20,
                                    spacing: 2.0,
                                    padding: 5.0,
                                    bg_color: palette.black().into(),
                                    unlit_color: palette.white().brightness(0.4).into(),
                                    lit_color: palette.color(7).into(),
                                    warn_color: palette.color(5).into(),
                                    peak_color: palette.color(3).into(),
                                }
                            );
                            // ui.painter().dingbat(
                            //     Pos2::new(672.0, 480.0),
                            //     Align2::LEFT_CENTER,
                            //     Dingbat::Plugin1,
                            //     140.0,
                            //     palette.white().brightness(0.9).into(),
                            // );
                        });
                    });
            },
        )
    }
}

impl ClapPlugin for OneTrickSimian2 {
    const CLAP_ID: &'static str = "com.punklabs.onetrick.simian2";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("A Drum Machine");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::Instrument,  //Plugin Category
        ClapFeature::DrumMachine, // Plugin Sub-Category
        ClapFeature::Stereo,
    ]; // Audio Capabilities
       /*
       const CLAP_POLY_MODULATION_CONFIG: Option<PolyModulationConfig> = Some(PolyModulationConfig {
           // If the plugin's voice capacity changes at runtime (for instance, when switching to a
           // monophonic mode), then the plugin should inform the host in the `initialize()` function
           // as well as in the `process()` function if it changes at runtime using
           // `context.set_current_voice_capacity()`
           max_voice_capacity: NUM_VOICES,
           // This enables voice stacking in Bitwig.
           supports_overlapping_voices: true,
       });
       */
}

impl Vst3Plugin for OneTrickSimian2 {
    const VST3_CLASS_ID: [u8; 16] = *b"OneTrick-SIMIAN2";
    //const VST3_CATEGORIES: &'static str = "Instrument|Synth";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(OneTrickSimian2);
nih_export_vst3!(OneTrickSimian2);
