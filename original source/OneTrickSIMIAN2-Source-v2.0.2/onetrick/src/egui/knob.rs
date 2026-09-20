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

use std::f32::consts::{PI, TAU};
//use std::sync::Arc;
//use substring::Substring;

use lazy_static::lazy_static;
use nih_plug::prelude::{Param, ParamSetter};
use nih_plug_egui::egui::{
    epaint::PathShape,
    output::CursorIcon,
    Color32,
    Id,
    Pos2,
    //        Align2,
    Response,
    Sense,
    Shape,
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
use super::shape::ShapeEx;
//use super::icons::{Icon, IconDrawer};

const DRAG_SCALE: f32 = 0.0025;
const DRAG_FINE_SCALE: f32 = 0.0005;

lazy_static! {
    static ref DRAG_NORMALIZED_START_VALUE_MEMORY_ID: Id = Id::new((file!(), 0));
    static ref DRAG_AMOUNT_MEMORY_ID: Id = Id::new((file!(), 1));
    static ref VALUE_ENTRY_MEMORY_ID: Id = Id::new((file!(), 2));
}

/// Display settings for a Knob's Tick Circle
#[derive(Clone)]
pub struct ParamKnobTickStyleCircle {
    /// The normalized radius of the tick mark
    pub relative_radius: f32,
}
impl Default for ParamKnobTickStyleCircle {
    fn default() -> Self {
        Self {
            relative_radius: 0.1,
        }
    }
}

/// Display settings for a Knob's Tick Rectangle/Line
#[derive(Clone)]
pub struct ParamKnobTickStyleRectangle {
    /// Normalized width of the tick mark
    pub relative_width: f32,

    /// Normalized length of the tick mark
    pub relative_length: f32,
}
impl Default for ParamKnobTickStyleRectangle {
    fn default() -> Self {
        Self {
            relative_width: 0.075,
            relative_length: 0.175,
        }
    }
}

/// Display settings for a Knob's Tick Triangle
#[derive(Clone)]
pub struct ParamKnobTickStyleTriangle {
    /// Normalized width of the tick mark
    pub relative_width: f32,

    /// Normalized length of the tick mark
    pub relative_length: f32,
}
impl Default for ParamKnobTickStyleTriangle {
    fn default() -> Self {
        Self {
            relative_width: 0.15,
            relative_length: 0.1,
        }
    }
}

/// A Knob's Tick style
#[derive(Clone)]
pub enum ParamKnobTickStyle {
    Circle(ParamKnobTickStyleCircle),
    Rectangle(ParamKnobTickStyleRectangle),
    Triangle(ParamKnobTickStyleTriangle),
}

/// A Knob's Indicator style
#[derive(Clone)]
pub enum ParamKnobIndicatorStyle {
    Continuous,
    Dots,
    //Bars,
}

/// A Knob's Indicator origin
#[derive(Clone)]
pub enum ParamKnobIndicatorOrigin {
    Start,
    Center,
    End,
}

/// Style of a Knob
#[derive(Clone)]
pub struct ParamKnobStyle {
    /// Radius of the knob
    pub radius: f32,

    /// Background color
    pub bg_color: Color32,

    /// Range the Knob can turn in Radians
    pub knob_angle_range: f32,

    /// Relative radius of the Knob compared to the entire UI Widget with Background
    pub knob_relative_radius: f32,

    /// Amount to scale the Knob when zooming
    pub knob_hover_scale: f32,

    /// Color of the Knob
    pub knob_color: Color32,

    /// Tick style of the Knob
    pub knob_tick_style: ParamKnobTickStyle,

    /// Color of the Knob Tick
    pub knob_tick_color: Color32,

    /// Normalized offset of the Knob Tick
    pub knob_tick_relative_offset: f32,

    /// Indicator style of the Knob
    pub indicator_style: ParamKnobIndicatorStyle,

    /// Indicator count (for dotted)
    pub indicator_count: u32,

    /// Origin of the Indicator
    pub indicator_origin: ParamKnobIndicatorOrigin,

    /// Normalized radius of the Indicator
    pub indicator_relative_radius: f32,

    /// Thickness of the Indicator
    pub indicator_width: f32,

    /// Background color of the Indicator
    pub indicator_bg_color: Color32,

    /// Fill color of the Indicator
    pub indicator_fill_color: Color32,

    /// Fill color of the Indicator when hovering
    pub indicator_fill_color_hover: Color32,

    /// Fill color of the Indicator when being modulated (CLAP)
    pub indicator_modulation_color: Color32,

    /// Fill color of the Indicator when being modulated and hovering (CLAP)
    pub indicator_modulation_color_hover: Color32,

    /// The detent color of the Indicator (center dot)
    pub indicator_detent_color: Color32,

    /// The value at which an Indicator will display a warning color (clipping)
    pub indicator_warning_value: Option<f32>,

    /// The color o the Indicator when displaying a warning (clipping)
    pub indicator_warning_color: Color32,

    /// Style of the Label
    pub label_style: LabelStyle,

    /// Style of the Label when hovering
    pub label_style_hover: LabelStyle,

    /// Offset of the Label from the Bottom of the knob
    pub label_offset: f32,
}
impl ParamKnobStyle {
    /// Returns the Indicator fill color
    pub fn get_indicator_fill_color(&self, hovered: bool) -> Color32 {
        if hovered {
            self.indicator_fill_color_hover
        } else {
            self.indicator_fill_color
        }
    }

    /// Returns the Indicator color when modulating (CLAP)
    pub fn get_indicator_modulation_color(&self, hovered: bool) -> Color32 {
        if hovered {
            self.indicator_modulation_color_hover
        } else {
            self.indicator_modulation_color
        }
    }

    /// Returns the width required to display this Knob
    pub fn required_width(&self) -> f32 {
        self.radius * 2.0
        //self.knob_relative_radius.max(self.indicator_relative_radius) * 2.0
    }
}

impl Default for ParamKnobStyle {
    fn default() -> Self {
        Self {
            radius: 40.0,
            bg_color: Color32::BLACK,

            knob_angle_range: TAU * 0.75,

            knob_relative_radius: 0.7,
            knob_hover_scale: 1.1,
            knob_color: Color32::LIGHT_GRAY,

            //knob_tick_style: ParamKnobTickStyle::Circle(ParamKnobTickStyleCircle::default()),
            knob_tick_style: ParamKnobTickStyle::Rectangle(ParamKnobTickStyleRectangle::default()),
            //knob_tick_style: ParamKnobTickStyle::Triangle(ParamKnobTickStyleTriangle::default()),
            knob_tick_color: Color32::DARK_GRAY,
            knob_tick_relative_offset: 0.85,

            //indicator_style: ParamKnobIndicatorStyle::Continuous,
            indicator_style: ParamKnobIndicatorStyle::Dots,
            indicator_count: 21,
            indicator_origin: ParamKnobIndicatorOrigin::Start,
            indicator_relative_radius: 0.9,
            indicator_width: 4.0,
            indicator_bg_color: Color32::DARK_GRAY,
            indicator_fill_color: Color32::LIGHT_GRAY,
            indicator_fill_color_hover: Color32::WHITE,
            indicator_modulation_color: Color32::YELLOW,
            indicator_modulation_color_hover: Color32::GRAY,
            indicator_detent_color: Color32::YELLOW,
            indicator_warning_value: None,
            indicator_warning_color: Color32::LIGHT_RED,

            label_style: LabelStyle::default(),
            label_style_hover: LabelStyle::default_hover(),
            label_offset: 2.0,
        }
    }
}

/// A Knob Widget that can modify a Plugin Parameter
pub struct ParamKnob<'a, P: Param> {
    param: Option<&'a P>,
    setter: &'a ParamSetter<'a>,
    style: ParamKnobStyle,
    label: Option<String>,

    draw_value: bool,
    //animation_id: Id, // Not needed!
}

