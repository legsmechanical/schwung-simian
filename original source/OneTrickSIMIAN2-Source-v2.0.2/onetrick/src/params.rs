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
use std::sync::atomic::{AtomicI32, Ordering};
use parking_lot::RwLock;

use nih_plug::prelude::*;

use std::collections::BTreeMap;
// use serde::{Serialize, Serializer, Deserialize};
// use serde_json::{json, Value};
// use nih_plug::params::persist::PersistentField;

// use std::fs;
// use std::fs::File;
// use std::io::{Write};
// use std::path::{Path, PathBuf};

use crate::faust::OneTrickDSPGeneral;
use crate::preset::Preset;

#[cfg(feature = "egui")]
use nih_plug_egui::EguiState;
//#[cfg(feature = "egui")]
//use nih_plug::params::persist::PersistentField;

/// Parameter Types
pub enum ParamType {
    /// A boolean parameter
    BoolParam(BoolParam),
    /// A float parameter
    FloatParam(FloatParam),
    /// An int parameter
    IntParam(IntParam),
}

pub struct FieldI32 {
    value: Arc<AtomicI32>,
}
impl FieldI32 {
    pub fn new(default_value: i32) -> Self {
        Self {
            value: Arc::new(AtomicI32::new(default_value)),
        }
    }
    pub fn get(&self) -> i32 {
        self.value.load(Ordering::Relaxed)
    }
    pub fn set(&self, value: i32) {
        self.value.store(value, Ordering::Relaxed);
    }
}

pub struct FieldString {
    value: RwLock<String>,
}
impl FieldString {
    pub fn new(default_value: String) -> Self {
        Self {
            value: RwLock::new(default_value.to_owned()),
        }
    }
    pub fn get(&self) -> String {
        self.value.read().to_owned()
    }
    pub fn set(&self, value: &str) {
        *(self.value.write()) = value.to_string();
    }
}
/// Field Types
/// Should consider switching to PersistentField to play nicer with nih-plug
pub enum FieldType {
    IntField(Arc<FieldI32>),
    StringField(Arc<FieldString>),
}

/// Container for OneTrick plugin parameters and editor state
pub struct OneTrickPluginParams {

    /// The editor state, saved together with the parameter state so the custom scaling can be
    /// restored.
    #[cfg(feature = "egui")]
    //#[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    /* // Not sure how to do this manually...
    #[cfg(feature = "egui")]
    #[persist = "editor-state"]
    pub editor_state: Arc<EguiState>,
    */
    //#[id = "global_volume"] //Global_Volume dB
    //pub global_volume: FloatParam,

    /// Map of parameters (export_id, param, group)
    pub params: Vec<(String, ParamType, String)>,

    pub fields: RwLock<BTreeMap<String, FieldType>>,

    // /// True the Editor fields have already been loaded
    // editor_fields_loaded: bool,
    
    // /// Editor fields are only deserialized once at start
    // pub editor_fields: BTreeMap<String, Value>,

    // /// Preset fields are deserialized every time a preset is loaded
    // pub preset_fields: BTreeMap<String, Value>,

    pub defaults_preset: Option<Preset>,
}

impl Default for OneTrickPluginParams {
    fn default() -> Self {
        Self {
            #[cfg(feature = "egui")]
            editor_state: EguiState::from_size(1024, 720),

            params: Vec::with_capacity(2048), // Hack: Reallocating the Vec will invalidate our unsafe pointers (DSP)

            fields: RwLock::new(BTreeMap::new()),

            // editor_fields_loaded: false, // Only read editor fields on the first load
            // editor_fields: BTreeMap::new(),
            // preset_fields: BTreeMap::new(),

            defaults_preset: None,
        }
    }
}

