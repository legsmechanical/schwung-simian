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

//use std::f32::consts::{PI, TAU};
//use std::sync::Arc;

use lazy_static::lazy_static;
use nih_plug::params::{BoolParam, FloatParam, IntParam};
use nih_plug::prelude::{Param, ParamSetter};
use nih_plug_egui::egui::{
    /*epaint::{
        PathShape,
    },*/
    output::CursorIcon,
    Color32,
    Id,
    Pos2,
    Rect,
    Response,
    Rounding,
    Sense,
    //Shape,
    Stroke,
    Ui,
    Vec2,
    Widget,
    Event,
    EventFilter,
    Key,
};
//use parking_lot::Mutex;

use super::label::{Label, LabelStyle};
//use super::shape::ShapeEx;
//use super::icons::{Icon, IconDrawer};

const DRAG_SCALE: f32 = 0.0025;
const DRAG_FINE_SCALE: f32 = 0.0005;
const DRAG_SCALE_CYCLE: f32 = 0.05;
const DRAG_SCALE_TOGGLE: f32 = 25.0;

/// Helper functions for Param types to check if they can be cycled
pub trait ParamDetails {
    /// Returns true if this param can be cycled
    fn can_cycle(&self) -> bool;
}
impl ParamDetails for BoolParam {
    /// Returns true, because a BoolParam can be cycled
    fn can_cycle(&self) -> bool {
        true
    }
}
impl ParamDetails for FloatParam {
    /// Returns false, because a FloatParam can't be cycled
    fn can_cycle(&self) -> bool {
        false
    }
}
impl ParamDetails for IntParam {
    /// Returns true if this param can be cycled
    fn can_cycle(&self) -> bool {
        if let Some(step_count) = self.step_count() {
            step_count < 5
        } else {
            false
        }
    }
}

lazy_static! {
    static ref DRAG_NORMALIZED_START_VALUE_MEMORY_ID: Id = Id::new((file!(), 0));
    static ref DRAG_AMOUNT_MEMORY_ID: Id = Id::new((file!(), 1));
    static ref VALUE_ENTRY_MEMORY_ID: Id = Id::new((file!(), 2));
}

/// Orientation of a Slider indicator fill
#[derive(Clone)]
pub enum ParamSliderFillOrigin {
    Start,
    Center,
    End,
}

/// Orientation of a Slider
#[derive(Clone)]
pub enum ParamSliderOrientation {
    Horizontal,
    Vertical,
}

/// Style of a Slider
#[derive(Clone)]
pub struct ParamSliderStyle {
    /// Orientation of of this Slider
    pub orientation: ParamSliderOrientation,
    
    /// Total length of the Widget
    pub length: f32,
    
    /// Total width of the Widget
    pub width: f32,

    /// Should the background be centered to account for tick marks?
    pub center_bg: bool,

    /// Background color of the Widget
    pub bg_color: Color32,

    /// Knob width, or size if round
    pub knob_width: f32,

    /// Aspect ratio of the Knob when rectangular
    pub knob_aspect: f32,

    /// Amount to scale the Knob when hovering
    pub knob_hover_scale: f32,

    /// How round to make the Knob, as a normalized value
    pub knob_round_ratio: f32,

    /// Color of the Knob
    pub knob_color: Color32,

    /// Color of the Knob tick
    pub knob_tick_color: Color32,

    /// Color of the Knob's outline stroke
    pub knob_outline_color: Color32,

    /// Thickness of the Knob's outline stroke
    pub knob_outline_width: f32,

    /// Number of ticks to the side of the indicator
    pub tick_count: u32,

    /// Color of ticks to the side of the indicator
    pub tick_color: Color32,

    /// Width of ticks to the side of the indicator
    pub tick_width: f32,

    /// Length of ticks to the side of the indicator
    pub tick_length: f32,

    /// Alternating length of ticks to the side of the indicator
    pub tick_alt_length: f32,

    /// Margin around the Knob track
    pub track_margin: f32,

    /// Color of the Knob track
    pub track_color: Color32,

    /// Normalized margin for the indicator fill
    pub fill_margin: f32,

