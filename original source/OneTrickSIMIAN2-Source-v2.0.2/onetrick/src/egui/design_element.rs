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
    epaint::{
        PathShape,
    },
    Color32,
    Painter,
    Pos2,
    Rect,
    Rounding,
    Vec2,
    Align2,
    FontId,
    FontFamily,
};

use crate::egui::Dingbat;
use crate::egui::icons::IconDrawer;

#[derive(Clone)]
pub struct MidiKeyboardStyle {
    pub white_color: Color32,
    pub black_color: Color32,
    pub active_color: Color32,
}

#[derive(Clone)]
pub struct VuMeterStyle {
    pub dot_count: u32,
    pub spacing: f32,
    pub padding: f32,
    pub bg_color: Color32,
    pub unlit_color: Color32,
    pub lit_color: Color32,
    pub warn_color: Color32,
    pub peak_color: Color32,
}

/// Design element shapes for Painter
pub trait DesignElementDrawer {
    /// Draws an 80s style line fade with increasing line size
    fn fade_80s(&self, rect: Rect, count: u32, color: Color32);

    /// Draws an 80s style line fade with even spacing
    fn fade_80s_uniform(&self, rect: Rect, spacing: f32, color: Color32);

    /// Draws a Parallelogram
    fn parallelogram(&self, rect: Rect, slope: f32, color: Color32);

    /// Draws a number of Parallelograms
    fn parallelograms(&self, rect: Rect, slope: f32, colors: &[Color32]);

    /// Draws a OneTrick logo
    fn one_trick_logo(&self, title: &str, pos: Pos2, height: f32, ot_color: Color32, logo_color: Color32);

    /// Draws a MIDI Keyboard
    fn midi_keyboard(&self, rect: Rect, style: MidiKeyboardStyle, lowest_key: u8, highest_key: u8, active_keys: &[u8]);

    /// Draws a VU Meter
    fn vu_meter_mono(&self, rect: Rect, level: f32, style: &VuMeterStyle);

    /// Draws a VU Meter
    fn vu_meter_stereo(&self, rect: Rect, level_left: f32, level_right: f32, style: &VuMeterStyle);
}

impl DesignElementDrawer for Painter {

    /// Draws faded 80s style horizontal lines
    fn fade_80s(&self, rect: Rect, count: u32, color: Color32) {
        //self.rect_filled(rect, Rounding::ZERO, Color32::from_white_alpha(64));
        let spacing = rect.height() / (count as f32);
        let smallest = 1.0 / (count as f32);
        let start_y = rect.top() + spacing * 0.5 - smallest;
        let start_x = rect.left();
        let width = rect.width();
        for i in 0..count {
            let height = ((i + 1) as f32) / (count as f32) * spacing;
            let line_rect = Rect::from_min_size(
                Pos2::new(start_x, start_y + spacing * (i as f32)),
                Vec2::new(width, height),
            );
            self.rect_filled(line_rect, Rounding::ZERO, color);
        }
    }