impl OneTrickPluginParams {
    pub fn set_defaults_preset(&mut self, preset: Preset) {
        self.defaults_preset = Some(preset);
    }
    fn get_defaults_bool(&self, name: &str, default_value: bool) -> bool {
        if let Some(preset) = &self.defaults_preset {
            if let Some(value) = preset.params.get(name) {
                if let Ok(value) = serde_json::from_value::<bool>(value.clone()) {
                    return value
                }
            }
        }
        default_value
    }
    fn get_defaults_int(&self, name: &str, default_value: i32) -> i32 {
        if let Some(preset) = &self.defaults_preset {
            if let Some(value) = preset.params.get(name) {
                if let Ok(value) = serde_json::from_value::<i32>(value.clone()) {
                    return value
                }
            }
    }
        default_value
    }
    fn get_defaults_float(&self, name: &str, default_value: f32) -> f32 {
        if let Some(preset) = &self.defaults_preset {
            if let Some(value) = preset.params.get(name) {
                if let Ok(value) = serde_json::from_value::<f32>(value.clone()) {
                    return value
                }
            }
    }
        default_value
    }

    /// Appends a parameter
    pub fn append(&mut self, param: ParamType, group: &str) -> usize {
        let export_id = match param {
            ParamType::BoolParam(ref p) => p.name(),
            ParamType::FloatParam(ref p) => p.name(),
            ParamType::IntParam(ref p) => p.name(),
        }
        .to_string();
        self.params.push((export_id, param, group.to_string()));
        self.params.len() - 1
    }

    /// Appends a bool parameter
    pub fn append_bool(&mut self, param: BoolParam, group: &str) -> usize {
        self.append(ParamType::BoolParam(param), group);
        self.params.len() - 1
    }

    /// Appends a float parameter
    pub fn append_float(&mut self, param: FloatParam, group: &str) -> usize {
        self.append(ParamType::FloatParam(param), group);
        self.params.len() - 1
    }

    /// Appends an int parameter
    pub fn append_int(&mut self, param: IntParam, group: &str) -> usize {
        self.append(ParamType::IntParam(param), group);
        self.params.len() - 1
    }

