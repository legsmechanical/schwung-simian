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
    //pos2,
    vec2, Vec2,
    Pos2,
    Rect,
    Id,
    Color32,
    Response,
    Rounding,
    Ui,
    Frame,
    TextEdit,
    ScrollArea,
    //Align,
    Align2,
    Event,
    EventFilter,
    //Key,
    Area,
    Order,
    Margin,
};

use std::time::{
    Instant,
    Duration,
};

use nih_plug::prelude::{ParamSetter};
use crate::params::OneTrickPluginParams;
use crate::preset::*;
use crate::preset_manager::PresetManager;
use super::icons:: {
    Dingbat,
    IconDrawer,
};
use super::icon_button::{IconButton, IconButtonStyle};
use super::label::{Label, LabelStyle};

use open;


/// Style for a PresetList Widget
#[derive(Clone)]
pub struct PresetListStyle {
    /// Width of the Widget
    pub width: Option<f32>,

    /// Height of the Widget
    pub height: Option<f32>,

    /// Background color of the Frame
    pub bg_color: Option<Color32>,

    /// Foreground color of text and icons
    pub color: Color32,

    /// Style for Icon Buttons
    pub icon_style: IconButtonStyle,

    /// Style for preset Labels
    pub label_style: LabelStyle,

    /// Color for highlighted Labels
    pub highlight_color: Color32,

    /// Background color for highlighted labels
    pub highlight_bg_color: Color32,

    /// Search field background color
    pub search_bg_color: Option<Color32>,

    /// Search field text color
    pub search_color: Option<Color32>,

    /// Popup info panel normalized offset
    pub popup_offset_normalized: Vec2,

    /// Popup info panel height
    pub popup_height: f32,
}

impl Default for PresetListStyle {
    fn default() -> Self {
        Self {
            width: None,
            height: None,
            bg_color: None,
            color: Color32::WHITE,
            icon_style: IconButtonStyle::default(),
            label_style: LabelStyle::default(),
            highlight_color: Color32::YELLOW,
            highlight_bg_color: Color32::BLACK,
            search_bg_color: None,
            search_color: None,
            popup_offset_normalized: vec2(0.0, 0.0),
            popup_height: 125.0,
        }
    }
}

/// Widget to display, search, select and edit Presets in a list
#[derive(Default)]
pub struct PresetList {
    style: PresetListStyle,
}
#[allow(unused)]
impl PresetList {
    /// Returns a new PresetList Widget
    pub fn new() -> Self {
        Self {
            style: PresetListStyle::default(),
        }
    }

    /// Sets the style
    pub fn with_style(mut self, style: &PresetListStyle) -> Self {
        self.style = style.clone();
        self
    }

    /// Sets the Frame's background color
    pub fn with_bg_color(mut self, color: Color32) -> Self {
        self.style.bg_color = Some(color);
        self
    }

    /// Sets the background color of highlighted Labels
    pub fn with_highlight_bg_color(mut self, color: Color32) -> Self {
        self.style.highlight_bg_color = color;
        self
    }

    /// Sets the text color of highlighted Labels
    pub fn with_highlight_color(mut self, color: Color32) -> Self {
        self.style.highlight_color = color;
        self
    }

    /// Sets the normalized popup info panel offset
    pub fn with_popup_offset_normalized(mut self, offset: Vec2) -> Self {
        self.style.popup_offset_normalized = offset;
        self
    }

    /// Sets the popup info panel height
    pub fn with_popup_height(mut self, height: f32) -> Self {
        self.style.popup_height = height;
        self
    }
    /// Sets the current text for editing tags
    fn update_edit_tags(&self, ui: &Ui, preset_manager: &mut PresetManager, tags_text_id: Id) {
        let tags_string = preset_manager.get_active().get_tags_string_filtered();
        ui.data_mut(|r|{*(r.get_temp_mut_or::<String>(tags_text_id, "".to_string())) = tags_string.to_string();});
    }