#[allow(unused)]
impl<'a, P: Param> ParamKnob<'a, P> {
    /// Returns a new knob for a parameter
    pub fn for_param(param: Option<&'a P>, setter: &'a ParamSetter<'a>) -> Self {
        Self {
            param,
            setter,
            style: ParamKnobStyle::default(),
            label: None,

            draw_value: true,
            //animation_id: Id::new((file!(), param.name(), "animation")), // Not needed
        }
    }

    /// Sets the style of the knob
    pub fn with_style(mut self, style: &ParamKnobStyle) -> Self {
        self.style = style.clone();
        self
    }

    /// Sets the Label text
    pub fn with_label(mut self, label: &str) -> Self {
        self.label = Some(label.to_string());
        self
    }

    /// Sets the Indicator style
    pub fn with_indicator_style(mut self, indicator_style: ParamKnobIndicatorStyle) -> Self {
        self.style.indicator_style = indicator_style;
        self
    }

    /// Sets the Indicator count
    pub fn with_indicator_count(mut self, indicator_count: u32) -> Self {
        self.style.indicator_count = indicator_count;
        self
    }

    /// Sets the Indicator's origin to Start
    pub fn with_indicator_from_start(mut self) -> Self {
        self.style.indicator_origin = ParamKnobIndicatorOrigin::Start;
        self
    }

