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

use std::collections::{BTreeMap, HashMap};
use nih_plug::prelude::*;
use nih_plug::wrapper::state::ParamValue;

use serde::{Serialize, Serializer, Deserialize};
use serde_json::{json, Value};

use rand::Rng;

/// Tag for Factory presets
pub const FACTORY_TAG: &str = "Factory";

/// Key for the Author field
pub const AUTHOR_KEY: &str = "Author";

/// Key for the Description field
pub const DESCRIPTION_KEY: &str = "Description";

use super::params::*;


/// Stores a OneTrick Preset with metadata, tags, and parameter values
#[derive(Clone, PartialEq)]
#[derive(Serialize, Deserialize)]
pub struct Preset {
    /// Name of the Preset
    #[serde(skip_serializing, skip_deserializing)]
    pub name: String,

    /// List of Tags for the Preset
    #[serde(serialize_with = "serde_sorted_vec")]
    pub tags: Vec<String>,

    /// List of metadata for the Preset
    #[serde(serialize_with = "serde_sorted_map")]
    pub info: HashMap<String, String>,

    /// Map of parameter names and values
    #[serde(serialize_with = "serde_sorted_map")]
    pub params: HashMap<String, Value>,
}

fn serde_sorted_map<S, K: Ord + Serialize, V: Serialize>(
    value: &HashMap<K, V>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let ordered: BTreeMap<_, _> = value.iter().collect();
    ordered.serialize(serializer)
}

fn serde_sorted_vec<S, K: Ord + Serialize+ Clone>(
    value: &[K],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let mut ordered = value.to_vec();
    ordered.sort();
    ordered.serialize(serializer)
}

impl Default for Preset {
    fn default() -> Self {
        Self {
            name: "New Preset".to_string(),
            tags: Vec::new(),
            info: HashMap::new(),
            params: HashMap::new(),
        }
    }
}
/*
impl PartialEq for Preset {
    fn eq(&self, other: &Self) -> bool {
        self.params == other.params
    }
}
*/

impl Preset {

    /// Returns a new Preset from OneTrickPluginParams
    pub fn from_params(name: &str, params: &OneTrickPluginParams) -> Self {
        let mut result = Self {
            name: name.to_string(),
            ..Default::default()
        };
        result.refresh(params);
        result
    }

    /// Returns a new Factory Default Preset from OneTrickPluginParam default values
    pub fn from_param_defaults(name: &str, params: &OneTrickPluginParams) -> Self {
        let mut result = Self {
            name: name.to_string(),
            ..Default::default()
        };

        result.params.clear();
        for (id, param, _group) in &params.params {
            match param {
                ParamType::BoolParam(p) => {
                    result.params.insert(id.to_string(), json!(p.default_plain_value()));
                }
                ParamType::FloatParam(p) => {
                    result.params.insert(id.to_string(), json!(p.default_plain_value()));
                }
                ParamType::IntParam(p) => {
                    result.params.insert(id.to_string(), json!(p.default_plain_value()));
                }
            }
        }
        result.set_info(AUTHOR_KEY, "Punk Labs");
        result.set_info(DESCRIPTION_KEY, "Default Preset");
        result.add_tag("Default");
        result
    }

    /// Returns a new Factory Default Preset from OneTrickPluginParam default values
    pub fn from_random_values(params: &OneTrickPluginParams) -> Self {
        let mut result = Self {
            name: format!("Random {}", rand::thread_rng().gen_range(0..10000)).to_string(),
            ..Default::default() };

        result.params.clear();
        for (id, param, _group) in &params.params {
            match param {
                ParamType::BoolParam(_p) => {
                    let value = rand::thread_rng().gen_range(0..2) != 0;
                    result.params.insert(id.to_string(), json!(value));
                }
                ParamType::FloatParam(p) => {
                    let n = rand::thread_rng().gen_range(0..10000) as f32 / 10000.0;
                    let value = p.preview_plain(n);
                    result.params.insert(id.to_string(), json!(value));
                }
                ParamType::IntParam(p) => {
                    let n = rand::thread_rng().gen_range(0..10000) as f32 / 10000.0;
                    let value = p.preview_plain(n);
                    result.params.insert(id.to_string(), json!(value));
                }
            }
        }
        result.set_info(AUTHOR_KEY, "Punk Labs");
        result.set_info(DESCRIPTION_KEY, "Default initialization");
        result.add_tag("Default");
        result
    }

