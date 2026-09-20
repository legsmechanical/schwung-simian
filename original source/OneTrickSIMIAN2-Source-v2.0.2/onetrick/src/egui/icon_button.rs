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
    Align2,
    Color32,
    Response,
    Rounding,
    Sense,
    Ui,
    Widget,
    EventFilter,
};

use super::icons::{Dingbat, IconDrawer};

/// Style info for an IconButton
#[derive(Clone)]
pub struct IconButtonStyle {
    /// The XY size of the entire button
    pub size: f32,

    /// The XY padding
    pub padding: f32,

    /// The amount of rounding, this should probably be replaced with a Rounding type
    pub rounding: f32,

    /// The text color
    pub color: Color32,

    /// The text color when hovering
    pub color_hover: Color32,

    /// The background color
    pub bg_color: Color32,

    /// The background color when hovering
    pub bg_color_hover: Color32,
}

impl Default for IconButtonStyle {
    fn default() -> Self {
        Self {
            size: 32.0,
            padding: 5.0,
            rounding: 5.0,
            color: Color32::LIGHT_GRAY,
            color_hover: Color32::YELLOW,
            bg_color: Color32::DARK_GRAY,
            bg_color_hover: Color32::BLACK,
        }
    }
}

/// A simple button with Dingbat icon instead of text
pub struct IconButton {
    dingbat: Dingbat,
    style: IconButtonStyle,
}

#[allow(unused)]
impl IconButton {
    /// Returns a new IconButton
    pub fn new(dingbat: Dingbat) -> Self {
        Self {
            dingbat,
            style: IconButtonStyle::default(),
        }
    }

    /// Sets the style
    pub fn with_style(mut self, style: &IconButtonStyle) -> Self {
        self.style = style.clone();
        self
    }

    /// Removes the background
    pub fn with_no_bg(mut self) -> Self {
        self.style.bg_color = Color32::TRANSPARENT;
        self.style.bg_color_hover = Color32::TRANSPARENT;
        self
    }
    
    /// Sets the text color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.style.color = color;
        self
    }

    /// Sets the text color when hovering
    pub fn with_color_hover(mut self, color: Color32) -> Self {
        self.style.color_hover = color;
        self
    }
}

impl Widget for IconButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(vec2(self.style.size, self.style.size), Sense::click());

        if response.clicked() {
            response.request_focus();
        }

        // Filter Navigation Keys
        ui.memory_mut(|mem| mem.set_focus_lock_filter(response.id, EventFilter{horizontal_arrows:false,vertical_arrows:false,escape:true, ..Default::default()}));

        let color = if response.hovered() || response.has_focus() {
            self.style.bg_color_hover
        } else {
            self.style.bg_color
        };
        if color.a() > 0 {
            ui.painter()
                .rect_filled(rect, Rounding::same(self.style.rounding), color);
        }

        let color = if response.hovered() || response.has_focus() {
            self.style.color_hover
        } else {
            self.style.color
        };
        ui.painter().dingbat(
            rect.center(),
            Align2::CENTER_CENTER,
            self.dingbat,
            self.style.size - self.style.padding * 2.0,
            color,
        );

        if response.has_focus() {
            let focus_stroke = ui.style().visuals.selection.stroke;
            if !focus_stroke.is_empty() {
                ui.painter().rect_stroke(rect, self.style.rounding, focus_stroke);
            }
        }

        response
    }
}