    /// Sets the Indicator's origin to Center
    pub fn with_indicator_from_center(mut self) -> Self {
        self.style.indicator_origin = ParamKnobIndicatorOrigin::Center;
        self
    }

    /// Sets the Indicator's origin to End
    pub fn with_indicator_from_end(mut self) -> Self {
        self.style.indicator_origin = ParamKnobIndicatorOrigin::End;
        self
    }

    /// Disable displaying the Knob's current value.
    pub fn without_value(mut self) -> Self {
        self.draw_value = false;
        self
    }

    /// Returns True if being hovered or dragged
    fn is_interacting(&self, response: &Response) -> bool {
        (response.hovered() || response.dragged())
    }

    /// Begins a drag operation, allowing the Parameter to be modified
    fn begin_drag(&self) {
        if let Some(param) = self.param {
            self.setter.begin_set_parameter(param);
        }
    }

    /// Sets the Parameter from a normalized value
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

    /// Sets the value from a string, and returns True if successful
    /// Be sure to call between begin_drag() and end_drag() calls.
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

    /// Resets the Paramter value to default
    /// Be sure to call between begin_drag() and end_drag() calls.
    fn reset_param(&self) {
        if let Some(param) = self.param {
            self.setter
                .set_parameter(param, param.default_plain_value());
        }
    }

    /// Increments or decrements the Parameter value
    /// Be sure to call between begin_drag() and end_drag() calls.
    fn increment_param(&self, next: bool, fine: bool) {
        if let Some(param) = self.param {
            let current_value = param.unmodulated_plain_value();
            let new_value = if next {param.next_step(current_value, fine)} else {param.previous_step(current_value, fine)};
            self.setter
                .set_parameter(param, new_value);
        }
    }
    
    /// Updates the current drag operation
    fn perform_drag(&self, ui: &Ui, drag_delta: Vec2, fine: bool) {
        let start_value = Self::get_drag_normalized_start_value_memory(ui);

        let drag_scale = if fine { DRAG_FINE_SCALE } else { DRAG_SCALE };

        let total_drag_amount = Self::get_drag_amount_memory(ui) + (-drag_delta.y) * drag_scale;
        Self::set_drag_amount_memory(ui, total_drag_amount);

        self.set_normalized_value((start_value + (total_drag_amount)).clamp(0.0, 1.0));
    }

    /// Ends the current drag operation
    fn end_drag(&self) {
        if let Some(param) = self.param {
            self.setter.end_set_parameter(param);
        }
    }