    pub fn filter_params<FilterFn>(mut self, param_filter: &mut FilterFn) -> Self
        where FilterFn: Fn(&str) -> bool, // (param_name) returns true if param is okay to keep
    {
        let mut params_to_remove: Vec<String> = vec![];
        for param in self.params.keys() {
            if !param_filter(param) {
                params_to_remove.push(param.to_owned());
            }
        }
        for param in params_to_remove {
            self.params.remove(&param);
        }
        self
    }


    pub fn filter_params_exclude(self, exclude: &[&str], except_for: &[&str]) -> Self {
        self.filter_params(&mut |param: &str|{
            let mut should_exclude = false;
            for s in exclude {
                if param.contains(s) {
                    //continue 2;
                    should_exclude = true;
                    break;
                }
            }
            if should_exclude {
                for s in except_for {
                    if param.contains(s) {
                        should_exclude = false;
                    }
                }
            }
            !should_exclude
        })
    }

    pub fn filter_params_include(self, include: &[&str]) -> Self {
        self.filter_params(&mut |param: &str|{
            for s in include {
                if param.contains(s) {
                    return true;
                }
            }
            false
        })
    }
    
    pub fn copy_params_by_prefix(mut self, from_prefix: &str, to_prefix: &str) -> Self {
        let mut param_list: Vec<String> = vec![];
        for param in self.params.keys() {
            if param.starts_with(from_prefix){
                param_list.push(param.to_owned());
            }
        }
        for from_param in param_list {
            let to_param = from_param.replace(from_prefix, to_prefix);
            let value = &self.params[&from_param];
            self.params.insert(to_param, value.clone());
        }
        self
    }
    
    /// Returns a new Preset from a string representation (JSON)
    pub fn from_string(name: &str, preset_string: &str) -> Option<Self> {
        if let Ok(mut preset) = serde_json::from_str::<Self>(preset_string) {
            preset.name = name.to_string();
            Some(preset)
        } else {
            None
        }
    }

    /// Updates all Preset param values from OneTrickPluginParams
    pub fn refresh(&mut self, params: &OneTrickPluginParams) {
        self.params.clear();
        for (id, param, _group) in &params.params {
            match param {
                ParamType::BoolParam(p) => {
                    self.params.insert(id.to_string(), json!(p.value()));
                }
                ParamType::FloatParam(p) => {
                    self.params.insert(id.to_string(), json!(p.value()));
                }
                ParamType::IntParam(p) => {
                    self.params.insert(id.to_string(), json!(p.value()));
                }
            }
        }
    }

    /// Returns a string representation (JSON) of the Preset
    pub fn to_string(&self) -> Option<String> {
        // Let's create a copy for editing...
        // We can fill out default info, etc...
        let mut edit = self.clone();
        if !edit.has_info(AUTHOR_KEY) {
            edit.set_info(AUTHOR_KEY, "");
        }
        if !edit.has_info(DESCRIPTION_KEY) {
            edit.set_info(DESCRIPTION_KEY, "");
        }
        edit.remove_tag(FACTORY_TAG);
        serde_json::to_string_pretty(&edit).ok()
    }