    /// Appends all parameters exported from a DSP, optionally prepending a group name
    pub fn append_dsp<DSP: OneTrickDSPGeneral>(&mut self, dsp: &mut DSP, group: &str) {
        for dsp_param in dsp.params_mut().params_mut().iter_mut() {
            if let Some(mut export_id) = dsp_param.meta("export") {
                if dsp_param.is_passive() {
                    continue;
                }
                // Allow groups/subgroups specified by parameters
                let mut group = group.to_string();
                if let Some(subgroup) = dsp_param.meta("group") {
                    group = if group.is_empty() {
                        subgroup.to_string()
                    } else {
                        format!("{} {}", group, subgroup)
                    };
                }
                if export_id.is_empty() {
                    export_id = dsp_param.name();
                }
                let export_id = if !group.is_empty() {
                    group.to_string() + " " + export_id
                } else {
                    export_id.to_string()
                };
                if let Some(existing_param_ptr) = self.param_ptr(&export_id) {
                    // Parameter already exists, so simply wire it up
                    dsp_param.set_plugin_param(existing_param_ptr);
                } else {
                    let default_export_type = if (dsp_param.step() - 1.0).abs() < 0.0001 {
                        "int"
                    } else {
                        "float"
                    };
                    let export_type = dsp_param.meta("type").unwrap_or(default_export_type);
                    match export_type {
                        "bool" => {
                            let default_value = self.get_defaults_bool(&export_id, dsp_param.init() > 0.5);
                            let mut base_param = BoolParam::new(export_id.to_string(), default_value)
                                .with_value_to_string(Arc::new(|v| {
                                    (if v { "on" } else { "off" }).to_string()
                                }))
                                .with_string_to_value(Arc::new(|s| {
                                    Some(s=="on")
                                }));

                            if let Some(enum_names) = dsp_param.meta("enum") {
                                let enum_names_v2s = enum_names.to_owned();
                                let enum_names_s2v = enum_names.to_owned();
                                base_param = base_param.with_value_to_string(Arc::new(move |v| {
                                    let names = enum_names_v2s.split(',');
                                    for (index, name) in names.into_iter().enumerate() {
                                        if v == (index != 0) {
                                            return name.to_string();
                                        }
                                    }
                                    "?".to_string()
                                })).with_string_to_value(Arc::new(move |s| {
                                    let names = enum_names_s2v.split(',');
                                    for (index, name) in names.into_iter().enumerate() {
                                        if name == s {
                                            return Some(index != 0);
                                        }
                                    }
                                    Some(false)
                                }));
                            }
    

                            let param = ParamType::BoolParam(base_param);

                            self.params
                                .push((export_id.to_string(), param, group.to_string()));
                            if let ParamType::BoolParam(param) =
                                &self.params[self.params.len() - 1].1
                            {
                                dsp_param.set_plugin_param(param.as_ptr());
                            }
                        }
                        "int"|"enum" => {
                            let default_value = self.get_defaults_int(&export_id, (dsp_param.init().round()) as i32);
                            let mut base_param = IntParam::new(
                                export_id.to_string(),
                                default_value,
                                IntRange::Linear {
                                    min: (dsp_param.min().round()) as i32,
                                    max: (dsp_param.max().round()) as i32,
                                },
                            )
                            .with_unit(dsp_param.unit_static());

                            if let Some(enum_names) = dsp_param.meta("enum") {
                                let enum_names_v2s = enum_names.to_owned();
                                let enum_names_s2v = enum_names.to_owned();
                                base_param = base_param.with_value_to_string(Arc::new(move |v| {
                                    let names = enum_names_v2s.split(',');
                                    for (index, name) in names.into_iter().enumerate() {
                                        if v == index as i32 {
                                            return name.to_string();
                                        }
                                    }
                                    "?".to_string()
                                })).with_string_to_value(Arc::new(move |s| {
                                    let names = enum_names_s2v.split(',');
                                    for (index, name) in names.into_iter().enumerate() {
                                        if name == s {
                                            return Some(index as i32);
                                        }
                                    }
                                    Some(0)
                                }));
                            }
                            let param = ParamType::IntParam(base_param);

                            self.params
                                .push((export_id.to_string(), param, group.to_string()));
                            if let ParamType::IntParam(param) =
                                &self.params[self.params.len() - 1].1
                            {
                                dsp_param.set_plugin_param(param.as_ptr());
                            }
                        }
                        _ => {
                            // "float"
                            /*
                                TODO: Check for Gain
                                [scale:log]
                                [scale:exp]
                                FloatRange::Skewed {
                                    min: util::db_to_gain(-30.0),
                                    max: util::db_to_gain(30.0),
                                    // This makes the range appear as if it was linear when displaying the values as
                                    // decibels
                                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                            },
                                */

                            
                            let range = match dsp_param.unit() {
                                "dB" | "db" => FloatRange::Skewed {
                                    min: dsp_param.min(),
                                    max: dsp_param.max(),
                                    factor: if dsp_param.min() < -50.0 && dsp_param.max() > 0.0 {
                                        0.5f32.log(
                                            //Volume-style gain, center at 0.0db
                                            (0.0 - dsp_param.min())
                                                / (dsp_param.max() - dsp_param.min()),
                                        )
                                    } else {
                                        // Simple gain, no skew
                                        1.0
                                    },
                                },
                                "Hz" | "hz" | "kHz" | "khz" | "mHz" | "mhz" | "rpm" => FloatRange::Skewed {
                                    min: dsp_param.min(),
                                    max: dsp_param.max(),
                                    factor: 0.5f32.log(
                                        (2.0f32.powf((dsp_param.min().log2() + dsp_param.max().log2())/2.0) - dsp_param.min())
                                            / (dsp_param.max() - dsp_param.min()),
                                    ),
                                },
                                _ => {
                                    let skew_factor = dsp_param.meta("skew").unwrap_or("0.0").parse::<f32>().ok().unwrap();
                                    if let Some(center) = dsp_param.meta("center") {
                                        let center = center.parse::<f32>().ok().unwrap();
                                        // SymmetricalSkewed is linear up until center, then skewed
                                        FloatRange::SymmetricalSkewed {
                                            min: dsp_param.min(),
                                            max: dsp_param.max(),
                                            center,
                                            factor: FloatRange::skew_factor(skew_factor)
                                        }
                                    } else if skew_factor != 0.0 {
                                        FloatRange::Skewed {
                                            min: dsp_param.min(),
                                            max: dsp_param.max(),
                                            factor: FloatRange::skew_factor(skew_factor)
                                        }
                                    } else {
                                        FloatRange::Linear {
                                            min: dsp_param.min(),
                                            max: dsp_param.max(),
                                        }
                                    }
                                }
                            };
                            let label_precision = if (dsp_param.max() - dsp_param.min()).abs() < 10.0 {
                                2
                            } else {
                                1
                            };
                            let min_value = dsp_param.min();

                            // We could skip string_to_value() here because we're only adjusting precision
                            // This shouldn't affect parsing, but we'll include it anyway for completeness.
                            let unit_s2v = dsp_param.unit_static();
                            let default_value = self.get_defaults_float(&export_id, dsp_param.init());
                            let mut base_param = FloatParam::new(export_id.to_string(), default_value, range)
                                .with_unit(dsp_param.unit_static())
                                .with_value_to_string(Arc::new(move |v| {
                                    format!("{value:.precision$}", precision = label_precision, value = v)
                                })).with_string_to_value(Arc::new(move |s| {
                                    s.trim().trim_end_matches(unit_s2v).parse().ok()
                                }));
                            if let Some(min_label) = dsp_param.meta("minlabel") {
                                let min_label_v2s = min_label.to_owned();
                                let min_label_s2v = min_label.to_owned();
                                let unit_s2v = dsp_param.unit_static();
                                base_param = base_param.with_value_to_string(Arc::new(move |v| {
                                    if v == min_value {
                                        min_label_v2s.to_string()
                                    } else {
                                        format!("{value:.precision$}", precision = label_precision, value = v)
                                    }
                                })).with_string_to_value(Arc::new(move |s| {
                                    let s = s.trim().trim_end_matches(unit_s2v);
                                    if s == min_label_s2v {
                                        return Some(min_value);
                                    }
                                    s.parse().ok()
                                }));
                            }
                            let param = ParamType::FloatParam(base_param);
                            self.params
                                .push((export_id.to_string(), param, group.to_string()));
                            if let ParamType::FloatParam(param) =
                                &self.params[self.params.len() - 1].1
                            {
                                dsp_param.set_plugin_param(param.as_ptr());
                            }
                        }
                    }
                }
            }
        }
    }

