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
    Rect,
    Response,
    Rounding,
    Sense,
    Ui,
    Vec2,
    Widget,
    /*
    Shape,
    epaint::{
        Shadow,
    },
    */
};

use super::icons::{Dingbat, IconDrawer};
use crate::color::InterpolateColors;


/// Style info for an IconTabs
#[derive(Clone)]
pub struct IconTabsStyle {
    /// Color of the backgrond panel
    pub panel_color: Color32,

    /// Rounding of the background panel
    pub panel_rounding: Rounding,

    /// Color of a tab icon
    pub color: Color32,
    
    /// Color of a tab icon when active
    pub color_active: Color32,

    /// Color of a tab background when active
    pub tab_color_active: Color32,

    /// Rounding of a tab background
    pub tab_rounding: Rounding,

    /// Amount to expand the tab when hovering
    pub tab_hover_resize: Vec2,

    /// XY Padding of the Icon
    pub icon_padding: f32,
}
impl Default for IconTabsStyle {
    fn default() -> Self {
        Self {
            panel_color: Color32::LIGHT_GRAY,
            panel_rounding: Rounding::ZERO,
            color: Color32::DARK_GRAY,
            color_active: Color32::DARK_RED,
            tab_color_active: Color32::WHITE,
            tab_rounding: Rounding::same(5.0),
            icon_padding: 10.0,
            tab_hover_resize: Vec2::new(0.0, 2.0),
        }
    }
}

/// Style info for a single Tab
#[derive(Clone)]
pub struct IconTabStyle {

    /// Dingbat Icon to draw on the tab
    pub dingbat: Dingbat,

    /// Icon color
    pub color: Color32,

    /// Amount to zoom when hovering
    pub zoom: f32,
}
impl Default for IconTabStyle {
    fn default() -> Self {
        Self {
            dingbat: Dingbat::Info,
            color: Color32::TRANSPARENT,
            zoom: 1.0,
        }
    }
}
impl IconTabStyle {

    /// Set the Icon to a Dingbat
    pub fn with_dingbat(mut self, dingbat: Dingbat) -> Self {
        self.dingbat = dingbat;
        self
    }

    /// set the Icon color
    pub fn with_color(mut self, color: Color32) -> Self {
        self.color = color;
        self
    }

    /// set the amount to zoom when hovering
    pub fn with_zoom(mut self, zoom: f32) -> Self {
        self.zoom = zoom;
        self
    }
}

/// An Icon Tabs UI Element
pub struct IconTabs<'a> {
    active_index: &'a mut u32,
    style: IconTabsStyle,
    tabs: Vec<IconTabStyle>,
}

#[allow(unused)]
impl<'a> IconTabs<'a> {
    /// Returns a new IconTabs
    pub fn new(active_index: &'a mut u32, style: IconTabsStyle, tabs: Vec<IconTabStyle>) -> Self {
        Self {
            active_index,
            style,
            tabs,
        }
    }
}

impl<'a> Widget for IconTabs<'a> {
    fn ui(self, ui: &mut Ui) -> Response {
        let tab_count = self.tabs.len();
        let tab_width = ui.available_width() / tab_count as f32;
        let tab_height = 64.0;
        //let tab_size = Vec2::new(tab_width, tab_height);
        let topleft = ui.next_widget_position();
        let rect = Rect::from_min_size(topleft, Vec2::new(ui.available_width(), tab_height));
        ui.allocate_ui_at_rect(rect, |ui| {
            let color = self.style.panel_color;
            if color.a() > 0 {
                ui.painter()
                    .rect_filled(rect, self.style.panel_rounding, color);
            }
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing = vec2(0.0, 0.0);
                for (index, tab) in self.tabs.iter().enumerate() {
                    let index = index as u32;
                    let is_active = index == *self.active_index;
                    let (rect, response) =
                        ui.allocate_exact_size(vec2(tab_width, tab_height), Sense::click());

                    if response.clicked() {
                        response.request_focus();
                    }
                    
                    let is_active_or_hovered = is_active || response.hovered();
                    let hover_amount = ui.ctx().animate_bool_with_time(
                        response.id,//.with("hover"),
                        response.hovered(),
                        0.100,
                    );

                    // Draw Tab
                    let color = if is_active_or_hovered {
                        self.style.tab_color_active
                    } else if hover_amount > 0.0 {
                        Color32::TRANSPARENT.interpolate_to(self.style.tab_color_active, hover_amount)
                    } else {
                        Color32::TRANSPARENT
                    };

                    let expanded_rect = rect.expand2(self.style.tab_hover_resize * hover_amount);

                    /*
                    if is_active {
                        let shadow = Shadow{extrusion: 6.0, color: Color32::from_black_alpha(32)};
                        let shadow = shadow.tessellate(expanded_rect, self.style.panel_rounding);
                        let shadow = Shape::Mesh(shadow);
                        ui.painter().add(shadow);
                    }
                    */

                    if color.a() > 0 {
                        ui.painter().rect_filled(
                            expanded_rect,
                            self.style.tab_rounding,
                            color,
                        );
                    }

                    // Draw Icon
                    let color = if is_active_or_hovered {
                        self.style.color_active
                    } else {
                        self.style.color
                    };
                    ui.painter().dingbat(
                        rect.center(),
                        Align2::CENTER_CENTER,
                        tab.dingbat,
                        (tab_height - self.style.icon_padding * 2.0) * tab.zoom,
                        color,
                    );

                    if response.has_focus() {
                        let focus_stroke = ui.style().visuals.selection.stroke;
                        if !focus_stroke.is_empty() {
                            ui.painter().rect_stroke(expanded_rect, self.style.tab_rounding, focus_stroke);
                        }
                    }

                    if response.clicked() {
                        *self.active_index = index;
                    }
                }
            });
        })
        .response
    }
}