    /// Origin of the indicator fill
    pub fill_origin: ParamSliderFillOrigin,

    /// Color of the indicator fill
    pub fill_color: Color32,

    /// Color of the indicator fill when hovering
    pub fill_color_hover: Color32,

    /// Color of the indicator fill when modulating (CLAP)
    pub fill_modulation_color: Color32,

    /// Color of the indicator fill when modulating and hovering (CLAP)
    pub fill_modulation_color_hover: Color32,

    /// Width of the label
    /// TODO: This could be replaced with label_style.width
    pub label_width: f32,

    /// Style of the label
    pub label_style: LabelStyle,

    /// Style of the label when hovering
    pub label_style_hover: LabelStyle,

    /// Offset of the label relative to the bottom of the Widget
    pub label_offset: f32,

    /// Only display the value label, not the property name
    pub label_only_value: bool,
}
impl ParamSliderStyle {
    /// Returns the indicator fill color
    pub fn get_fill_color(&self, hovered: bool) -> Color32 {
        if hovered {
            self.fill_color_hover
        } else {
            self.fill_color
        }
    }

    /// Returns the indicator fill color when modulating (CLAP)
    pub fn get_fill_modulation_color(&self, hovered: bool) -> Color32 {
        if hovered {
            self.fill_modulation_color_hover
        } else {
            self.fill_modulation_color
        }
    }

    /// Returns the required margin for tick marks
    pub fn required_tick_margin(&self) -> f32 {
        if self.tick_count > 0 {
            self.tick_length + self.track_margin
        } else {
            0.0
        }
    }

    /// Returns the required width for the Widget
    pub fn required_width(&self) -> f32 {
        let bg_width = match self.orientation {
            ParamSliderOrientation::Horizontal => self.length,
            ParamSliderOrientation::Vertical => self.width,
        };

        self.label_width.max(bg_width)
    }

    /// Returns the required height for the Widget
    pub fn required_height(&self) -> f32 {
        match self.orientation {
            ParamSliderOrientation::Horizontal => self.width + self.required_tick_margin(),
            ParamSliderOrientation::Vertical => self.length,
        }
    }
}

impl Default for ParamSliderStyle {
    fn default() -> Self {
        Self {
            orientation: ParamSliderOrientation::Vertical,
            length: 80.0,
            width: 20.0,
            center_bg: true,
            bg_color: Color32::BLACK,

            knob_width: 15.0,
            knob_aspect: 1.0,
            knob_hover_scale: 1.4,
            knob_round_ratio: 1.0,
            knob_color: Color32::LIGHT_GRAY,
            knob_tick_color: Color32::DARK_GRAY,
            knob_outline_color: Color32::from_gray(64), //Color32::from_black_alpha(128),
            knob_outline_width: 1.5,

            tick_count: 7,
            tick_color: Color32::from_white_alpha(128),
            tick_width: 1.5,
            tick_length: 10.0,
            tick_alt_length: 5.0,

            track_margin: 4.0,
            track_color: Color32::DARK_GRAY,
            fill_margin: 1.0,
            fill_origin: ParamSliderFillOrigin::Start,
            fill_color: Color32::LIGHT_GRAY,
            fill_color_hover: Color32::WHITE,
            fill_modulation_color: Color32::YELLOW,
            fill_modulation_color_hover: Color32::GRAY,

            label_width: 80.0,
            label_style: LabelStyle::default(),
            label_style_hover: LabelStyle::default_hover(),
            label_offset: 2.0,
            label_only_value: false,
        }
    }
}

/// A Slider Widget that can modify a Plugin Parameter
pub struct ParamSlider<'a, P: Param + ParamDetails> {
    param: Option<&'a P>,
    setter: &'a ParamSetter<'a>,
    style: ParamSliderStyle,
    label: Option<String>,

    draw_value: bool,
    //animation_id: Id, // Not needed!
}