    /// Applies the Preset values to a ParamSetter using automation
    /// WARNING: While this should be valid, it can overwhelm hosts...
    pub fn apply_with_automation(&self, params: &OneTrickPluginParams, setter: &ParamSetter) {
        for (key, value) in &self.params {
            if let Some(param) = params.param_bool(key) {
                if let Ok(value) = serde_json::from_value::<bool>(value.clone()) {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, value);
                    setter.end_set_parameter(param);
                }
            } else if let Some(param) = params.param_float(key) {
                if let Ok(value) = serde_json::from_value::<f32>(value.clone()) {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, value);
                    setter.end_set_parameter(param);
                }
            } else if let Some(param) = params.param_int(key) {
                if let Ok(value) = serde_json::from_value::<i32>(value.clone()) {
                    setter.begin_set_parameter(param);
                    setter.set_parameter(param, value);
                    setter.end_set_parameter(param);
                }
            }
        }
    }

    /// Applies the Preset values to a ParamSetter's GuiContext
    pub fn apply(&self, params: &OneTrickPluginParams, setter: &ParamSetter) {
        let ctx = &setter.raw_context;
        let state = self.get_plugin_state(params, setter);
        ctx.set_state(state);
    }

    /// Returns a new PluginState with the Preset values applied
    pub fn get_plugin_state(&self, params: &OneTrickPluginParams, setter: &ParamSetter) -> PluginState {
        let ctx = &setter.raw_context;
        let mut state = ctx.get_state().clone();
        for (id, param, _group) in params.param_map(){
            if state.params.contains_key(&id.to_owned()) {
                let name = unsafe { param.name() };
                if let Some(value) = self.params.get(name) {
                    match param {
                        ParamPtr::BoolParam(_p) => {
                            if let Ok(value) = serde_json::from_value::<bool>(value.clone()) {
                                state.params.entry(id.to_owned()).and_modify(|e|*e=ParamValue::Bool(value));
                            }
                        },
                        ParamPtr::FloatParam(_p) => {
                            if let Ok(value) = serde_json::from_value::<f32>(value.clone()) {
                                state.params.entry(id.to_owned()).and_modify(|e|*e=ParamValue::F32(value));
                            }
                        },
                        ParamPtr::IntParam(_p) => {
                            if let Ok(value) = serde_json::from_value::<i32>(value.clone()) {
                                state.params.entry(id.to_owned()).and_modify(|e|*e=ParamValue::I32(value));
                            }
                        },
                        _ => {},
                    }
                }
            }
        }
        state
    }

    /// Returns true if the Preset has a tag, case sensitive
    pub fn has_tag_case_sensitive<S: Into<String>>(&self, tag: S) -> bool {
        let tag = tag.into();
        self.tags.iter().any(|t|*t==tag)
    }
    
    /// Returns optional index of the tag, case sensitive
    pub fn tag_index_case_sensitive<S: Into<String>>(&self, tag: S) -> Option<usize> {
        let tag = tag.into();
        self.tags.iter().position(|t|*t==tag)
    }

    /// Returns true if the Preset has a tag, case insensitive
    pub fn has_tag<S: Into<String>>(&self, tag: S) -> bool {
        let tag = tag.into();
        self.tags.iter().any(|t|t.eq_ignore_ascii_case(&tag))
    }

    /// Returns optional index of the tag, case insensitive
    pub fn tag_index<S: Into<String>>(&self, tag: S) -> Option<usize> {
        let tag = tag.into();
        self.tags.iter().position(|t|t.eq_ignore_ascii_case(&tag))
    }

    /// Returns a list of tags
    pub fn get_tags(&self) -> &Vec<String> {
        &self.tags
    }

    /// Returns a comma-separated list of tags
    pub fn get_tags_string(&self) -> String {
        self.tags.join(" ")
    }

    /// Returns a comma-separated list of tags, filtered for disallowed tags ("Factory")
    pub fn get_tags_string_filtered(&self) -> String {
        self.tags.iter().filter_map(|t|{
            if t==FACTORY_TAG {
                None
            } else {
                Some(t.to_string())
            }
        }).collect::<Vec<_>>().join(" ")
    }

    /// Sets the list of tags from a comma-separated list of tags
    pub fn set_tags_string<S: Into<String>>(&mut self, text: S) {
        let text = text.into();
        self.tags.clear();
        for tag in text.replace(',', " ").split(' ') {
            self.add_tag(&tag.trim_start().trim_end().to_string());
        }
    }

    /// Adds a tag to the Preset
    pub fn add_tag<S: Into<String>>(&mut self, tag: S) {
        let tag = tag.into();
        if !tag.is_empty() && !self.has_tag(&tag) {
            self.tags.push(tag.to_string());
        }
    }

    /// Adds a tag to the Preset
    pub fn insert_tag<S: Into<String>>(&mut self, index: usize, tag: S) {
        let tag = tag.into();
        if !tag.is_empty() && !self.has_tag(&tag) {
            self.tags.insert(index, tag.to_string());
        }
    }

    /// Removes a tag to the Preset
    pub fn remove_tag<S: Into<String>>(&mut self, tag: S) {
        let tag = tag.into();
        if let Some(index) = self.tag_index(tag) {
            self.tags.remove(index);
        }
    }

    /// Returns true if the Preset's info contains a key
    pub fn has_info<S: Into<String>>(&self, key: S) -> bool {
        self.info.contains_key(&key.into())
    }

    /// Adds or updates a key of info in the Preset
    pub fn set_info<S: Into<String>>(&mut self, key: S, value: S) {
        self.info.insert(key.into(), value.into());
    }

    /// Returns info by key (or "" if not found)
    pub fn get_info<S: Into<String>>(&self, key: S) -> String {
        self.info.get(&key.into()).unwrap_or(&"".to_string()).to_string()
    }

    /// Removes a key of info in the PReset
    pub fn remove_info<S: Into<String>>(&mut self, key: S) {
        self.info.remove(&key.into());
    }
}