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

use std::convert::From;

pub use nih_plug_egui::egui::ecolor::{Color32, Hsva, HsvaGamma, Rgba};


/// A mathmatical palette to pick nice colors using the power of math
#[derive(Clone)]
pub struct Palette {
    hue_count: u32,
    hue_shift: f32,
    saturation: f32,
    shade_count: u32,
    alt_hue_step: f32,
    dark_hue_shift: f32,
    dark_desaturation: f32,
    black_level: f32,
    white_level: f32,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            hue_count: 12,
            hue_shift: -0.05,
            saturation: 0.85,
            shade_count: 6,
            alt_hue_step: 0.5,
            dark_hue_shift: 0.1,
            dark_desaturation: -0.5,
            black_level: 0.0,
            white_level: 1.0,
        }
    }
}

impl Palette {

    /// Returns a new Palette with a set number of base colors
    pub fn new(hue_count: u32) -> Self {
        Self {
            hue_count,
            ..Default::default()
        }
    }

    /// Returns an alternate Palette shifted by a number of hues
    pub fn alternate(&self, hue_offset: f32) -> Self {
        self.clone()
            .shift(self.hue_shift + (hue_offset / self.shade_count as f32))
    }

    /// Returns a modified Palette with a set number of shades
    pub fn shades(mut self, shade_count: u32) -> Self {
        self.shade_count = shade_count.max(2);
        self
    }

    /// Returns a modified Palette with a the hue shifted
    pub fn shift(mut self, shift: f32) -> Self {
        self.hue_shift = shift;
        self
    }

    /// Returns a modified Palette with a specified hue shift for alternate color steps
    pub fn alt_hue_step(mut self, alt_hue_step: f32) -> Self {
        self.alt_hue_step = alt_hue_step;
        self
    }

    /// Returns a modified Palette with a specified hue shift for darkened colors
    pub fn dark_shift(mut self, shift: f32) -> Self {
        self.dark_hue_shift = shift;
        self
    }

    /// Returns a modified Palette with a specified desaturation for darkened colors
    pub fn dark_desaturation(mut self, desaturation: f32) -> Self {
        self.dark_desaturation = desaturation;
        self
    }

    /// Returns a modified Palette withe a specified saturation
    pub fn saturation(mut self, saturation: f32) -> Self {
        self.saturation = saturation;
        self
    }

    /// Returns a modified Palette with a specified maximum brightness
    pub fn white_level(mut self, white_level: f32) -> Self {
        self.white_level = white_level;
        self
    }

    /// Returns a modified Palette with a specified minimum brightness
    pub fn black_level(mut self, black_level: f32) -> Self {
        self.black_level = black_level;
        self
    }

    /// Returns a new PaletteColor by hue index
    pub fn color(&self, hue_index: u32) -> PaletteColor {
        self.color_smooth(hue_index as f32)
    }

    /// Returns a new PaletteColor by hue index, smoothly interpolated
    pub fn color_smooth(&self, hue_index: f32) -> PaletteColor {
        let hue = (hue_index / (self.hue_count as f32)).rem_euclid(1.0);
        PaletteColor::color(hue)
            .black_level(self.black_level)
            .white_level(self.white_level)
            .shift(self.hue_shift)
            .saturation(self.saturation)
            .dark_shift(self.dark_hue_shift)
            .dark_desaturation(self.dark_desaturation)
            .set_shade_count(self.shade_count)
    }

    /// Returns a new alternate PaletteColor by hue index and alt index
    pub fn color_alt(&self, hue_index: u32, alt_hue_index: i32) -> PaletteColor {
        self.color_smooth(hue_index as f32 + alt_hue_index as f32 * self.alt_hue_step)
    }

    /// Returns a white PaletteColor
    pub fn white(&self) -> PaletteColor {
        PaletteColor::white()
            .black_level(self.black_level)
            .white_level(self.white_level)
            .dark_shift(self.dark_hue_shift)
            .dark_desaturation(self.dark_desaturation)
            .desaturate()
            .set_shade_count(self.shade_count)
    }

    /// Returns a grey PaletteColor
    pub fn grey(&self) -> PaletteColor {
        // Used to get a light grey balanced with other colors
        self.color(0).desaturate()
    }

    /// Returns a black PaletteColor
    pub fn black(&self) -> PaletteColor {
        // Used to get a light grey balanced with other colors
        self.color(0).desaturate().brightness(0.0)
    }

    /// Returns an alpha-blended black PaletteColor for layering panels
    pub fn darker(&self) -> PaletteColor {
        // Used to layer panels that are each slightly darker than the one below
        self.black().alpha(0.15)
    }

    /// Returns an alpha-blended white PaletteColor for layering panels
    pub fn lighter(&self) -> PaletteColor {
        // Used to layer panels that are each slightly darker than the one below
        self.white().alpha(0.15)
    }

    /// Returns an increasingly opaque alpha-blended black PaletteColor for layering panels
    pub fn dark_layer(&self, layer: usize) -> PaletteColor {
        // Used to layer panels that are each slightly darker than the one below
        self.black().alpha((0.15 * layer as f32).min(1.0))
    }

    /// Returns an increasingly opaque alpha-blended white PaletteColor for layering panels
    pub fn light_layer(&self, layer: usize) -> PaletteColor {
        // Used to layer panels that are each slightly lighter than the one below
        self.white().alpha((0.15 * layer as f32).min(1.0))
    }
}

/// A color that can be modified and resolved into various concrete formats
#[derive(Clone)]
pub struct PaletteColor {
    /// Base color value
    value: HsvaGamma,

    /// Amount of hue shift to apply when darkening
    dark_hue_shift: f32,

    /// Amount of desaturation to apply when darkening
    dark_desaturation: f32,

    /// Logical shade count
    shade_count: u32,