#[allow(unused)]
impl<'a, P: Param + ParamDetails> ParamSlider<'a, P> {
    /// Returns a new Slider Widget
    pub fn for_param(param: Option<&'a P>, setter: &'a ParamSetter<'a>) -> Self {
        Self {
            param,
            setter,
            style: ParamSliderStyle::default(),
            label: None,

            draw_value: true,
            //animation_id: Id::new((file!(), param.name(), "animation")), // Not needed
        }
    }

    /// Sets the style
    pub fn with_style(mut self, style: &ParamSliderStyle) -> Self {
        self.style = style.clone();
        self
    }

    /// Sets the label text
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Sets the indicator fill origin to Center
    pub fn with_indicator_from_center(mut self) -> Self {
        self.style.fill_origin = ParamSliderFillOrigin::Center;
        self
    }

    /// Disable displaying the Slider's current value
    pub fn without_value(mut self) -> Self {
        self.draw_value = false;
        self
    }

    /// Sets the Slider orientation
    pub fn with_orientation(mut self, orientation: ParamSliderOrientation) -> Self {
        self.style.orientation = orientation;
        self
    }

    /// Sets the tick count next to the indicator
    pub fn with_tick_count(mut self, tick_count: u32) -> Self {
        self.style.tick_count = tick_count;
        self
    }

    /// Sets the alternating tick length
    pub fn without_alt_tick_length(mut self) -> Self {
        self.style.tick_alt_length = self.style.tick_length;
        self
    }

    /// Sets the length of the Widget
    pub fn with_length(mut self, length: f32) -> Self {
        self.style.length = length;
        self
    }

    /// Sets the width of the Label
    pub fn with_label_width(mut self, width: f32) -> Self {
        self.style.label_width = width;
        self
    }
    
    /// Sets the label to only display the value, not the property name
    pub fn with_label_only_value(mut self) -> Self {
        self.style.label_only_value = true;
        self
    }

    /// Disables the indicator fill
    pub fn without_fill(mut self) -> Self {
        self.style.fill_color = Color32::TRANSPARENT;
        self.style.fill_color_hover = Color32::TRANSPARENT;
        self
    }

    fn is_interacting(&self, response: &Response) -> bool {
        (response.hovered() || response.dragged())
    }

    fn begin_drag(&self) {
        if let Some(param) = self.param {
            self.setter.begin_set_parameter(param);
        }
    }

    fn set_normalized_value(&self, normalized: f32) {
        // This snaps to the nearest plain value if the parameter is stepped in some way.
        // TODO: As an optimization, we could add a `const CONTINUOUS: bool` to the parameter to
        //       avoid this normalized->plain->normalized conversion for parameters that don't need
        //       it
        if let Some(param) = self.param {
            let value = param.preview_plain(normalized);
            if value != param.unmodulated_plain_value() {
                self.setter.set_parameter(param, value);
            }
        }
    }

    /// Begin and end drag still need to be called when using this. Returns `false` if the string
    /// could not be parsed.
    fn set_from_string(&self, string: &str) -> bool {
        if let Some(param) = self.param {
            match param.string_to_normalized_value(string) {
                Some(normalized_value) => {
                    self.set_normalized_value(normalized_value);
                    true
                }
                None => false,
            }
        } else {
            false
        }
    }

    /// Begin and end drag still need to be called when using this..
    fn reset_param(&self) {
        if let Some(param) = self.param {
            self.setter
                .set_parameter(param, param.default_plain_value());
        }
    }

    /// Begin and end drag still need to be called when using this..
    fn increment_param(&self, next: bool, fine: bool) {
        if let Some(param) = self.param {
            let current_value = param.unmodulated_plain_value();
            let new_value = if next {param.next_step(current_value, fine)} else {param.previous_step(current_value, fine)};
            self.setter
                .set_parameter(param, new_value);
        }
    }
    

    fn can_cycle(&self) -> bool {
        if let Some(param) = self.param {
            param.can_cycle()
        } else {
            false
        }
    }

    fn is_toggle(&self) -> bool {
        if let Some(param) = self.param {
            if let Some(step_count) = param.step_count() {
                param.can_cycle() && step_count == 1
            } else {
                false
            }
        } else {
            false
        }
    }