    /// Updates the filtered search results
    fn update_search(&self, ui: &Ui, preset_manager: &mut PresetManager, search_text_id: Id, search_results_id: Id) {
        //nih_log!("Updating Search");
        let search_text: String = ui.data(|r|r.get_temp::<String>(search_text_id).unwrap_or("".to_string()).to_string());
        let search_results = preset_manager.search(&search_text);
        ui.data_mut(|r|{*(r.get_temp_mut_or::<Vec<usize>>(search_results_id, vec!())) = search_results;});
    }

    /// Returns the filtered search results as a list of indices
    fn get_search_results(&self, ui: &Ui, preset_manager: &PresetManager, search_results_id: Id) -> Vec<usize> {
        ui.data(|r|r.get_temp::<Vec<usize>>(search_results_id)
            .unwrap_or((0..preset_manager.get_preset_count()).collect()) // Default to a list of all presets
        )
    }

    /// Returns true if Preset has been changed too rapidly and should be throttled
    fn throttle_preset_change(&self, ui: &Ui, preset_changed_timestamp: Id) -> bool {
        let now = Instant::now();
        let last_timestamp = ui.data(|r| r.get_temp::<Instant>(preset_changed_timestamp).unwrap_or(now - Duration::from_secs(1)));
        let result = now.duration_since(last_timestamp).as_millis() < 1000/8;
        if !result {
            ui.data_mut(|r| {
                *(r.get_temp_mut_or::<Instant>(preset_changed_timestamp, now)) = now;
            });
        }
        result
    }

