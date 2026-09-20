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

use nih_plug_egui::egui::{
    vec2,
    Vec2,
    Align2,
    Color32,
    FontFamily,
    FontId,
    Response,
    Rounding,
    Sense,
    Ui,
    Widget,
    Shape,
    epaint::{
        Shadow,
    }
};

use crate::color::InterpolateColors;

/// Style for a Label
#[derive(Clone)]
pub struct LabelStyle {
    /// Optional width of the Label
    pub width: Option<f32>,

    /// Color of the Text
    pub color: Color32,

    /// Color of the Text when hovering
    pub color_hover: Option<Color32>,

    /// Height of the background, which controls the UI Widget's height
    pub bg_height: f32,

    /// Color of the background
    pub bg_color: Color32,
    
    /// Color of the background when hovering
    pub bg_color_hover: Option<Color32>,

    /// ID of the fonts
    pub font_id: FontId,

    /// Optional background Shadow
    pub shadow: Option<Shadow>,

    /// Background Shadow offset
    pub shadow_offset: Vec2,
}

impl Default for LabelStyle {
    fn default() -> Self {
        Self {
            width: None,
            color: Color32::LIGHT_GRAY,
            color_hover: None,

            bg_height: 18.0,
            bg_color: Color32::TRANSPARENT,
            bg_color_hover: None,

            // Probably best to use the UI specified font by default:
            font_id: FontId::new(17.0, FontFamily::Proportional),
            //font_id: FontId::new(17.0, FontFamily::Name("Label".into())),
            //font_id: FontId::new(25.0, FontFamily::Name("Title".into())),
            shadow: None,
            shadow_offset: Vec2::new(0.0, 0.0),
        }
    }
}

impl LabelStyle {
    /// Returns a default style for hovered Labels
    pub fn default_hover() -> Self {
        Self {
            color: Color32::BLACK,
            bg_color: Color32::LIGHT_GRAY,
            ..Default::default()
        }
    }

    /// Returns a default style for Headings
    pub fn default_heading() -> Self {
        Self {
            color: Color32::WHITE,
            font_id: FontId::new(22.0, FontFamily::Proportional),
            bg_color: Color32::DARK_GRAY,
            bg_height: 22.0,
            shadow: Some(Shadow { offset: Vec2::new(0.0, 2.0), blur: 0.0, spread: 0.0, color: Color32::from_black_alpha(16) }),
            shadow_offset: vec2(0.0, 1.0),
            ..Default::default()
        }
    }
}

/// Text label with an optional backgound and shadow
pub struct Label {
    style: LabelStyle,
    text: String,
    clickable: bool,
}

#[allow(unused)]
impl Label {
    /// Returns a new Label widget
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            style: LabelStyle::default(),
            text: text.into(),
            clickable: false,
        }
    }

    /// Sets the style
    pub fn with_style(mut self, style: &LabelStyle) -> Self {
        self.style = style.clone();
        self
    }

    /// Sets the text color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.style.color = color;
        self
    }

    /// Sets the text color when hovering
    pub fn with_color_hover(mut self, color: Color32) -> Self {
        self.style.color_hover = Some(color);
        self
    }

    /// Sets the background color
    pub fn with_bg_color(mut self, color: Color32) -> Self {
        self.style.bg_color = color;
        self
    }

    /// Sets the background color when hovering
    pub fn with_bg_color_hover(mut self, color: Color32) -> Self {
        self.style.bg_color_hover = Some(color);
        self
    }

    /// Enables clicking and interacting with the label (for list views)
    pub fn with_clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    /// Sets the shadow and offset
    pub fn with_shadow(mut self, shadow: Shadow, offset: Vec2) -> Self {
        self.style.shadow = Some(shadow);
        self.style.shadow_offset = offset;
        self
    }
}

impl Widget for Label {
    fn ui(self, ui: &mut Ui) -> Response {
        let sense = if self.clickable {
            Sense::click()
        } else {
            Sense::hover()
            //Sense::focusable_noninteractive()
        };
        
        let (rect, response) = ui.allocate_exact_size(
            vec2(
                self.style.width.unwrap_or_else(|| ui.available_width()),
                self.style.bg_height,
            ),
            sense
        );
        
        let hovered_amount = ui.ctx().animate_bool_with_time(response.id, response.hovered(), 0.100);

        let mut color = self.style.bg_color;
        if let Some(to_color) = self.style.bg_color_hover {
            if hovered_amount > 0.0 {
                color = color.interpolate_to(to_color, hovered_amount);
            }
        }

        let rounding = Rounding::same(9999.0);

        if let Some(shadow) = self.style.shadow {
            let shadow = shadow.tessellate(rect.translate(self.style.shadow_offset), rounding);
            let shadow = Shape::Mesh(shadow);
            ui.painter().add(shadow);
        }

        if color.a() > 0 {
            ui.painter()
                .rect_filled(rect, rounding, color);
        }

        let mut color = self.style.color;
        if let Some(to_color) = self.style.color_hover {
            if hovered_amount > 0.0 {
                color = color.interpolate_to(to_color, hovered_amount);
            }
        }
        ui.painter().text(
            rect.center(),
            Align2::CENTER_CENTER,
            &self.text,
            self.style.font_id,
            color,
        );

        if response.has_focus() {
            let focus_stroke = ui.style().visuals.selection.stroke;
            if !focus_stroke.is_empty() {
                ui.painter().rect_stroke(rect, rounding, focus_stroke);
            }
        }

        response
    }
}