    /// Returns a pointer to a parameter
    pub fn param_ptr(&self, name: &str) -> Option<ParamPtr> {
        for (id, param, _group) in &self.params {
            match param {
                ParamType::BoolParam(p) => {
                    if id == name {
                        return Some(ParamPtr::BoolParam(p));
                    }
                }
                ParamType::FloatParam(p) => {
                    if id == name {
                        return Some(ParamPtr::FloatParam(p));
                    }
                }
                ParamType::IntParam(p) => {
                    if id == name {
                        return Some(ParamPtr::IntParam(p));
                    }
                }
            }
        }
        None
    }

    /// Returns a BoolParam by name
    pub fn param_bool(&self, name: &str) -> Option<&BoolParam> {
        for (id, param, _group) in &self.params {
            if let ParamType::BoolParam(p) = param {
                if id == name {
                    return Some(p);
                }
            }
        }
        None
    }

    /// Returns the index of a BoolParam by name
    pub fn param_bool_index(&self, name: &str) -> Option<usize> {
        for (index, (id, param, _group)) in self.params.iter().enumerate() {
            if let ParamType::BoolParam(_p) = param {
                if id == name {
                    return Some(index);
                }
            }
        }
        None
    }

    /// Returns a BoolParam by index
    pub fn param_bool_at(&self, index: usize) -> Option<&BoolParam> {
        let (_id, param, _group) = &self.params[index];
        if let ParamType::BoolParam(p) = param {
            Some(p)
        } else {
            None
        }
    }

    /// Returns a FloatParam by name
    pub fn param_float(&self, name: &str) -> Option<&FloatParam> {
        for (id, param, _group) in &self.params {
            if let ParamType::FloatParam(p) = param {
                if id == name {
                    return Some(p);
                }
            }
        }
        None
    }

    /// returns the index of a FloatParam by name
    pub fn param_float_index(&self, name: &str) -> Option<usize> {
        for (index, (id, param, _group)) in self.params.iter().enumerate() {
            if let ParamType::FloatParam(_p) = param {
                if id == name {
                    return Some(index);
                }
            }
        }
        None
    }