    fn cycle(&self) {
        if self.can_cycle() {
            if let Some(param) = self.param {
                let current_step = param.unmodulated_normalized_value();
                let mut next_step = param.next_normalized_step(current_step, false);
                if next_step == current_step {
                    next_step = 0.0;
                }
                self.set_normalized_value(next_step);
                /*
                self.set_normalized_value(if param.unmodulated_normalized_value() > 0.5 {
                    0.0
                } else {
                    1.0
                });
                */
            }
        }
    }

    fn perform_drag(&self, ui: &Ui, drag_delta: Vec2, fine: bool) {
        let start_value = Self::get_drag_normalized_start_value_memory(ui);

        let drag_scale = if self.is_toggle() {
            DRAG_SCALE_TOGGLE
        } else if self.can_cycle() {
            DRAG_SCALE_CYCLE / self.param.unwrap().step_count().unwrap() as f32
        } else if fine {
            DRAG_FINE_SCALE
        } else {
            DRAG_SCALE
        };

        let drag_delta = match self.style.orientation {
            ParamSliderOrientation::Horizontal => drag_delta.x,
            ParamSliderOrientation::Vertical => -drag_delta.y,
        };
        let total_drag_amount = Self::get_drag_amount_memory(ui) + drag_delta * drag_scale;
        Self::set_drag_amount_memory(ui, total_drag_amount);

        self.set_normalized_value((start_value + (total_drag_amount)).clamp(0.0, 1.0));
    }

    fn end_drag(&self) {
        if let Some(param) = self.param {
            self.setter.end_set_parameter(param);
        }
    }

    fn get_drag_normalized_start_value_memory(ui: &Ui) -> f32 {
        ui.data(|r|r
            .get_temp(*DRAG_NORMALIZED_START_VALUE_MEMORY_ID)
            .unwrap_or(0.5))
    }

    fn set_drag_normalized_start_value_memory(ui: &Ui, amount: f32) {
        ui.data_mut(|r| { r
            .insert_temp(*DRAG_NORMALIZED_START_VALUE_MEMORY_ID, amount);
        });
    }

    fn get_drag_amount_memory(ui: &Ui) -> f32 {
        ui.data(|r| r
            .get_temp(*DRAG_AMOUNT_MEMORY_ID)
            .unwrap_or(0.0))
    }

    fn set_drag_amount_memory(ui: &Ui, amount: f32) {
        ui.data_mut(|r| { r
            .insert_temp(*DRAG_AMOUNT_MEMORY_ID, amount);
        });
    }