    /// Shows the Widget's UI
    pub fn show(self, ui: &mut Ui, preset_manager: &mut PresetManager, params: &OneTrickPluginParams, setter: &ParamSetter) -> Response {

        // TODO: Switch to using Widget State?
        let is_saving_id = ui.id().with("saving");
        let preset_changed_timestamp = ui.id().with("preset_changed_timestamp");
        let preset_focused_id = ui.id().with("preset_focused");
        let refresh_flash_id = ui.id().with("refresh_flash");
        let tags_text_id = ui.id().with("tags_text");
        let search_results_id = ui.id().with("search_results");
        let search_text_id: Id = ui.id().with("search_text");
        let mut is_refreshing = false;

        let mut frame = Frame::none()
            .outer_margin(Margin::same(0.0))
            .inner_margin(Margin::same(5.0))
            .rounding(Rounding::same(10.0));
            if let Some(color) = self.style.bg_color {
                if color.a() > 0 {
                    frame = frame.fill(color);
                }
            }
        //let response = ui.allocate_ui(vec2(width, height), |ui| {

        frame.show(ui, |ui| {
            let width = self.style.width.unwrap_or_else(|| ui.available_width());
            let height = self.style.width.unwrap_or_else(|| ui.available_height());
            //let popup_rect = Rect::from_min_size(ui.next_widget_position()+vec2(-width/2.0+6.0, -height/2.0+67.0), vec2(width, height));//.expand2(vec2(10.0, 10.0));
            // Widgets are currently centered on x
            let save_rect = Rect::from_min_size(ui.next_widget_position()+vec2(-width/2.0, 0.0), vec2(width, height));
            let mut popup_rect = save_rect.translate(save_rect.size() * self.style.popup_offset_normalized);
            popup_rect.set_height(self.style.popup_height);

            //ui.painter().rect_filled(popup_rect, Rounding::same(5.0), Color32::RED);

            let mut is_saving = ui.data(|r| r.get_temp::<bool>(is_saving_id).unwrap_or(false));
            let mut on_save_open = false;

            if true {
                //ui.set_width(width);
                //ui.set_height(height);

                // Might need to wait for the updated egui tooltip delay feature...
                //ui.style_mut().interaction.show_tooltips_only_when_still = true;

                let label_style = self.style.label_style.clone();
                let mut label_style_selected = label_style.clone();
                label_style_selected.color = self.style.highlight_color;
                label_style_selected.bg_color = self.style.highlight_bg_color;

                let mut updown: i32 = 0;
                let mut changed_selection: bool = false;

                // if !is_saving {
                //     ui.input(|r| {
                //         for event in &r.filtered_events(&EventFilter{vertical_arrows:true, ..Default::default()}) {
                //             match event {
                //                 Event::Key{key, pressed, modifiers, repeat, physical_key} => {
                //                     if *pressed {
                //                         updown = match key {
                //                             Key::ArrowUp => -1,
                //                             Key::ArrowDown => 1,
                //                             _ => 0,
                //                         }
                //                     }
                //                 },
                //                 //Event::Text(t) => { nih_log!("Text = {:?}", t) }
                //                 _ => {}
                //             }
                //         }
                //     });
                // }

                let mut preset_has_focus = ui.data(|r| r.get_temp::<bool>(preset_focused_id).unwrap_or(false));

                if updown != 0 && preset_has_focus {
                    if let Some(index) = preset_manager.get_active_index() {
                        let search_results = self.get_search_results(ui, preset_manager, search_results_id);
                        // Map the preset_index to a search_index
                        if let Some(index) = search_results.iter().position(|i|*i==index) {
                            let new_index = (index as i32 + updown).min(search_results.len() as i32 - 1).max(0) as usize;
                            if new_index != index && !self.throttle_preset_change(ui, preset_changed_timestamp) {
                                let new_index = search_results[new_index]; // Map search_index to preset_index
                                preset_manager.activate_preset(new_index, params, setter);
                                self.update_edit_tags(ui, preset_manager, tags_text_id);
                                changed_selection = true;
                            }                    
                        }
                    }
                }


                let mut active_index = preset_manager.get_active_index();
                let mut selected_index: Option<usize> = None;
                let preset_count = preset_manager.get_preset_count();

                let max_visible = ((height - IconButtonStyle::default().size)/label_style.bg_height) as usize;
                //let scroll_pos_id: Id = ui.id().with("scroll_pos");
                ui.horizontal(|ui|{
                    /*
                    if ui.add(IconButton::new(Dingbat::Up)
                        .with_no_bg()
                        .with_color(self.style.color)
                    ).clicked() {
                        let mut binding = ui.data();
                        let scroll_pos: &mut usize = binding.get_temp_mut_or::<usize>(scroll_pos_id, 0);
                        if *scroll_pos > 0 {
                            *scroll_pos -= 1;
                        }
                    }
                    if ui.add(IconButton::new(Dingbat::Down)
                        .with_no_bg()
                        .with_color(self.style.color)
                    ).clicked() {
                        let mut binding = ui.data();
                        let scroll_pos: &mut usize = binding.get_temp_mut_or::<usize>(scroll_pos_id, 0);
                        if *scroll_pos < (preset_count - max_visible) {
                            *scroll_pos += 1;
                        }
                    }
                    */

                    if ui.add(
                        IconButton::new(Dingbat::Save)
                            .with_style(&self.style.icon_style)
                        )
                        .on_hover_text("Save Preset")
                        .clicked() && !is_saving
                    {
                        ui.data_mut(|r| {
                            *(r.get_temp_mut_or::<bool>(is_saving_id, false)) = true;
                            is_saving = true;
                            on_save_open = true;
                        });
                    }

                    if ui.add(
                        IconButton::new(Dingbat::Refresh)
                            .with_style(&self.style.icon_style)
                        )
                        .on_hover_text("Refresh User Presets")
                        .clicked() && !is_saving
                    {
                        preset_manager.refresh();
                        self.update_search(ui, preset_manager, search_text_id, search_results_id);
                        is_refreshing = true;
                    }

                    if ui.add(
                        IconButton::new(Dingbat::Open)
                            .with_style(&self.style.icon_style)
                        )
                        .on_hover_text("View in File Manager")
                        .clicked() && !is_saving
                    {
                        if let Some(path) = preset_manager.get_preset_path() {
                            let _ = open::that(path);
                        }
                    }

                    let mut search_text: String = ui.data(|r|r.get_temp::<String>(search_text_id).unwrap_or("".to_string()).to_string());

                    // Hidden feature to generate random presets
                    if &search_text == "RANDOMIZE" && ui.add(
                            IconButton::new(Dingbat::Refresh)
                                .with_style(&self.style.icon_style)
                            )
                            .on_hover_text("Randomize current preset")
                            .clicked() && !is_saving {
                        let preset = Preset::from_random_values(params)
                            .filter_params_exclude(&["Gain", "Volume", "Pan"], &[]);
                        preset.apply(params, setter);
                    }

                    ui.scope(|ui| {
                        ui.style_mut().visuals.override_text_color = Some(self.style.color);
                        ui.label("Search");
                    });
                    ui.scope(|ui| {
                        if let Some(color) = self.style.search_bg_color {
                            ui.style_mut().visuals.extreme_bg_color = color;
                        }
                        let color = self.style.search_color.unwrap_or(self.style.color);
                        ui.style_mut().visuals.override_text_color = Some(color);
                        if ui.add(
                            TextEdit::singleline(&mut search_text)
                        ).changed() {
                            ui.data_mut(|r|{*(r.get_temp_mut_or::<String>(search_text_id, {"".to_string()})) = search_text;});
                            self.update_search(ui, preset_manager, search_text_id, search_results_id);
                        }
                    });
                });

                //let scroll_pos: usize = ui.data().get_temp::<usize>(scroll_pos_id).unwrap_or(0);

                ui.scope(|ui|{
                    if let Some(color) = self.style.search_bg_color {
                        ui.style_mut().visuals.extreme_bg_color = color;
                    }

                    let refresh_flash_amount = ui.ctx().animate_bool_with_time(
                        refresh_flash_id,
                        is_refreshing,
                        if is_refreshing { 0.0 } else { 0.5 },
                    );
                    let mut label_style = label_style.clone();
                    let mut label_style_selected = label_style_selected.clone();
                    let flash_max = 0.66;
                    label_style.bg_color = label_style.bg_color.linear_multiply(1.0-refresh_flash_amount*flash_max);
                    label_style_selected.bg_color = label_style_selected.bg_color.linear_multiply(1.0-refresh_flash_amount*flash_max);
                    label_style_selected.color_hover = None;
                    label_style_selected.bg_color_hover = None;

                    
                    // FIXME: ScrollArea is Focusable! 
                    // Need to deal with this somehow... maybe set focus on the first item if Area is focused
                    // let mut first_preset_label: Option<Response> = None;
                    let scroll_output = ScrollArea::vertical().show(ui, |ui| {
                        // This is a hack because labels aren't recieving the gained_focus() event
                        let allow_autoscroll = ui.input(|r|{
                            for event in &r.events {
                                if let Event::Key{key, pressed, modifiers, repeat, physical_key} = event {
                                    return true;
                                }
                            }
                            false
                            }
                        );
                        preset_has_focus = false;
                        //let search_string: String = ui.data(|r|{(*(r.get_temp::<String>(search_text_id).unwrap_or("".to_string()))).to_string().to_lowercase()});
                        //let search_results = preset_manager.search(&search_string);
                        let search_results = self.get_search_results(ui, preset_manager, search_results_id);
                        for index in search_results {
                            let preset = preset_manager.get_preset(index);
                            if let Some(preset) = preset {
                                let mut name = preset.name.to_string();
                                // if preset.has_tag(&FACTORY_TAG.to_string()) {
                                //     name = format!("{}: {}", FACTORY_TAG, name);
                                // }
                                let label_style = if Some(index) == active_index {
                                    &label_style_selected
                                } else {
                                    &label_style
                                };
                                let response = ui.add(
                                    Label::new(name).with_style(label_style).with_clickable(),
                                );
            
                                // Filter Navigation Keys
                                ui.memory_mut(|mem| mem.set_focus_lock_filter(response.id, EventFilter{horizontal_arrows:false,vertical_arrows:false,escape:true, ..Default::default()}));            
                                if response.clicked()
                                    // && !response.double_clicked() // Superseded by throttling
                                    // && !response.triple_clicked()
                                    && !is_saving
                                    && !self.throttle_preset_change(ui, preset_changed_timestamp) {
                                    preset_manager.activate_preset(index, params, setter);
    
                                    self.update_edit_tags(ui, preset_manager, tags_text_id);
                                    changed_selection = true;
                                    active_index = Some(index);
                                    response.request_focus();
                                }
                                if Some(index) == active_index && changed_selection {
                                    response.scroll_to_me(None);
                                    response.request_focus();
                                }
                                // This stopped triggering in an update...
                                // if response.gained_focus() {
                                //     response.scroll_to_me(None);
                                // }
                                if response.has_focus() {
                                    selected_index = Some(index);
                                    preset_has_focus = true;
                                    // This is a hack because labels aren't recieving the gained_focus() event
                                    if allow_autoscroll {
                                        //response.scroll_to_me(None);//Some(Align::Center));
                                        // Fix for selecting the next control instead of scrolling
                                        // Expand selection to reveal part of the next item
                                        ui.scroll_to_rect(response.rect.expand2(vec2(10.0, 10.0)), None);
                                    }
                                }
                                // if !first_preset_label.is_some() {
                                //     first_preset_label = Some(response);
                                // }
                            }
                        }
                    });

                    // if let Some(focus) = ui.memory(|m|m.focus()) {
                    //     nih_log!("Focus: {}... {}", focus.short_debug_format(), scroll_output.id.short_debug_format);
                    // }
                    // if Some(scroll_output.id) == ui.memory(|m|m.focus())
                    // {
                    //     nih_log!("Bad Focus");
                    //     first_preset_label.unwrap().request_focus();
                    // }
                    ui.data_mut(|r| {
                        *(r.get_temp_mut_or::<bool>(preset_focused_id, false)) = preset_has_focus;
                    });

                    if !is_saving && preset_has_focus {
                        Area::new("preset_info".into())
                        .interactable(false)
                        .fixed_pos(popup_rect.min)
                        //.fixed_rect(popup_rect)
                        .order(Order::Foreground)
                        //.inner_margin(Margin::same(10.0))
                        .show(ui.ctx(), |ui| {
                                //ui.style_mut().visuals.override_text_color = Some(Color32::WHITE);
                                //Set Size, and adjust for margins, how do you detect margins?
                                Frame::none()
                                .outer_margin(Margin::same(0.0))
                                .inner_margin(Margin::same(5.0))
                                .fill(Color32::from_rgba_premultiplied(32, 32, 32, 220))
                                .rounding(Rounding::same(10.0)).show(ui, |ui| {
                                    ui.set_width(popup_rect.width()-10.0); //Subtract margins
                                    ui.set_height(popup_rect.height()-10.0); //Subtract margins

                                    ui.painter().dingbat(Pos2::new(popup_rect.right()-8.0, popup_rect.top()+8.0), Align2::RIGHT_TOP, Dingbat::Info, 16.0, Color32::from_white_alpha(128));
                                        /*
                                    ui.vertical_centered(|ui|{
                                        ui.add(Label::new("Save Preset"));
                                    });
                                    */
    
                                    //let preset = preset_manager.get_active();
                                    //let preset = preset_manager.get_preset(active_index);
                                    if let Some(index) = selected_index {
                                        if let Some(preset) = preset_manager.get_preset(index) {

                                            let label_x = ui.next_widget_position().x+74.0;
            
                                            ui.label(format!("Name: {}", preset.name));
                                            ui.label(format!("Author: {}", preset.get_info(AUTHOR_KEY)));
                                            ui.label(format!("Description: {}", preset.get_info(DESCRIPTION_KEY)));
                                            ui.label(format!("Tags: {}", preset.get_tags_string()));
                                            /*
                                            let style = LabelStyle {
                                                bg_color: Color32::GRAY,
                                                ..Default::default()
                                            };
                                            ui.horizontal_wrapped(|ui| {
                                                ui.label("Tags: ");
                                                for tag in preset.get_tags() {
                                                    ui.add(
                                                        Label::new(tag)
                                                        .with_style(&style)
                                                    );
                                                }
                                            });
                                            */
                                        }
                                    }
                                });
                        });                        
                    }
                });
            }

            let was_open = is_saving;
            ui.scope(|ui| {
                //ui.style_mut().visuals.window_fill = Color32::WHITE;
                //ui.style_mut().visuals.window_shadow = Shadow{extrusion: 10.0, color: Color32::from_black_alpha(0)};
                ui.style_mut().visuals.extreme_bg_color = Color32::WHITE;

                /*
                Window::new("Save Preset")
                .open(&mut is_saving)
                .fixed_rect(save_rect)
                .collapsible(false)
                .show(ui.ctx(), |ui| {
                */
                if is_saving {
                    Area::new("save_preset".into())
                    .fixed_pos(save_rect.min)
                    //.fixed_rect(save_rect)
                    .order(Order::Foreground)
                    //.inner_margin(Margin::same(10.0))
                    .show(ui.ctx(), |ui| {
                            //ui.style_mut().visuals.override_text_color = Some(self.style.color);
                            Frame::none()
                            .outer_margin(Margin::same(0.0))
                            .inner_margin(Margin::same(5.0))
                            .fill(Color32::from_rgba_premultiplied(32, 32, 32, 220))
                            .rounding(Rounding::same(10.0)).show(ui, |ui| {
                                ui.set_width(save_rect.width()-10.0); //Subtract margins
                                ui.set_height(save_rect.height()-10.0); //Subtract margins
                                /*
                                ui.vertical_centered(|ui|{
                                    ui.add(Label::new("Save Preset"));
                                });
                                */

                                let preset = preset_manager.get_active();
                                let label_x = ui.next_widget_position().x+74.0;

                                ui.horizontal(|ui|{
                                    ui.label("Name: ");
                                    ui.add_space(label_x-ui.next_widget_position().x);
                                    let mut text = preset.name.clone();
                                    let response = ui.add(
                                        TextEdit::singleline(&mut text)
                                    );
                                    if on_save_open {
                                        response.request_focus();
                                    }
                                    if response.changed() {
                                        preset.name = text;
                                    }
                                });
                                ui.horizontal(|ui|{
                                    ui.label("Author: ");
                                    ui.add_space(label_x-ui.next_widget_position().x);
                                    let mut text = preset.get_info(AUTHOR_KEY);
                                    let response = ui.add(
                                        TextEdit::singleline(&mut text)
                                    );
                                    if response.changed() {
                                        preset.set_info(AUTHOR_KEY, &text);
                                    }
                                });
                                ui.horizontal(|ui|{
                                    ui.label("Description: ");
                                    ui.add_space(label_x-ui.next_widget_position().x);
                                    let mut text = preset.get_info(DESCRIPTION_KEY);
                                    let response = ui.add(
                                        TextEdit::singleline(&mut text)
                                    );
                                    if response.changed() {
                                        preset.set_info(DESCRIPTION_KEY, &text);
                                    }
                                });
                                ui.horizontal(|ui|{
                                    ui.label("Tags: ");
                                    ui.add_space(label_x-ui.next_widget_position().x);
                                    let mut tags_text: String = ui.data(|r|r.get_temp::<String>(tags_text_id).unwrap_or("".to_string()).to_string());
                                    //let mut text = preset.get_tags_string();
                                    let response = ui.add(
                                        TextEdit::singleline(&mut tags_text)
                                    );
                                    if response.changed() {
                                        preset.set_tags_string(tags_text.clone());
                                        ui.data_mut(|r|{*(r.get_temp_mut_or::<String>(tags_text_id, "".to_string())) = tags_text;});
                                    }

                                });

                                ui.add_space(5.0);

                                ui.horizontal(|ui|{
                                    let mut should_close = false;
                                    if ui.button("Cancel").clicked() {
                                        should_close = true;
                                    }
                                    ui.set_enabled(preset_manager.has_write_access());
                                    if ui.button("Save!").clicked() {
                                        preset_manager.save_active(params);
                                        if preset_manager.has_write_access() { // Only refresh/close on success
                                            self.update_search(ui, preset_manager, search_text_id, search_results_id);
                                            should_close = true;
                                        }
                                    }
                                    ui.set_enabled(true);
                                    if should_close {
                                        ui.data_mut(|r| {
                                            *(r.get_temp_mut_or::<bool>(is_saving_id, false)) = false;
                                        });
                                    }
                                });
                                if !preset_manager.has_write_access() {
                                    ui.label("ERROR: No write access to preset folder.");
                                }
                            });
                    });
                }
            });
            if was_open && !is_saving {
                ui.data_mut(|r| {
                    *(r.get_temp_mut_or::<bool>(is_saving_id, false)) = false;
                });
            }

        }).response
    }
}
