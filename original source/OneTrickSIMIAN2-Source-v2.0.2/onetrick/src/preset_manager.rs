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

use std::sync::Arc;

use std::fs;
use std::fs::File;
use std::io::{Write};
use std::path::{Path, PathBuf};

#[allow(unused_imports)]
use directories::{
    UserDirs,
    ProjectDirs,
};

//use nih_plug::context::gui::ParamSetter;
use nih_plug::prelude::*;

use super::preset::*;
use super::params::*;

/// Manages a list of Presets for use in the GUI and elsewhere
#[derive(Default)]
pub struct PresetManager {
    /// Base directory where plugin data can be stored
    plugin_folder: String,

    /// List of available Factory Presets
    factory_presets: Vec<Preset>,

    /// List of available Presets
    presets: Vec<Preset>,

    /// The current active Preset
    active: Preset,

    /// The current active Preset Index
    active_index: Option<usize>,

    /// The current active Preset Name for persistence
    active_field: Option<Arc<FieldString>>,

    /// The current active_field value for checking if Dirty
    active_field_value: String,

    /// Parameters to exclude from applying
    excluded_params: Vec<String>,

    /// Tracks read access to Presets Directory
    read_access: bool,
    /// Tracks write access to Presets Directory
    write_access: bool,
}

impl PresetManager {

    /// Returns a new PresetManager for with a specified plugin folder name
    /// The folder name should be something like "OneTrick SIMIAN"
    pub fn new(plugin_folder: &str) -> Self {
        Self {
            plugin_folder: plugin_folder.to_string(),
            read_access: true,
            write_access: true,
            ..Default::default()
        }
    }

    pub fn has_read_access(&self) -> bool {
        self.read_access
    }

    pub fn has_write_access(&self) -> bool {
        self.read_access && self.write_access
    }

    pub fn with_active_field(mut self, active_field: Option<Arc<FieldString>>) -> Self {
        self.active_field = active_field;
        self
    }

    /// Excludes params from being applied when activating a preset
    pub fn with_excluded_params(mut self, params: &[&str]) -> Self {
        let mut params_vec: Vec<String> = Vec::new();
        for p in params {
            params_vec.push(p.to_string());
        }
        self.excluded_params = params_vec;
        self
    }


    #[cfg(not(target_os = "macos"))]
    /// Gets the Preset Directory on Linux/Windows
    pub fn get_preset_path(&self) -> Option<String> {
        let project_dirs = ProjectDirs::from("com", "PunkLabs", &self.plugin_folder)?;
        let path = project_dirs.data_local_dir().join(PathBuf::from("Presets"));
        if !Path::new(&path).exists() {
            fs::create_dir_all(path.clone()).ok()?;
        }
        Some(path.to_str()?.to_string())
    }
    #[cfg(target_os = "macos")]
    /// Gets the Preset Directory on macOS, which should be...
    /// ~/Library/Audio/Presets/ to avoid Sandbox issues using GarageBand
    pub fn get_preset_path(&self) -> Option<String> {
        let user_dirs = UserDirs::new()?;
        let home_dir = user_dirs.home_dir();
        let path = home_dir.join(PathBuf::from("Library/Audio/Presets/Punk Labs/".to_string()+&self.plugin_folder));
        if !Path::new(&path).exists() {
            fs::create_dir_all(path.clone()).ok()?;
        }
        Some(path.to_str()?.to_string())
    }

    /// Refreshes the list of Presets
    pub fn refresh(&mut self) {
        // Assume we have access until further notice
        // The user may have corrected an issue, and wants to refresh...
        self.read_access = true;
        self.write_access = true;

        self.active_index = None;
        self.presets.clear();
        self.presets.extend(self.factory_presets.clone()); //Start with all Factory Presets

        if let Some(path) = self.get_preset_path() {
            self.read_access = true;
            //let directory = fs::read_dir(&path).unwrap();
            // ... in directory.flatten()
            let mut directory: Vec<_> = fs::read_dir(path).unwrap()
                .map(|r| r.unwrap())
                .collect();
            directory.sort_by_key(|dir| dir.path());

            for dir_entry in directory {
                let entry_path = dir_entry.path();
                let path = Path::new(&entry_path);
                if let Some(extension) = path.extension() {
                    if extension == "preset" {
                        if let Ok(contents) = fs::read_to_string(path) {
                            //nih_log!("Preset: {}", contents);
                            if let Some(name) = path.file_stem() {
                                if let Some(name) = name.to_str() {
                                    if let Some(mut preset) = Preset::from_string(name, &contents) {
                                        preset.remove_tag(FACTORY_TAG);
                                        self.presets.push(preset);
                                    }
                                }
                            }
                        }
                    }
                }
            }
        } else {
            self.read_access = false;
        }

        // Restore Active Index
        for (index, preset) in self.presets.iter().enumerate() {
            if *preset == self.active {
                self.active_index = Some(index);
                break;
            }
        }

        // No active_index? Restore Active Index from Active Field...
        self.refresh_active_field()
    }