    fn slider_ui(&self, ui: &mut Ui, response: &mut Response) {
        /*
         * There seems to be a 1-frame lag in detection, so dragging quickly off the slider
         * can result in a missed drag... This may need to be fixed upstream.
         */

         if response.clicked() {
            response.request_focus();
         }

         if let Some(param) = self.param {
            if response.drag_started() {
                response.surrender_focus();
                self.begin_drag();
                Self::set_drag_amount_memory(ui, 0.0);
                Self::set_drag_normalized_start_value_memory(
                    ui,
                    param.unmodulated_normalized_value(),
                );
            }
            if response.dragged() {
                let drag_cursor = match self.style.orientation {
                    ParamSliderOrientation::Horizontal => CursorIcon::ResizeHorizontal,
                    ParamSliderOrientation::Vertical => CursorIcon::ResizeVertical,
                };

                ui.output_mut(|r| { r.cursor_icon = drag_cursor; } );
                if let Some(_click_pos) = response.interact_pointer_pos() {
                    let fine = ui.input(|r|r.modifiers.shift);
                    self.perform_drag(ui, response.drag_delta(), fine);
                    response.mark_changed();
                }
            }
            if response.drag_stopped() {
                self.end_drag();
                ui.output_mut(|r| { r.cursor_icon = CursorIcon::Default; } );
            }

            let should_reset =
                response.double_clicked() || (response.clicked() && ui.input(|r|r.modifiers.ctrl));
            let can_cycle = self.can_cycle() && response.clicked();
            if should_reset || can_cycle {
                let must_fake_drag = !response.dragged(); // APIs require begin_set_parameter/end_set_parameter calls
                if must_fake_drag {
                    self.begin_drag();
                }
                if should_reset {
                    self.reset_param();
                } else {
                    self.cycle();
                }
                if must_fake_drag {
                    self.end_drag();
                }
                response.mark_changed();
            }

            if response.has_focus() && ui.input(|i| i.modifiers.ctrl) { // Keyboard Input
                ui.input(|r| {
                    for event in &r.events {
                        if let Event::Key{key, pressed, modifiers, repeat, physical_key} = event {
                            if *pressed {
                                let forward = match self.style.orientation {
                                    ParamSliderOrientation::Horizontal => match key {
                                        Key::ArrowRight => Some(true),
                                        Key::ArrowLeft => Some(false),
                                        _ => None,
                                    },
                                    ParamSliderOrientation::Vertical => match key {
                                        Key::ArrowUp => Some(true),
                                        Key::ArrowDown => Some(false),
                                        _ => None,
                                    },
                                };
                                if let Some(forward) = forward {
                                    self.begin_drag();
                                    self.increment_param(forward, modifiers.shift);
                                    self.end_drag();
                                }
                            }
                        }
                    }
                });
            }

            // Draw Visuals
            let is_interacting = self.is_interacting(response);
            let hovered_amount =
                ui.ctx()
                    .animate_bool_with_time(response.id, is_interacting, 0.100);

            // Smooth visuals?
            //let unmodulated_value = ui.ctx().animate_value_with_time(response.id, self.normalized_value(), 0.075);
            let unmodulated_value = param.unmodulated_normalized_value();
            let modulated_value = param.modulated_normalized_value();

            let size = match self.style.orientation {
                ParamSliderOrientation::Horizontal => {
                    Vec2::new(self.style.length, self.style.width)
                }
                ParamSliderOrientation::Vertical => Vec2::new(self.style.width, self.style.length),
            };

            let expand_bg_for_ticks = self.style.tick_count > 0;
            let bg_padding = if expand_bg_for_ticks {
                self.style.tick_length + self.style.track_margin
            } else {
                0.0
            };
            let mut slider_rect = Rect::from_center_size(response.rect.center(), size);
            if self.style.center_bg {
                match self.style.orientation {
                    ParamSliderOrientation::Horizontal => {
                        slider_rect = slider_rect.translate(Vec2::new(0.0, bg_padding * 0.5));
                    }
                    ParamSliderOrientation::Vertical => {
                        slider_rect = slider_rect.translate(Vec2::new(bg_padding * 0.5, 0.0));
                    }
                };
            }
            let track_rect =
                slider_rect.shrink(slider_rect.width().min(slider_rect.height()) * 0.5);

            let mut focus_rect = track_rect;
            let mut focus_rounding = Rounding::same(9999.0);

            // Backing
            let color = self.style.bg_color;
            if color.a() > 0 {
                let mut bg_rect = slider_rect;
                if expand_bg_for_ticks {
                    match self.style.orientation {
                        ParamSliderOrientation::Horizontal => {
                            bg_rect.set_top(bg_rect.top() - bg_padding);
                        }
                        ParamSliderOrientation::Vertical => {
                            bg_rect.set_left(bg_rect.left() - bg_padding);
                        }
                    };
                }
                let rounding = Rounding::same(self.style.width * 0.5); // Keep it nicely wrapped around the track
                ui.painter().rect_filled(bg_rect, rounding, color);
                focus_rect = bg_rect;
                focus_rounding = rounding;
            }

            // Tick Marks
            let color = self.style.tick_color;
            if color.a() > 0 {
                let start = match self.style.orientation {
                    ParamSliderOrientation::Horizontal => track_rect.left(),
                    ParamSliderOrientation::Vertical => track_rect.bottom(),
                };
                let end = match self.style.orientation {
                    ParamSliderOrientation::Horizontal => track_rect.right(),
                    ParamSliderOrientation::Vertical => track_rect.top(),
                };
                let step = (end - start) / (self.style.tick_count - 1) as f32;
                let center = track_rect.center();
                let odd_tick_count = self.style.tick_count % 2 == 1;
                let allow_alt = odd_tick_count && self.style.tick_count > 2;
                for i in 0..self.style.tick_count {
                    let pos = start + (i as f32) * step;
                    let tick_length = if i % 2 == 0 || !allow_alt {
                        self.style.tick_length
                    } else {
                        self.style.tick_alt_length
                    };
                    let rect = match self.style.orientation {
                        ParamSliderOrientation::Horizontal => Rect::from_center_size(
                            Pos2::new(
                                pos,
                                center.y - self.style.width * 0.5 - self.style.tick_length * 0.5,
                            ),
                            Vec2::new(self.style.tick_width, tick_length),
                        ),
                        ParamSliderOrientation::Vertical => Rect::from_center_size(
                            Pos2::new(
                                center.x - self.style.width * 0.5 - self.style.tick_length * 0.5,
                                pos,
                            ),
                            Vec2::new(tick_length, self.style.tick_width),
                        ),
                    };
                    //ui.painter().line_segment([], Stroke::new(self.style.tick_width, self.style.tick_color));
                    ui.painter()
                        .rect_filled(rect, Rounding::same(9999.0), self.style.tick_color);
                }
            }

            // Indicator Background
            let color = self.style.track_color;
            if color.a() > 0 {
                let track_rect = slider_rect.shrink(self.style.track_margin);
                if focus_rect == Rect::NOTHING {
                    focus_rect = track_rect;
                }
                ui.painter()
                    .rect_filled(track_rect, Rounding::same(9999.0), color);
            }

            // Indicator Fill
            let color = self.style.get_fill_color(is_interacting);
            let modulation_color = self.style.get_fill_modulation_color(is_interacting);
            let mut fill_rect =
                slider_rect.shrink(self.style.track_margin + self.style.fill_margin);
            let mut modulation_rect = fill_rect;
            match self.style.orientation {
                ParamSliderOrientation::Horizontal => {
                    let radius = fill_rect.height();
                    let start = fill_rect.left();
                    let end = start + fill_rect.width() - radius;
                    let center = (start + end) * 0.5;
                    let pos = start + (end - start) * unmodulated_value;

                    // Position
                    let modulated_pos = start + (end - start) * modulated_value;
                    modulation_rect.set_left(pos.min(modulated_pos) + radius * 0.5);
                    modulation_rect.set_right(pos.max(modulated_pos) + radius * 0.5);

                    // Make smaller
                    let modulation_width = self.style.track_margin;
                    modulation_rect.set_bottom(modulation_rect.top() + modulation_width);
                    modulation_rect = modulation_rect
                        .translate(Vec2::new(0.0, -self.style.fill_margin - modulation_width));

                    match self.style.fill_origin {
                        ParamSliderFillOrigin::Start => {
                            fill_rect.set_right(pos + radius);
                        }
                        ParamSliderFillOrigin::Center => {
                            fill_rect.set_right(center.max(pos) + radius);
                            fill_rect.set_left(center.min(pos));
                        }
                        ParamSliderFillOrigin::End => {
                            fill_rect.set_left(pos);
                        }
                    }
                }
                ParamSliderOrientation::Vertical => {
                    let radius = fill_rect.width();
                    let start = fill_rect.bottom();
                    let end = start - fill_rect.height() + radius;
                    let center = (start + end) * 0.5;
                    let pos = start + (end - start) * unmodulated_value;

                    // Position
                    let modulated_pos = start + (end - start) * modulated_value;
                    modulation_rect.set_top(pos.min(modulated_pos) - radius * 0.5);
                    modulation_rect.set_bottom(pos.max(modulated_pos) - radius * 0.5);

                    // Make smaller
                    let modulation_width = self.style.track_margin;
                    modulation_rect.set_right(modulation_rect.left() + modulation_width);
                    modulation_rect = modulation_rect
                        .translate(Vec2::new(-self.style.fill_margin - modulation_width, 0.0));

                    match self.style.fill_origin {
                        ParamSliderFillOrigin::Start => {
                            fill_rect.set_top(pos - radius);
                        }
                        ParamSliderFillOrigin::Center => {
                            fill_rect.set_top(center.min(pos) - radius);
                            fill_rect.set_bottom(center.max(pos));
                        }
                        ParamSliderFillOrigin::End => {
                            fill_rect.set_bottom(pos);
                        }
                    }
                }
            }

            if color.a() > 0 {
                ui.painter()
                    .rect_filled(fill_rect, Rounding::same(9999.0), color);
            }

            if modulation_rect.area() > 1.0 && modulation_color.a() > 0 {
                ui.painter()
                    .rect_filled(modulation_rect, Rounding::same(9999.0), modulation_color);
            }

            // Selection
            if response.has_focus() {
                let focus_stroke = ui.style().visuals.selection.stroke;
                if !focus_stroke.is_empty() {
                    ui.painter().rect_stroke(focus_rect, focus_rounding, focus_stroke);
                }
            }            

            // Knob
            let color = self.style.knob_color; //Color32::from_white_alpha(64);
            if color.a() > 0 || self.style.knob_outline_color.a() > 0 {
                let hover_scale = (1.0 + (self.style.knob_hover_scale - 1.0) * hovered_amount);
                let knob_rect = match self.style.orientation {
                    ParamSliderOrientation::Horizontal => {
                        let start = track_rect.left();
                        let range = track_rect.width();
                        let size = Vec2::new(
                            self.style.knob_width / self.style.knob_aspect,
                            self.style.knob_width,
                        ) * hover_scale;
                        Rect::from_center_size(
                            Pos2::new(start + range * unmodulated_value, track_rect.center().y),
                            size,
                        )
                    }
                    ParamSliderOrientation::Vertical => {
                        let start = track_rect.bottom();
                        let range = -track_rect.height();
                        let size = Vec2::new(
                            self.style.knob_width,
                            self.style.knob_width / self.style.knob_aspect,
                        ) * hover_scale;
                        Rect::from_center_size(
                            Pos2::new(track_rect.center().x, start + range * unmodulated_value),
                            size,
                        )
                    }
                };
                let radius =
                    knob_rect.width().min(knob_rect.height()) * self.style.knob_round_ratio * 0.5;
                ui.painter().rect(
                    knob_rect,
                    Rounding::same(radius),
                    color,
                    Stroke::new(
                        self.style.knob_outline_width * hover_scale,
                        self.style.knob_outline_color,
                    ),
                );
            }

        }
    }