    /// Darkest black allowed
    black_level: f32,

    /// Brightest white allowed
    white_level: f32,
}

impl PaletteColor {
    fn color(hue: f32) -> Self {
        Self {
            value: HsvaGamma {
                h: hue,
                s: 1.0,
                v: 1.0,
                a: 1.0,
            },
            dark_hue_shift: 0.0,
            dark_desaturation: 0.0,
            shade_count: 5,
            black_level: 0.0,
            white_level: 1.0,
        }
    }

    fn white() -> Self {
        Self {
            value: HsvaGamma {
                h: 0.0,
                s: 0.0,
                v: 1.0,
                a: 1.0,
            },
            dark_hue_shift: 0.0,
            dark_desaturation: 0.0,
            shade_count: 5,
            black_level: 0.0,
            white_level: 1.0,
        }
    }

    fn set_shade_count(mut self, shade_count: u32) -> Self {
        self.shade_count = shade_count.max(2);
        self
    }

    fn dark_shift(mut self, shift: f32) -> Self {
        self.dark_hue_shift = shift;
        self
    }

    fn dark_desaturation(mut self, shift: f32) -> Self {
        self.dark_desaturation = shift;
        self
    }
    fn black_level(mut self, black_level: f32) -> Self {
        self.black_level = black_level;
        self
    }
    fn white_level(mut self, white_level: f32) -> Self {
        self.white_level = white_level;
        self
    }

    /// Returns a modified PaletteColor with a specified hue
    pub fn set_hue(mut self, hue: f32) -> Self {
        self.value.h = hue;
        self
    }

    /// Returns a modified PaletteColor with the hue shifted
    pub fn hue(mut self, shift: f32) -> Self {
        self.value.h = (self.value.h + shift).rem_euclid(1.0);
        self
    }

    /// Returns a modified PaletteColor with the hue shifted. Alias for hue()
    pub fn shift(self, shift: f32) -> Self {
        // Alias
        self.hue(shift)
    }

    /// Returns a the compliment of a PaletteColor
    pub fn complimentary(self) -> Self {
        self.hue(0.5)
    }

    /// Returns a modified PaletteColor with the saturation set absolutely
    pub fn set_saturation(mut self, saturation: f32) -> Self {
        self.value.s = saturation;
        self
    }

    /// Returns a modified PaletteColor with the saturation adjusted relatively
    pub fn saturation(mut self, saturation: f32) -> Self {
        self.value.s = (self.value.s * saturation).clamp(0.0, 1.0);
        self
    }

    /// Returns a modified PaletteColor with the color fully saturation
    pub fn saturate(mut self) -> Self {
        self.value.s = 1.0;
        self
    }

    /// Returns a modified PaletteColor with the color fully desaturated
    pub fn desaturate(mut self) -> Self {
        self.value.v *= 1.0 - self.value.s * 0.1; //darken up to 10% to fit with colors
        self.value.s = 0.0;
        self
    }

    /// Returns a modified PaletteColor with the brightness set absolutely
    pub fn set_brightness(mut self, brightness: f32) -> Self {
        let shift_amount = self.value.v - brightness;
        let hue_shift = shift_amount * self.dark_hue_shift;
        let saturation_shift = shift_amount * -self.dark_desaturation;
        self.value.v = brightness;
        self.value.s = (self.value.s + saturation_shift).clamp(0.0, 1.0);
        self.shift(hue_shift)
    }

    /// Returns a modified PaletteColor with the brightness adjusted relatively
    pub fn brightness(self, scale: f32) -> Self {
        let v = self.value.v;
        self.set_brightness((v * scale).clamp(0.0, 1.0))
    }

    /// Returns a modified PaletteColor representing the specified shade color
    pub fn shade(self, shade: u32) -> Self {
        let shade_count = self.shade_count;
        let shade = shade.clamp(0, shade_count - 1); // shade_count is black, so not valid
        self.set_brightness(1.0 - shade as f32 / shade_count as f32)
    }

    /// Returns a modified PaletteColor with the alpha set absolutely
    pub fn set_alpha(mut self, alpha: f32) -> Self {
        self.value.a = alpha;
        self
    }

    /// Returns a modified PaletteColor with the alpha adjusted relatively
    pub fn alpha(mut self, alpha: f32) -> Self {
        self.value.a = (self.value.a * alpha).clamp(0.0, 1.0);
        self
    }

    /// Returns the color converted to a HsvaGamma
    fn resolve(&self) -> HsvaGamma {
        let mut result = self.value;
        result.v =
            (result.v * (self.white_level - self.black_level) + self.black_level).clamp(0.0, 1.0);
        result
    }

    /// Returns the color converted to a Color32
    pub fn to_color32(&self) -> Color32 {
        self.resolve().into()
    }

    /// Returns the color converted to a Rgba
    pub fn to_rgba(&self) -> Rgba {
        self.resolve().into()
    }

    /// Returns the color converted to a Hsva
    pub fn to_hsva(&self) -> Hsva {
        self.resolve().into()
    }

    /// Returns the color converted to a HsvaGamma
    pub fn to_hsvagamma(&self) -> HsvaGamma {
        self.resolve()
    }
}

impl From<PaletteColor> for Color32 {
    fn from(color_picker: PaletteColor) -> Self {
        color_picker.to_color32()
    }
}
impl From<PaletteColor> for Rgba {
    fn from(color_picker: PaletteColor) -> Self {
        color_picker.to_rgba()
    }
}
impl From<PaletteColor> for Hsva {
    fn from(color_picker: PaletteColor) -> Self {
        color_picker.to_hsva()
    }
}
impl From<PaletteColor> for HsvaGamma {
    fn from(color_picker: PaletteColor) -> Self {
        color_picker.to_hsvagamma()
    }
}