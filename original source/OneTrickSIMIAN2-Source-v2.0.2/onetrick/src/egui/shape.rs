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

use std::f32::consts::TAU;

use nih_plug_egui::egui::{
    emath::{Pos2, Vec2},
    epaint::PathShape,
    Shape, Stroke,
};

/// Extends Shapes with an arc_stroke function
pub trait ShapeEx {
    fn arc_stroke(
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        stroke: impl Into<Stroke>,
    ) -> Shape;
}

impl ShapeEx for Shape {
    /// Draws an arc stroke
    fn arc_stroke(
        center: Pos2,
        radius: f32,
        start_angle: f32,
        end_angle: f32,
        stroke: impl Into<Stroke>,
    ) -> Shape {
        //self.circle_stroke(center, radius, stroke);
        let angle_range_abs = (end_angle - start_angle).abs();
        if angle_range_abs < (TAU / 360.0 * 0.5) {
            // Less than 1/2 a degree should be ignored
            return Shape::Noop;
        }
        let steps = 2 + (angle_range_abs * 6.0) as usize;
        let mut path = Vec::with_capacity(steps);
        let delta_angle = end_angle - start_angle;
        let step_ratio = 1.0 / ((steps - 1) as f32);
        for i in 0..steps {
            let t = (i as f32) * step_ratio;
            let a = start_angle + delta_angle * t;
            let v = Vec2::new(a.cos(), -a.sin()); //Vec2::angled(a);
            let vert = center + v * radius;
            path.push(vert);
        }
        Shape::Path(PathShape::line(path, stroke))
    }
}