    fn value_ui(&self, ui: &mut Ui, slider_response: &Response) {
        let is_interacting = self.is_interacting(slider_response) || slider_response.has_focus();;
        let text = if let Some(param) = self.param {
            if is_interacting || self.style.label_only_value {
                param.to_string()
            } else if let Some(label) = &self.label {
                label.to_string()
            } else {
                param.name().to_string()
            }
        } else {
            "???".to_string()
        };

        let mut label_style = if is_interacting {
            self.style.label_style_hover.clone()
        } else {
            self.style.label_style.clone()
        };
        label_style.width = Some(self.style.label_width);
        
        ui.add_space(self.style.label_offset);
        ui.horizontal(|ui|{
            ui.add_space((slider_response.rect.width()-self.style.label_width)/2.0);
            let label = Label::new(text).with_style(&label_style);
            ui.add(label);
        });
    }
}

impl<P: Param + ParamDetails> Widget for ParamSlider<'_, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        let required_width = self.style.required_width();

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.set_width(required_width);
            let mut response = ui.allocate_response(
                Vec2::new(required_width, self.style.required_height()),
                Sense::click_and_drag(),
            );

            // Filter Navigation Keys
            let is_editing = response.has_focus() && ui.input(|i| i.modifiers.ctrl);
            ui.memory_mut(|mem| mem.set_focus_lock_filter(response.id, EventFilter{horizontal_arrows:is_editing,vertical_arrows:is_editing,escape:true, ..Default::default()}));

            self.slider_ui(ui, &mut response);
            if self.draw_value {
                self.value_ui(ui, &response);
            }

            response
        })
        .inner
    }
}