    /// Returns a FloatParam by index
    pub fn param_float_at(&self, index: usize) -> Option<&FloatParam> {
        let (_id, param, _group) = &self.params[index];
        if let ParamType::FloatParam(p) = param {
            Some(p)
        } else {
            None
        }
    }

    /// Returns an IntParam by name
    pub fn param_int(&self, name: &str) -> Option<&IntParam> {
        for (id, param, _group) in &self.params {
            if let ParamType::IntParam(p) = param {
                if id == name {
                    return Some(p);
                }
            }
        }
        None
    }

    /// Returns the index of an IntParam by name
    pub fn param_int_index(&self, name: &str) -> Option<usize> {
        for (index, (id, param, _group)) in self.params.iter().enumerate() {
            if let ParamType::IntParam(_p) = param {
                if id == name {
                    return Some(index);
                }
            }
        }
        None
    }

    /// Returns an IntParam by index
    pub fn param_int_at(&self, index: usize) -> Option<&IntParam> {
        let (_id, param, _group) = &self.params[index];
        if let ParamType::IntParam(p) = param {
            Some(p)
        } else {
            None
        }
    }

    /// Appends a FieldI32
    pub fn append_field_int(&self, name: &str, default_value: i32) -> Arc<FieldI32> {
        let mut fields = self.fields.write();
        let new_field = Arc::new(FieldI32::new(default_value));
        fields.insert(name.to_string(), FieldType::IntField(new_field.clone()));
        new_field
    }

    /// Appends a FieldString
    pub fn append_field_string(&self, name: &str, default_value: &str) -> Arc<FieldString> {
        let mut fields = self.fields.write();
        let new_field = Arc::new(FieldString::new(default_value.to_owned()));
        fields.insert(name.to_string(), FieldType::StringField(new_field.clone()));
        new_field
    }

    /// Returns an `Option<Arc<FieldI32>>` by name
    pub fn field_int(&self, name: &str) -> Option<Arc<FieldI32>> {
        let fields = self.fields.read();
        if let Some(FieldType::IntField(field)) = fields.get(name) {
            return Some(field.clone());
        }
        None
    }

    /// Returns an `Option<Arc<FieldString>>` by name
    pub fn field_string(&self, name: &str) -> Option<Arc<FieldString>> {
        let fields = self.fields.read();
        if let Some(FieldType::StringField(field)) = fields.get(name) {
            return Some(field.clone());
        }
        None
    }

}

unsafe impl Params for OneTrickPluginParams {
    fn param_map(&self) -> Vec<(String, ParamPtr, String)> {
        let mut result = vec![];

        for (id, param, group) in &self.params {
            let id = id.to_string().to_lowercase().replace(' ', "_");
            match param {
                ParamType::BoolParam(p) => {
                    result.push((id.to_string(), ParamPtr::BoolParam(p), group.to_string()));
                }
                ParamType::FloatParam(p) => {
                    result.push((id.to_string(), ParamPtr::FloatParam(p), group.to_string()));
                }
                ParamType::IntParam(p) => {
                    result.push((id.to_string(), ParamPtr::IntParam(p), group.to_string()));
                }
            }
        }

        result
    }


    fn serialize_fields(&self) -> BTreeMap<String, String> {
        // nih_log!("Serializing fields");

        let fields = self.fields.read();
        let mut result = BTreeMap::new();
        for (id, value) in fields.iter() {
            let serialized_value: String = match value {
                FieldType::StringField(v) => {
                    v.get()
                },
                FieldType::IntField(v) => {
                    serde_json::to_string(&(v.get())).unwrap()
                },
            };
            result.insert(id.clone(), serialized_value);
        }

        result
    }

    fn deserialize_fields(&self, serialized: &BTreeMap<String, String>) {
        // nih_log!("Deserializing fields");
        {
            let mut fields = self.fields.write();
            for (id, value) in serialized {
                if let Some(field) = fields.get_mut(id) {
                    match field {
                        FieldType::IntField(v) => {
                            if let Ok(Some(parsed)) = serde_json::from_str::<Option<i32>>(value) {
                                v.set(parsed);
                            }
                        },
                        FieldType::StringField(v) => { v.set(value); },
                    }
                }
            }
        }
    }

}
