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

pub use nih_plug_egui::egui::ecolor::{Color32, Hsva, HsvaGamma, Rgba};


/// Adds linear interpolation to built-in Colors
pub trait InterpolateColors {
    fn interpolate_to(&self, to: Self, t: f32) -> Self;
}

impl InterpolateColors for Rgba {
    fn interpolate_to(&self, to: Rgba, t: f32) -> Rgba {
        let from = *self;
        let inv_t = 1.0 - t;
        Rgba::from_rgba_premultiplied(
            from.r() * inv_t + to.r() * t,
            from.g() * inv_t + to.g() * t,
            from.b() * inv_t + to.b() * t,
            from.a() * inv_t + to.a() * t
        )
    }
}

impl InterpolateColors for Color32 {
    fn interpolate_to(&self, to: Color32, t: f32) -> Color32 {
        let from = Rgba::from(*self);
        let to = Rgba::from(to);
        from.interpolate_to(to, t).into()
    }
}