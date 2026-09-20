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
    vec2, Color32, Response, Rounding, Sense, Ui, Widget,
};

/// Orientation for a Separator Widget
#[derive(Clone)]
pub enum SeparatorOrientation {
    Horizontal,
    Vertical,
}

/// Style for a Separator Widget
#[derive(Clone)]
pub struct SeparatorStyle {
    /// Orientation of the Separator
    pub orientation: SeparatorOrientation,

    /// Thickness of the Separator line
    pub width: f32,

    /// Padding on either side of the line
    pub padding: f32,

    /// Color of the line
    pub color: Color32,
}

impl Default for SeparatorStyle {
    fn default() -> Self {
        Self {
            orientation: SeparatorOrientation::Vertical,
            width: 2.0,
            padding: 5.0,
            color: Color32::GRAY,
        }
    }
}

/// A Separator Widget
#[derive(Default)]
pub struct Separator {
    style: SeparatorStyle,
}

#[allow(unused)]
impl Separator {
    /// Returns a new Separator Widget
    pub fn new() -> Self {
        Self {
            style: SeparatorStyle::default(),
        }
    }

    /// Sets the style
    pub fn with_style(mut self, style: &SeparatorStyle) -> Self {
        self.style = style.clone();
        self
    }

    /// Sets the orientation
    pub fn with_orientation(mut self, orientation: SeparatorOrientation) -> Self {
        self.style.orientation = orientation;
        self
    }

    /// Sets the width
    pub fn with_width(mut self, width: f32) -> Self {
        self.style.width = width;
        self
    }

    /// Sets the padding
    pub fn with_padding(mut self, padding: f32) -> Self {
        self.style.padding = padding;
        self
    }

    /// Sets the color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.style.color = color;
        self
    }
}

impl Widget for Separator {
    fn ui(self, ui: &mut Ui) -> Response {
        let total_thickness = self.style.width + self.style.padding * 2.0;
        let size = match self.style.orientation {
            SeparatorOrientation::Vertical => vec2(total_thickness, ui.available_height()),
            SeparatorOrientation::Horizontal => vec2(ui.available_width(), total_thickness),
        };
        let (rect, response) = ui.allocate_exact_size(
            size,
            Sense::hover()
        );

        let color = self.style.color;
        if color.a() > 0 {
            /*
            ui.painter() // Visualize padding:
                .rect_filled(rect, Rounding::same(9999.0), Color32::GRAY);
            */
            ui.painter()
                .rect_filled(rect.shrink(self.style.padding), Rounding::same(9999.0), color);
        }

        response
    }
}