    /// Returns the normalized Parameter value the drag operation began at
    fn get_drag_normalized_start_value_memory(ui: &Ui) -> f32 {
        ui.data(|r| { r
            .get_temp(*DRAG_NORMALIZED_START_VALUE_MEMORY_ID)
            .unwrap_or(0.5)
        })
    }

    /// Sets the normalized Parameter value for a new drag operation
    fn set_drag_normalized_start_value_memory(ui: &Ui, amount: f32) {
        ui.data_mut(|r| { r
            .insert_temp(*DRAG_NORMALIZED_START_VALUE_MEMORY_ID, amount);
        })
    }

    /// Returns the amount we've dragged so far from memory
    fn get_drag_amount_memory(ui: &Ui) -> f32 {
        ui.data(|r| { r
            .get_temp(*DRAG_AMOUNT_MEMORY_ID)
            .unwrap_or(0.0)
        })
    }

    /// Sets the amount we've dragged so far into memory
    fn set_drag_amount_memory(ui: &Ui, amount: f32) {
        ui.data_mut(|r| { r
            .insert_temp(*DRAG_AMOUNT_MEMORY_ID, amount);
        });
    }

    /// Run the UI for this Knob
    fn knob_ui(&self, ui: &mut Ui, response: &mut Response) {
        /*
         * There seems to be a 1-frame lag in detection, so dragging quickly off the knob
         * can result in a missed drag... This may need to be fixed upstream.
         */

         if response.clicked() {
            response.request_focus();
         }

        if let Some(param) = self.param {
            let unmodulated_normalized_value = param.unmodulated_normalized_value();

            if response.drag_started() {
                response.surrender_focus();
                self.begin_drag();
                Self::set_drag_amount_memory(ui, 0.0);
                Self::set_drag_normalized_start_value_memory(ui, unmodulated_normalized_value);
            }
            if response.dragged() {
                ui.output_mut(|r| {
                    r.cursor_icon = CursorIcon::ResizeVertical;
                });

                if let Some(_click_pos) = response.interact_pointer_pos() {
                    let fine = ui.input(|r| 
                        r.modifiers.shift
                    );
                    self.perform_drag(ui, response.drag_delta(), fine);
                    response.mark_changed();
                }
            }
            if response.drag_stopped() {
                self.end_drag();
                ui.output_mut(|r| {
                    r.cursor_icon = CursorIcon::Default;
                });
            }

            if response.double_clicked() || (response.clicked() && ui.input(|r|r.modifiers.ctrl)) {
                let must_fake_drag = !response.dragged(); // APIs require begin_set_parameter/end_set_parameter calls
                if must_fake_drag {
                    self.begin_drag();
                }
                self.reset_param();
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
                                let forward = match key {
                                    Key::ArrowUp | Key::ArrowRight => Some(true),
                                    Key::ArrowDown | Key::ArrowLeft => Some(false),
                                    _ => None,
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

            // And finally draw the thing
            let is_interacting = self.is_interacting(response);
            let hovered_amount =
                ui.ctx()
                    .animate_bool_with_time(response.id, is_interacting, 0.100);

            // Smooth visuals?
            //let modulated_normalized_value = ui.ctx().animate_value_with_time(response.id, self.normalized_value(), 0.075);
            let modulated_normalized_value = param.modulated_normalized_value();

            let min_angle = PI * 0.5 + self.style.knob_angle_range * 0.5;
            let max_angle = min_angle - self.style.knob_angle_range;
            let angle_range = -self.style.knob_angle_range;
            let unmodulated_value_angle = min_angle + angle_range * unmodulated_normalized_value;
            let value_angle = min_angle + angle_range * modulated_normalized_value;

            if ui.is_rect_visible(response.rect) {
                let knob_center = response.rect.center();
                {
                    // Background
                    ui.painter()
                        .circle_filled(knob_center, self.style.radius, self.style.bg_color);

                    // Knob
                    let hover_scale = (1.0 + (self.style.knob_hover_scale - 1.0) * hovered_amount);
                    let radius = self.style.radius * self.style.knob_relative_radius * hover_scale;
                    ui.painter()
                        .circle_filled(knob_center, radius, self.style.knob_color);

                    let tick_offset = self.style.knob_tick_relative_offset * radius;
                    let unit_x = Vec2::new(
                        unmodulated_value_angle.cos(),
                        -unmodulated_value_angle.sin(),
                    ); //Vec2::angled(value_angle);
                    let tick_pos = knob_center + unit_x * tick_offset;

                    match &self.style.knob_tick_style // Tick
                    {
                        ParamKnobTickStyle::Circle(tick_style) => {
                            let tick_radius = tick_style.relative_radius * radius;
                            let tick_pos = tick_pos - unit_x * tick_radius; // Align to outer edge
                            ui.painter().circle_filled(tick_pos, tick_radius, self.style.knob_tick_color);
                        }
                        ParamKnobTickStyle::Rectangle(tick_style) => {
                            let tick_width = tick_style.relative_width * radius * 2.0;
                            let tick_length = tick_style.relative_length * radius * 2.0;
                            let a:Pos2 = tick_pos;
                            let b:Pos2 = tick_pos - unit_x * tick_length;
                            ui.painter().line_segment([a, b], Stroke::new(tick_width, self.style.knob_tick_color));
                        }
                        ParamKnobTickStyle::Triangle(tick_style) => {
                            let tick_width = tick_style.relative_width * radius * 2.0;
                            let tick_length = tick_style.relative_length * radius * 2.0;
                            let unit_y = unit_x.rot90();
                            let a:Pos2 = tick_pos;
                            let b:Pos2 = a-unit_x*tick_length+unit_y*tick_width * 0.5;
                            let c:Pos2 = a-unit_x*tick_length-unit_y*tick_width * 0.5;
                            let points = vec![a,b,c];
                            let path = PathShape::convex_polygon(points, self.style.knob_tick_color, Stroke::default());
                            ui.painter().add(Shape::Path(path));
                        }
                    }
                    //ui.painter().icon(knob_center, Icon::TomDrumMedium, radius*4.0, Color32::BLACK);
                }

                let arc_radius = (self.style.radius * self.style.indicator_relative_radius)
                    - self.style.indicator_width / 2.0;
                let indicator_origin = knob_center;
                let fill_color = self.style.get_indicator_fill_color(is_interacting);
                let modulation_color = self.style.get_indicator_modulation_color(is_interacting);
                match self.style.indicator_style // Indicator
                {
                    ParamKnobIndicatorStyle::Continuous => {

                        let (bg_angle_start, bg_angle_end, fill_angle_start, fill_angle_end) =
                            match self.style.indicator_origin {
                                ParamKnobIndicatorOrigin::Start => {
                                    (value_angle.max(unmodulated_value_angle), max_angle, min_angle, value_angle.max(unmodulated_value_angle))
                                }
                                ParamKnobIndicatorOrigin::Center => {
                                    (min_angle, max_angle, PI * 0.5, unmodulated_value_angle)
                                }
                                ParamKnobIndicatorOrigin::End => {
                                    (min_angle, value_angle.min(unmodulated_value_angle), value_angle.min(unmodulated_value_angle), max_angle)
                                }
                            };

                        ui.painter().add(
                            Shape::arc_stroke(
                                indicator_origin,
                                arc_radius,
                                bg_angle_start,
                                bg_angle_end,
                                Stroke::new(self.style.indicator_width, self.style.indicator_bg_color)
                                )
                            );

                        ui.painter().add(
                            Shape::arc_stroke(
                                indicator_origin,
                                arc_radius,
                                fill_angle_start,
                                fill_angle_end.max(-TAU),
                                Stroke::new(self.style.indicator_width, fill_color)
                                )
                            );

                        if unmodulated_value_angle != value_angle {
                            ui.painter().add( // Visualize Modulation
                                Shape::arc_stroke(
                                    indicator_origin,
                                    arc_radius,
                                    unmodulated_value_angle,
                                    value_angle,
                                    Stroke::new(self.style.indicator_width, modulation_color)
                                    )
                                );
                        }
                    }
                    ParamKnobIndicatorStyle::Dots => {
                        let count = self.style.indicator_count;
                        let angle_step = (max_angle - min_angle) / ((count - 1) as f32);
                        let dot_radius = self.style.indicator_width * 0.5;
                        let closest_value_index = ((unmodulated_value_angle - min_angle) / angle_range * ((count-1) as f32) + 0.5).abs().floor() as u32;
                        let modulated_closest_value_index = ((value_angle - min_angle) / angle_range * ((count-1) as f32) + 0.5).abs().floor() as u32;
                        for i in 0..count {
                            let angle = min_angle + (i as f32) * angle_step;
                            let v = Vec2::new(angle.cos(), -angle.sin());//Vec2::angled(angle);
                            let pos = indicator_origin + v * arc_radius;
                            let mut color = self.style.indicator_bg_color;
                            match self.style.indicator_origin {
                                ParamKnobIndicatorOrigin::Start => {
                                    if i <= modulated_closest_value_index {
                                        color = fill_color;
                                    }
                                }
                                ParamKnobIndicatorOrigin::Center => {
                                    let center_value_index = (count-1) / 2;
                                    if (i <= center_value_index && i >= closest_value_index) || (i >= center_value_index && i <= closest_value_index) {
                                        color = fill_color;
                                    }
                                    if !is_interacting && i == center_value_index && closest_value_index == center_value_index { // Detent
                                        color = self.style.indicator_detent_color;
                                    }
                                }
                                ParamKnobIndicatorOrigin::End => {
                                    if i >= closest_value_index {
                                        color = fill_color;
                                    }
                                }
                            }

                            let modulated_fill =
                                value_angle != unmodulated_value_angle &&
                                (
                                    (i >= closest_value_index && i <= modulated_closest_value_index) ||
                                    (i <= closest_value_index && i >= modulated_closest_value_index)
                                );
                            if modulated_fill {
                                color = modulation_color;
                            }

                            ui.painter().circle_filled(pos, dot_radius, color);
                        }
                    }
                }
                if response.has_focus() {
                    let focus_stroke = ui.style().visuals.selection.stroke;
                    if !focus_stroke.is_empty() {
                        ui.painter().circle_stroke(knob_center, self.style.radius, focus_stroke);
                    }
                }
            }
        }
    }

    /// Run the UI for the Knob's Label
    fn value_ui(&self, ui: &mut Ui, knob_response: &Response) {
        let is_interacting = self.is_interacting(knob_response) || knob_response.has_focus();
        let text = if let Some(param) = self.param {
            if is_interacting {
                param.to_string()
            } else if let Some(label) = &self.label {
                label.to_string()
            } else {
                param.name().to_string()
            }
        } else {
            "???".to_string()
        };

        let label_style = if is_interacting {
            self.style.label_style_hover.clone()
        } else {
            self.style.label_style.clone()
        };
        ui.add_space(self.style.label_offset);
        let label = Label::new(text).with_style(&label_style);
        ui.add(label);
    }
}

impl<P: Param> Widget for ParamKnob<'_, P> {
    fn ui(self, ui: &mut Ui) -> Response {
        let required_width = self.style.required_width();

        ui.vertical(|ui| {
            ui.spacing_mut().item_spacing.y = 0.0;
            ui.set_width(required_width);
            let mut response = ui.allocate_response(
                Vec2::new(required_width, required_width),
                Sense::click_and_drag(),
            );

            // Filter Navigation Keys
            let is_editing = response.has_focus() && ui.input(|i| i.modifiers.ctrl);
            ui.memory_mut(|mem| mem.set_focus_lock_filter(response.id, EventFilter{horizontal_arrows:is_editing,vertical_arrows:is_editing,escape:true, ..Default::default()}));

            self.knob_ui(ui, &mut response);
            if self.draw_value {
                self.value_ui(ui, &response);
            }

            response
        })
        .inner
    }
}