    fn active_field_is_dirty(&self) -> bool {
        if let Some(active_field) = &self.active_field {
            let old_value = active_field.get();
            return self.active_field_value != old_value;
        }
        false
    }
    fn refresh_active_field(&mut self) {
        if self.active_field_is_dirty() {
            if let Some(active_field) = &self.active_field {
                let new_value = active_field.get();
                for (index, preset) in self.presets.iter().enumerate() {
                    if preset.name == new_value {
                        self.active_index = Some(index);
                        break;
                    }
                }
                self.active_field_value = new_value;
            }
    
        }
    }

    /// Adds a Factory Preset from a string (JSON)
    pub fn add_factory_string(&mut self, name: &str, preset: &str) {
        if let Some(mut preset) = Preset::from_string(name, preset) {
            preset.insert_tag(0, FACTORY_TAG);
            self.add_factory(preset);
        }
    }

    /// Adds a Factory Preset
    pub fn add_factory(&mut self, preset: Preset) {
        let mut preset = preset.clone();
        preset.insert_tag(0, FACTORY_TAG);
        self.factory_presets.push(preset);
    }

    /// Returns a Preset by index
    pub fn get_preset(&self, index: usize) -> Option<&Preset> {
        if self.presets.len() > index {
            Some(&self.presets[index])
        } else {
            None
        }
    }

    /// Returns a list of Presets
    pub fn get_presets(&self) -> Vec<Preset> {
        self.presets.to_vec()
    }

    /// Returns the number of presets
    pub fn get_preset_count(&self) -> usize {
        self.presets.len()
    }

    /// Sets the active Preset
    pub fn set_active(&mut self, preset: Preset, params: &OneTrickPluginParams, setter: &ParamSetter) {
        let excluded_params = self.excluded_params.clone();
        let filtered_preset = preset.clone().filter_params(&mut |param: &str| {
            for p in &excluded_params {
                if param.contains(p) {
                    return false
                }
            }
            true
        });
        self.active = preset;
        filtered_preset.apply(params, setter);
    }

    /// Returns the active Preset
    pub fn get_active(&mut self) -> &mut Preset {
        &mut self.active
    }

    /// Returns the index of the active Preset
    pub fn get_active_index(&mut self) -> Option<usize> {
        self.refresh_active_field();
        self.active_index
    }

    /// Activates a Preset by index
    pub fn activate_preset(&mut self, index: usize, params: &OneTrickPluginParams, setter: &ParamSetter) {
        if let Some(preset) = self.get_preset(index) {
            if let Some(active_field) = &self.active_field {
                active_field.set(&preset.name);
            }
            // nih_log!("Activate Preset: {}", preset.name);
            self.set_active(preset.clone(), params, setter);
            self.active_index = Some(index);
        } else {
            self.active_index = None;
        }
    }

    fn refresh_active(&mut self, params: &OneTrickPluginParams) {
        self.active.refresh(params);
    }

    /// Saves the active Preset to disk.
    pub fn save_active(&mut self, params: &OneTrickPluginParams) {
        self.refresh_active(params);
        if let Some(path) = self.get_preset_path() {
            if !self.active.name.is_empty() {
                if let Some(json) = self.active.to_string() {
                    // Hack: make sure the active preset is identical to the newly saved one
                    // This will help reselecting it after a refresh
                    if let Some(newly_saved_preset) = Preset::from_string(&self.active.name, &json) {
                        self.active = newly_saved_preset.clone();
                    }

                    let full_path = path+"/"+&self.active.name+".preset";
                    if let Ok(mut output) = File::create(full_path) {
                        self.write_access = true;
                        let _ = write!(output, "{}", json);
                        self.refresh();
                    } else {
                        self.write_access = false;
                    }
                }
            }
        }
    }

    /// Returns a filtered list of indices for Presets that match the search terms
    pub fn search(&self, search: &str) -> Vec<usize> {
        let search = search.trim_start().trim_end().to_string();
        let search_terms = search.split(' ').collect::<Vec<&str>>();
        let is_searching = !search_terms.is_empty();
        self.get_presets().iter().enumerate()
            .filter(|(_i,p)| { // Search Filter
                //let is_searching = !cleaned_search_string.is_empty();
                if !is_searching {
                    return true;
                }
                let p_name_lower = p.name.to_lowercase();
                let p_auth_lower = p.get_info(AUTHOR_KEY.to_string()).to_lowercase();
                let p_desc_lower = p.get_info(DESCRIPTION_KEY.to_string()).to_lowercase();
                for term in &search_terms {
                    if p_name_lower.contains(term)
                    || p_auth_lower.contains(term)
                    || p_desc_lower.contains(term)
                    || p.has_tag(*term)
                    {
                        continue;
                    }
                    return false;
                }
                true
                //|| p.name.to_lowercase().contains(&cleaned_search_string)
                //|| p.has_tag(&cleaned_search_string)
            }).map(|(i,_p)| i).collect::<Vec<usize>>()
    }
}