    /// Draws faded 80s style uniform horizontal lines
    fn fade_80s_uniform(&self, rect: Rect, spacing: f32, color: Color32) {
        //self.rect_filled(rect, Rounding::ZERO, Color32::from_white_alpha(64));
        let start_y = rect.top();
        let count = (rect.height() / spacing * 0.5) as u32;
        let start_x = rect.left();
        let width = rect.width();
        for i in 0..count {
            let line_rect = Rect::from_min_size(
                Pos2::new(start_x, start_y + spacing * 2.0 * (i as f32)),
                Vec2::new(width, spacing),
            );
            let a = (((i + 1) as f32 / count as f32) * color.a() as f32) as u8;
            let color = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), a);
            self.rect_filled(line_rect, Rounding::ZERO, color);
        }
    }
    
    /// Draws a parallelogram
    fn parallelogram(&self, rect: Rect, slope: f32, color: Color32) {
        let shift_x = rect.height() * slope;
        self.add(PathShape {
            points: vec![
                Pos2::new(rect.left()+shift_x.max(0.0), rect.top()),
                Pos2::new(rect.right()+shift_x.min(0.0), rect.top()),
                Pos2::new(rect.right()-shift_x.max(0.0), rect.bottom()),
                Pos2::new(rect.left()-shift_x.min(0.0), rect.bottom()),
            ],
            closed: true,
            fill: color,
            stroke: Default::default(),
        });
    }

    /// Draws some parallelograms
    fn parallelograms(&self, rect: Rect, slope: f32, colors: &[Color32]) {
        let count = colors.len();
        let stripe_width_padding = (rect.height() * slope).abs();
        let mut rect = rect;
        rect.set_right(rect.right()-stripe_width_padding);
        let stripe_width = rect.width() / count as f32 + stripe_width_padding;
        let stride = rect.width() / count as f32;
        for (index, color) in colors.iter().enumerate() {
            let x = rect.left() + stride * index as f32;
            let is_last = index == count-1;
            let overlap = if is_last {0.0} else {1.0};
            let size = Vec2::new(stripe_width+overlap, rect.height());
            let rect = Rect::from_min_size(Pos2::new(x, rect.top()), size);
            self.parallelogram(rect, slope, *color);
        }
    }

    /// Draws an OT PLUGIN logo
    fn one_trick_logo(&self, title: &str, pos: Pos2, height: f32, ot_color: Color32, logo_color: Color32) {
        self.dingbat(
            pos,
            Align2::LEFT_CENTER,
            Dingbat::OneTrickIcon,
            height * 0.9615,
            ot_color,
        );
        self.text(
            pos + Vec2::new(height*1.05769, height*0.03846),
            Align2::LEFT_CENTER,
            title,
            FontId::new(height, FontFamily::Name("Title".into())),
            logo_color,
        );
    }

    /// Draws a MIDI Keyboard
    fn midi_keyboard(&self, rect: Rect, style: MidiKeyboardStyle, lowest_key: u8, highest_key: u8, active_keys: &[u8]) {
        // Step 1) Build some metadata for a single octave
        let white_keys_from_c = [true,false,true,false,true,true,false,true,false,true,false,true];
        let white_width_ratio: f32 = 12.0/7.0;
        let mut key_shapes: [Rect; 12]=[Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(0.0, 0.0)); 12];
        {
            let mut white_x: f32 = 0.0;
            for key in 0..12 {
                let base_x = key as f32;
                let is_white = white_keys_from_c[key];
                let x = if is_white {white_x - base_x} else {0.0};
                let width = if is_white {white_width_ratio} else {1.0};
                let height = if is_white {1.0} else {6.3/10.0};
                key_shapes[key] = Rect::from_min_size(Pos2::new(x, 0.0), Vec2::new(width, height));
                if is_white {
                    white_x += white_width_ratio;
                }
            }
        }

        // Step 2) Calculate some base values
        let lowest_key = lowest_key as usize;
        let highest_key = highest_key as usize;

        let lowest_key_shape = key_shapes[lowest_key%12];
        let highest_key_shape = key_shapes[highest_key%12];

        let key_count = highest_key - lowest_key + 1;
        let key_base_width = rect.width() / key_count as f32;
        let key_base_height = rect.height();
        
        // Step 3) Tweak the rect to compensate for start/end offsets
        let left_adjustment = -lowest_key_shape.left()*key_base_width;
        let right_adjustment = -(highest_key_shape.right()-1.0)*key_base_width;
        let mut rect = rect;
        rect.set_left(rect.left()+left_adjustment);
        rect.set_right(rect.right()+right_adjustment);
        let key_base_width = rect.width() / key_count as f32; // Recalculate after adjustments
        
        // Step 4) Draw the keys
        let key_base_rect_template = Rect::from_min_size(rect.left_top(), Vec2::new(key_base_width, key_base_height));
        for pass in 0..2 { // White, then Black keys
            for (index, key) in (lowest_key ..= highest_key).enumerate() {
                let is_white = white_keys_from_c[key%12];
                let is_active = !active_keys.is_empty() && active_keys.contains(&(key as u8));
                if is_white == (pass == 0) {
                    let mut key_rect = key_base_rect_template.translate(Vec2::new(index as f32 * key_base_width, 0.0));
                    let key_shape = key_shapes[key%12];
                    key_rect = key_rect.translate(Vec2::new(key_base_width*key_shape.left(), key_base_height*key_shape.top()));
                    key_rect.set_width(key_base_width*key_shape.width());
                    key_rect.set_height(key_base_height*key_shape.height());
                    if is_white {
                        key_rect = key_rect.shrink2(Vec2::new(0.5, 0.0));
                    }

                    let color = if is_white {style.white_color} else {style.black_color};
                    self.rect_filled(key_rect, Rounding::ZERO, color);
                    if is_active {
                        let key_rect = if is_white {key_rect} else {key_rect.shrink2(Vec2::new(1.0, 0.5)).translate(Vec2::new(0.0, -0.5))};
                        self.rect_filled(key_rect, Rounding::ZERO, style.active_color);
                    }
                }
            }
        }
    }

    /// Draws a VU Meter
    fn vu_meter_mono(&self, rect: Rect, level: f32, style: &VuMeterStyle) {
        // Draw the background
        if style.bg_color.a() > 0 { 
            self.rect_filled(rect, Rounding::same(style.padding), style.bg_color);
        }
        let is_vertical = rect.height() > rect.width();
        let rect = rect.shrink(style.padding);
        let length = if is_vertical {rect.height()} else {rect.width()};
        let width = if is_vertical {rect.width()} else {rect.height()};
        let tick_length = length / style.dot_count as f32 - style.spacing;
        let tick_size = if is_vertical { Vec2::new(width, tick_length) } else { Vec2::new(tick_length, width ) };
        let tick_rect = Rect::from_min_size(rect.min, tick_size);
        let adjusted_spacing = style.spacing + style.spacing / (style.dot_count - 1) as f32;
        let tick_spacing = if is_vertical { Vec2::new(0.0, tick_length+adjusted_spacing) } else { Vec2::new(tick_length+adjusted_spacing, 0.0) };
        let lit_dot_count = (style.dot_count as f32 * level).round() as u32;
        // Draw the dots
        for i in 0..style.dot_count {
            let tick_rect = tick_rect.translate(tick_spacing * i as f32);
            let index = if is_vertical {style.dot_count - i - 1} else {i};
            let ratio = index as f32 / style.dot_count as f32;
            let dot_color = if ratio > 0.89 {style.peak_color} else if ratio > 0.69 {style.warn_color} else {style.lit_color};
            let color = if index < lit_dot_count {dot_color} else {style.unlit_color};
            //self.circle_filled(pos, radius, color);
            self.rect_filled(tick_rect, Rounding::same(1.0), color);
        }
    }

    fn vu_meter_stereo(&self, rect: Rect, level_left: f32, level_right: f32, style: &VuMeterStyle) {
        if style.bg_color.a() > 0 { 
            self.rect_filled(rect, Rounding::same(style.padding), style.bg_color);
        }
        let mut style = style.clone();
        style.bg_color = Color32::TRANSPARENT;

        let is_vertical = rect.height() > rect.width();
        let mut rect_left = rect;
        let padding_half = style.padding / 2.0;
        if is_vertical {
            rect_left.set_width(rect.width() / 2.0+padding_half);
        } else {
            rect_left.set_height(rect.height() / 2.0+padding_half);
        }
        self.vu_meter_mono(rect_left, level_left, &style);

        let mut rect_right = rect_left;
        if is_vertical {
            rect_right = rect_right.translate(Vec2::new(rect.width()/2.0-padding_half, 0.0));
        } else {
            rect_right = rect_right.translate(Vec2::new(0.0, rect.height()/2.0-padding_half));
        }
        self.vu_meter_mono(rect_right, level_right, &style);
    }


}
