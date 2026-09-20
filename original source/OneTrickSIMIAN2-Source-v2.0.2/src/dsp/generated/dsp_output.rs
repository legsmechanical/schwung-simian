// Postprocessed
/* ------------------------------------------------------------
author: "Oren Kurtz"
copyright: "Punk Labs LLC 2024"
license: "GPLv3-or-later"
name: "OneTrick SIMIAN"
version: "2.00"
Code generated with Faust 2.72.14 (https://faust.grame.fr)
Compilation options: -a arch.rs -lang rust -ct 0 -cn DSP_Output -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */
use faust_types::*;



fn DSP_Output_faustpower2_f(value: F32) -> F32 {
	return value * value;
}
mod ffi {
	use std::os::raw::{c_float};
	#[cfg_attr(not(target_os="windows"), link(name="m"))]
	extern {
		pub fn remainderf(from: c_float, to: c_float) -> c_float;
		pub fn rintf(val: c_float) -> c_float;
	}
}
fn remainder_f32(from: f32, to: f32) -> f32 {
	unsafe { ffi::remainderf(from, to) }
}
fn rint_f32(val: f32) -> f32 {
	unsafe { ffi::rintf(val) }
}

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct DSP_Output {
	fSampleRate: i32,
	fConst0: F32,
	fConst1: F32,
	fConst2: F32,
	fConst3: F32,
	fConst4: F32,
	fConst5: F32,
	fConst6: F32,
	fConst7: F32,
	fConst8: F32,
	fHslider0: F32,
	fConst9: F32,
	fConst10: F32,
	fConst11: F32,
	fConst12: F32,
	fConst13: F32,
	fConst14: F32,
	fRec18: [F32;2],
	fConst15: F32,
	fRec17: [F32;2],
	IOTA0: i32,
	fVec0: [F32;65536],
	fConst16: F32,
	iConst17: i32,
	fVec1: [F32;2048],
	iConst18: i32,
	fVec2: [F32;8192],
	iConst19: i32,
	fRec15: [F32;2],
	fConst20: F32,
	fConst21: F32,
	fConst22: F32,
	fRec22: [F32;2],
	fConst23: F32,
	fRec21: [F32;2],
	fVec3: [F32;32768],
	fConst24: F32,
	iConst25: i32,
	fVec4: [F32;4096],
	iConst26: i32,
	fRec19: [F32;2],
	fConst27: F32,
	fConst28: F32,
	fConst29: F32,
	fRec26: [F32;2],
	fConst30: F32,
	fRec25: [F32;2],
	fVec5: [F32;32768],
	fConst31: F32,
	iConst32: i32,
	fVec6: [F32;8192],
	iConst33: i32,
	fRec23: [F32;2],
	fConst34: F32,
	fConst35: F32,
	fConst36: F32,
	fRec30: [F32;2],
	fConst37: F32,
	fRec29: [F32;2],
	fVec7: [F32;32768],
	fConst38: F32,
	iConst39: i32,
	fVec8: [F32;4096],
	iConst40: i32,
	fRec27: [F32;2],
	fConst41: F32,
	fConst42: F32,
	fConst43: F32,
	fRec34: [F32;2],
	fConst44: F32,
	fRec33: [F32;2],
	fVec9: [F32;65536],
	fConst45: F32,
	iConst46: i32,
	fVec10: [F32;8192],
	iConst47: i32,
	fRec31: [F32;2],
	fConst48: F32,
	fConst49: F32,
	fConst50: F32,
	fRec38: [F32;2],
	fConst51: F32,
	fRec37: [F32;2],
	fVec11: [F32;65536],
	fConst52: F32,
	iConst53: i32,
	fVec12: [F32;8192],
	iConst54: i32,
	fRec35: [F32;2],
	fConst55: F32,
	fConst56: F32,
	fConst57: F32,
	fRec42: [F32;2],
	fConst58: F32,
	fRec41: [F32;2],
	fVec13: [F32;65536],
	fConst59: F32,
	iConst60: i32,
	fVec14: [F32;8192],
	iConst61: i32,
	fRec39: [F32;2],
	fConst62: F32,
	fConst63: F32,
	fConst64: F32,
	fRec46: [F32;2],
	fConst65: F32,
	fRec45: [F32;2],
	fVec15: [F32;65536],
	fConst66: F32,
	iConst67: i32,
	fVec16: [F32;4096],
	iConst68: i32,
	fRec43: [F32;2],
	fRec7: [F32;3],
	fRec8: [F32;3],
	fRec9: [F32;3],
	fRec10: [F32;3],
	fRec11: [F32;3],
	fRec12: [F32;3],
	fRec13: [F32;3],
	fRec14: [F32;3],
	fRec6: [F32;3],
	fHslider1: F32,
	fConst69: F32,
	fConst70: F32,
	fRec49: [F32;3],
	fConst71: F32,
	fRec48: [F32;2],
	iVec17: [i32;2],
	iConst72: i32,
	iRec50: [i32;2],
	fConst73: F32,
	fRec47: [F32;2],
	fButton0: F32,
	fVec18: [F32;2],
	fConst74: F32,
	fHslider2: F32,
	fRec51: [F32;2],
	fConst75: F32,
	fConst76: F32,
	fRec5: [F32;2],
	fConst77: F32,
	fConst78: F32,
	fRec4: [F32;2],
	fHslider3: F32,
	fRec52: [F32;2],
	fHslider4: F32,
	fRec53: [F32;2],
	iRec2: [i32;2],
	fRec3: [F32;2],
	fConst79: F32,
	fRec1: [F32;2],
	fVbargraph0: F32,
	iRec56: [i32;2],
	fRec57: [F32;2],
	fRec55: [F32;2],
	fVbargraph1: F32,
}

impl FaustDsp for DSP_Output {
	type T = F32;
		
	fn new() -> DSP_Output { 
		DSP_Output {
			fSampleRate: 0,
			fConst0: 0.0,
			fConst1: 0.0,
			fConst2: 0.0,
			fConst3: 0.0,
			fConst4: 0.0,
			fConst5: 0.0,
			fConst6: 0.0,
			fConst7: 0.0,
			fConst8: 0.0,
			fHslider0: 0.0,
			fConst9: 0.0,
			fConst10: 0.0,
			fConst11: 0.0,
			fConst12: 0.0,
			fConst13: 0.0,
			fConst14: 0.0,
			fRec18: [0.0;2],
			fConst15: 0.0,
			fRec17: [0.0;2],
			IOTA0: 0,
			fVec0: [0.0;65536],
			fConst16: 0.0,
			iConst17: 0,
			fVec1: [0.0;2048],
			iConst18: 0,
			fVec2: [0.0;8192],
			iConst19: 0,
			fRec15: [0.0;2],
			fConst20: 0.0,
			fConst21: 0.0,
			fConst22: 0.0,
			fRec22: [0.0;2],
			fConst23: 0.0,
			fRec21: [0.0;2],
			fVec3: [0.0;32768],
			fConst24: 0.0,
			iConst25: 0,
			fVec4: [0.0;4096],
			iConst26: 0,
			fRec19: [0.0;2],
			fConst27: 0.0,
			fConst28: 0.0,
			fConst29: 0.0,
			fRec26: [0.0;2],
			fConst30: 0.0,
			fRec25: [0.0;2],
			fVec5: [0.0;32768],
			fConst31: 0.0,
			iConst32: 0,
			fVec6: [0.0;8192],
			iConst33: 0,
			fRec23: [0.0;2],
			fConst34: 0.0,
			fConst35: 0.0,
			fConst36: 0.0,
			fRec30: [0.0;2],
			fConst37: 0.0,
			fRec29: [0.0;2],
			fVec7: [0.0;32768],
			fConst38: 0.0,
			iConst39: 0,
			fVec8: [0.0;4096],
			iConst40: 0,
			fRec27: [0.0;2],
			fConst41: 0.0,
			fConst42: 0.0,
			fConst43: 0.0,
			fRec34: [0.0;2],
			fConst44: 0.0,
			fRec33: [0.0;2],
			fVec9: [0.0;65536],
			fConst45: 0.0,
			iConst46: 0,
			fVec10: [0.0;8192],
			iConst47: 0,
			fRec31: [0.0;2],
			fConst48: 0.0,
			fConst49: 0.0,
			fConst50: 0.0,
			fRec38: [0.0;2],
			fConst51: 0.0,
			fRec37: [0.0;2],
			fVec11: [0.0;65536],
			fConst52: 0.0,
			iConst53: 0,
			fVec12: [0.0;8192],
			iConst54: 0,
			fRec35: [0.0;2],
			fConst55: 0.0,
			fConst56: 0.0,
			fConst57: 0.0,
			fRec42: [0.0;2],
			fConst58: 0.0,
			fRec41: [0.0;2],
			fVec13: [0.0;65536],
			fConst59: 0.0,
			iConst60: 0,
			fVec14: [0.0;8192],
			iConst61: 0,
			fRec39: [0.0;2],
			fConst62: 0.0,
			fConst63: 0.0,
			fConst64: 0.0,
			fRec46: [0.0;2],
			fConst65: 0.0,
			fRec45: [0.0;2],
			fVec15: [0.0;65536],
			fConst66: 0.0,
			iConst67: 0,
			fVec16: [0.0;4096],
			iConst68: 0,
			fRec43: [0.0;2],
			fRec7: [0.0;3],
			fRec8: [0.0;3],
			fRec9: [0.0;3],
			fRec10: [0.0;3],
			fRec11: [0.0;3],
			fRec12: [0.0;3],
			fRec13: [0.0;3],
			fRec14: [0.0;3],
			fRec6: [0.0;3],
			fHslider1: 0.0,
			fConst69: 0.0,
			fConst70: 0.0,
			fRec49: [0.0;3],
			fConst71: 0.0,
			fRec48: [0.0;2],
			iVec17: [0;2],
			iConst72: 0,
			iRec50: [0;2],
			fConst73: 0.0,
			fRec47: [0.0;2],
			fButton0: 0.0,
			fVec18: [0.0;2],
			fConst74: 0.0,
			fHslider2: 0.0,
			fRec51: [0.0;2],
			fConst75: 0.0,
			fConst76: 0.0,
			fRec5: [0.0;2],
			fConst77: 0.0,
			fConst78: 0.0,
			fRec4: [0.0;2],
			fHslider3: 0.0,
			fRec52: [0.0;2],
			fHslider4: 0.0,
			fRec53: [0.0;2],
			iRec2: [0;2],
			fRec3: [0.0;2],
			fConst79: 0.0,
			fRec1: [0.0;2],
			fVbargraph0: 0.0,
			iRec56: [0;2],
			fRec57: [0.0;2],
			fRec55: [0.0;2],
			fVbargraph1: 0.0,
		}
	}
	fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("analyzers.lib/amp_follower_ar:author", r"Jonatan Liljedahl, revised by Romain Michon");
		m.declare("analyzers.lib/name", r"Faust Analyzer Library");
		m.declare("analyzers.lib/version", r"1.2.0");
		m.declare("author", r"Oren Kurtz");
		m.declare("basics.lib/name", r"Faust Basic Element Library");
		m.declare("basics.lib/peakholder:author", r"Dario Sanfilippo");
		m.declare("basics.lib/peakholder:copyright", r"Copyright (C) 2022 Dario Sanfilippo <sanfilippo.dario@gmail.com>");
		m.declare("basics.lib/peakholder:license", r"MIT-style STK-4.3 license");
		m.declare("basics.lib/tabulateNd", r"Copyright (C) 2023 Bart Brouns <bart@magnetophon.nl>");
		m.declare("basics.lib/version", r"1.15.0");
		m.declare("compile_options", r"-a arch.rs -lang rust -ct 0 -cn DSP_Output -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m.declare("compressors.lib/compression_gain_mono:author", r"Julius O. Smith III");
		m.declare("compressors.lib/compression_gain_mono:copyright", r"Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compression_gain_mono:license", r"MIT-style STK-4.3 license");
		m.declare("compressors.lib/compressor_stereo:author", r"Julius O. Smith III");
		m.declare("compressors.lib/compressor_stereo:copyright", r"Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("compressors.lib/compressor_stereo:license", r"MIT-style STK-4.3 license");
		m.declare("compressors.lib/name", r"Faust Compressor Effect Library");
		m.declare("compressors.lib/version", r"1.6.0");
		m.declare("copyright", r"Punk Labs LLC 2024");
		m.declare("delays.lib/name", r"Faust Delay Library");
		m.declare("delays.lib/version", r"1.1.0");
		m.declare("filename", r"output.dsp");
		m.declare("filters.lib/allpass_comb:author", r"Julius O. Smith III");
		m.declare("filters.lib/allpass_comb:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/allpass_comb:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/fir:author", r"Julius O. Smith III");
		m.declare("filters.lib/fir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/fir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/highpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/highpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:author", r"Julius O. Smith III");
		m.declare("filters.lib/iir:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/iir:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/lowpass0_highpass1:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:author", r"Julius O. Smith III");
		m.declare("filters.lib/lowpass:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/lowpass:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/name", r"Faust Filters Library");
		m.declare("filters.lib/tf1:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf1s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf1s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf1s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/tf2s:author", r"Julius O. Smith III");
		m.declare("filters.lib/tf2s:copyright", r"Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m.declare("filters.lib/tf2s:license", r"MIT-style STK-4.3 license");
		m.declare("filters.lib/version", r"1.3.0");
		m.declare("interpolators.lib/interpolate_linear:author", r"Stéphane Letz");
		m.declare("interpolators.lib/interpolate_linear:licence", r"MIT");
		m.declare("interpolators.lib/name", r"Faust Interpolator Library");
		m.declare("interpolators.lib/remap:author", r"David Braun");
		m.declare("interpolators.lib/version", r"1.3.1");
		m.declare("license", r"GPLv3-or-later");
		m.declare("maths.lib/author", r"GRAME");
		m.declare("maths.lib/copyright", r"GRAME");
		m.declare("maths.lib/license", r"LGPL with exception");
		m.declare("maths.lib/name", r"Faust Math Library");
		m.declare("maths.lib/version", r"2.8.0");
		m.declare("misceffects.lib/gate_gain_mono:author", r"Julius O. Smith III");
		m.declare("misceffects.lib/gate_gain_mono:license", r"STK-4.3");
		m.declare("misceffects.lib/gate_stereo:author", r"Julius O. Smith III");
		m.declare("misceffects.lib/gate_stereo:license", r"STK-4.3");
		m.declare("misceffects.lib/name", r"Misc Effects Library");
		m.declare("misceffects.lib/version", r"2.4.0");
		m.declare("name", r"OneTrick SIMIAN");
		m.declare("onetrick.lib/copyright", r"Copyright (c) 2023 Punk Labs LLC");
		m.declare("onetrick.lib/license", r"GPLv3 (or later)");
		m.declare("onetrick.lib/name", r"OneTrick DSP Library");
		m.declare("platform.lib/name", r"Generic Platform Library");
		m.declare("platform.lib/version", r"1.3.0");
		m.declare("reverbs.lib/name", r"Faust Reverb Library");
		m.declare("reverbs.lib/version", r"1.2.1");
		m.declare("routes.lib/hadamard:author", r"Remy Muller, revised by Romain Michon");
		m.declare("routes.lib/name", r"Faust Signal Routing Library");
		m.declare("routes.lib/version", r"1.2.0");
		m.declare("shared.lib/author", r"Oren Kurtz");
		m.declare("shared.lib/copyright", r"Punk Labs LLC 2024");
		m.declare("shared.lib/license", r"GPLv3-or-later");
		m.declare("shared.lib/name", r"OneTrick SIMIAN");
		m.declare("shared.lib/options", r"[midi:on]");
		m.declare("shared.lib/version", r"2.00");
		m.declare("signals.lib/name", r"Faust Signal Routing Library");
		m.declare("signals.lib/onePoleSwitching:author", r"Jonatan Liljedahl, revised by Dario Sanfilippo");
		m.declare("signals.lib/onePoleSwitching:licence", r"STK-4.3");
		m.declare("signals.lib/version", r"1.5.0");
		m.declare("version", r"2.00");
	}

	fn get_sample_rate(&self) -> i32 {
		return self.fSampleRate;
	}
	fn get_num_inputs(&self) -> i32 {
		return 3;
	}
	fn get_num_outputs(&self) -> i32 {
		return 2;
	}
	
	fn class_init(sample_rate: i32) {
	}
	fn instance_reset_params(&mut self) {
		self.fHslider0 = 5e+01;
		self.fHslider1 = -1e+02;
		self.fButton0 = 0.0;
		self.fHslider2 = -6.0;
		self.fHslider3 = 0.0;
		self.fHslider4 = 0.0;
	}
	fn instance_clear(&mut self) {
		for l0 in 0..2 {
			self.fRec18[l0 as usize] = 0.0;
		}
		for l1 in 0..2 {
			self.fRec17[l1 as usize] = 0.0;
		}
		self.IOTA0 = 0;
		for l2 in 0..65536 {
			self.fVec0[l2 as usize] = 0.0;
		}
		for l3 in 0..2048 {
			self.fVec1[l3 as usize] = 0.0;
		}
		for l4 in 0..8192 {
			self.fVec2[l4 as usize] = 0.0;
		}
		for l5 in 0..2 {
			self.fRec15[l5 as usize] = 0.0;
		}
		for l6 in 0..2 {
			self.fRec22[l6 as usize] = 0.0;
		}
		for l7 in 0..2 {
			self.fRec21[l7 as usize] = 0.0;
		}
		for l8 in 0..32768 {
			self.fVec3[l8 as usize] = 0.0;
		}
		for l9 in 0..4096 {
			self.fVec4[l9 as usize] = 0.0;
		}
		for l10 in 0..2 {
			self.fRec19[l10 as usize] = 0.0;
		}
		for l11 in 0..2 {
			self.fRec26[l11 as usize] = 0.0;
		}
		for l12 in 0..2 {
			self.fRec25[l12 as usize] = 0.0;
		}
		for l13 in 0..32768 {
			self.fVec5[l13 as usize] = 0.0;
		}
		for l14 in 0..8192 {
			self.fVec6[l14 as usize] = 0.0;
		}
		for l15 in 0..2 {
			self.fRec23[l15 as usize] = 0.0;
		}
		for l16 in 0..2 {
			self.fRec30[l16 as usize] = 0.0;
		}
		for l17 in 0..2 {
			self.fRec29[l17 as usize] = 0.0;
		}
		for l18 in 0..32768 {
			self.fVec7[l18 as usize] = 0.0;
		}
		for l19 in 0..4096 {
			self.fVec8[l19 as usize] = 0.0;
		}
		for l20 in 0..2 {
			self.fRec27[l20 as usize] = 0.0;
		}
		for l21 in 0..2 {
			self.fRec34[l21 as usize] = 0.0;
		}
		for l22 in 0..2 {
			self.fRec33[l22 as usize] = 0.0;
		}
		for l23 in 0..65536 {
			self.fVec9[l23 as usize] = 0.0;
		}
		for l24 in 0..8192 {
			self.fVec10[l24 as usize] = 0.0;
		}
		for l25 in 0..2 {
			self.fRec31[l25 as usize] = 0.0;
		}
		for l26 in 0..2 {
			self.fRec38[l26 as usize] = 0.0;
		}
		for l27 in 0..2 {
			self.fRec37[l27 as usize] = 0.0;
		}
		for l28 in 0..65536 {
			self.fVec11[l28 as usize] = 0.0;
		}
		for l29 in 0..8192 {
			self.fVec12[l29 as usize] = 0.0;
		}
		for l30 in 0..2 {
			self.fRec35[l30 as usize] = 0.0;
		}
		for l31 in 0..2 {
			self.fRec42[l31 as usize] = 0.0;
		}
		for l32 in 0..2 {
			self.fRec41[l32 as usize] = 0.0;
		}
		for l33 in 0..65536 {
			self.fVec13[l33 as usize] = 0.0;
		}
		for l34 in 0..8192 {
			self.fVec14[l34 as usize] = 0.0;
		}
		for l35 in 0..2 {
			self.fRec39[l35 as usize] = 0.0;
		}
		for l36 in 0..2 {
			self.fRec46[l36 as usize] = 0.0;
		}
		for l37 in 0..2 {
			self.fRec45[l37 as usize] = 0.0;
		}
		for l38 in 0..65536 {
			self.fVec15[l38 as usize] = 0.0;
		}
		for l39 in 0..4096 {
			self.fVec16[l39 as usize] = 0.0;
		}
		for l40 in 0..2 {
			self.fRec43[l40 as usize] = 0.0;
		}
		for l41 in 0..3 {
			self.fRec7[l41 as usize] = 0.0;
		}
		for l42 in 0..3 {
			self.fRec8[l42 as usize] = 0.0;
		}
		for l43 in 0..3 {
			self.fRec9[l43 as usize] = 0.0;
		}
		for l44 in 0..3 {
			self.fRec10[l44 as usize] = 0.0;
		}
		for l45 in 0..3 {
			self.fRec11[l45 as usize] = 0.0;
		}
		for l46 in 0..3 {
			self.fRec12[l46 as usize] = 0.0;
		}
		for l47 in 0..3 {
			self.fRec13[l47 as usize] = 0.0;
		}
		for l48 in 0..3 {
			self.fRec14[l48 as usize] = 0.0;
		}
		for l49 in 0..3 {
			self.fRec6[l49 as usize] = 0.0;
		}
		for l50 in 0..3 {
			self.fRec49[l50 as usize] = 0.0;
		}
		for l51 in 0..2 {
			self.fRec48[l51 as usize] = 0.0;
		}
		for l52 in 0..2 {
			self.iVec17[l52 as usize] = 0;
		}
		for l53 in 0..2 {
			self.iRec50[l53 as usize] = 0;
		}
		for l54 in 0..2 {
			self.fRec47[l54 as usize] = 0.0;
		}
		for l55 in 0..2 {
			self.fVec18[l55 as usize] = 0.0;
		}
		for l56 in 0..2 {
			self.fRec51[l56 as usize] = 0.0;
		}
		for l57 in 0..2 {
			self.fRec5[l57 as usize] = 0.0;
		}
		for l58 in 0..2 {
			self.fRec4[l58 as usize] = 0.0;
		}
		for l59 in 0..2 {
			self.fRec52[l59 as usize] = 0.0;
		}
		for l60 in 0..2 {
			self.fRec53[l60 as usize] = 0.0;
		}
		for l61 in 0..2 {
			self.iRec2[l61 as usize] = 0;
		}
		for l62 in 0..2 {
			self.fRec3[l62 as usize] = 0.0;
		}
		for l63 in 0..2 {
			self.fRec1[l63 as usize] = 0.0;
		}
		for l64 in 0..2 {
			self.iRec56[l64 as usize] = 0;
		}
		for l65 in 0..2 {
			self.fRec57[l65 as usize] = 0.0;
		}
		for l66 in 0..2 {
			self.fRec55[l66 as usize] = 0.0;
		}
	}
	fn instance_constants(&mut self, sample_rate: i32) {
		self.fSampleRate = sample_rate;
		self.fConst0 = F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
		self.fConst1 = 0.05 * self.fConst0;
		self.fConst2 = F32::tan(628.31854 / self.fConst0);
		self.fConst3 = DSP_Output_faustpower2_f(self.fConst2);
		self.fConst4 = 2.0 * (1.0 - 1.0 / self.fConst3);
		self.fConst5 = 1.0 / self.fConst2;
		self.fConst6 = (self.fConst5 + -1.4142135) / self.fConst2 + 1.0;
		self.fConst7 = (self.fConst5 + 1.4142135) / self.fConst2 + 1.0;
		self.fConst8 = 1.0 / self.fConst7;
		self.fConst9 = F32::floor(0.174713 * self.fConst0 + 0.5);
		self.fConst10 = self.fConst9 / self.fConst0;
		self.fConst11 = 8.913233 * self.fConst10;
		self.fConst12 = F32::cos(37699.113 / self.fConst0);
		self.fConst13 = 1.0 - self.fConst5;
		self.fConst14 = 1.0 / (self.fConst5 + 1.0);
		self.fConst15 = 6.9077554 * self.fConst10;
		self.fConst16 = F32::floor(0.022904 * self.fConst0 + 0.5);
		self.iConst17 = (F32::min(32768.0, F32::max(0.0, self.fConst9 - self.fConst16))) as i32;
		self.iConst18 = (F32::min(8192.0, F32::max(0.0, 0.01 * self.fConst0))) as i32;
		self.iConst19 = (F32::min(8192.0, F32::max(0.0, self.fConst16 + -1.0))) as i32;
		self.fConst20 = F32::floor(0.153129 * self.fConst0 + 0.5);
		self.fConst21 = self.fConst20 / self.fConst0;
		self.fConst22 = 8.913233 * self.fConst21;
		self.fConst23 = 6.9077554 * self.fConst21;
		self.fConst24 = F32::floor(0.020346 * self.fConst0 + 0.5);
		self.iConst25 = (F32::min(32768.0, F32::max(0.0, self.fConst20 - self.fConst24))) as i32;
		self.iConst26 = (F32::min(4096.0, F32::max(0.0, self.fConst24 + -1.0))) as i32;
		self.fConst27 = F32::floor(0.127837 * self.fConst0 + 0.5);
		self.fConst28 = self.fConst27 / self.fConst0;
		self.fConst29 = 8.913233 * self.fConst28;
		self.fConst30 = 6.9077554 * self.fConst28;
		self.fConst31 = F32::floor(0.031604 * self.fConst0 + 0.5);
		self.iConst32 = (F32::min(32768.0, F32::max(0.0, self.fConst27 - self.fConst31))) as i32;
		self.iConst33 = (F32::min(8192.0, F32::max(0.0, self.fConst31 + -1.0))) as i32;
		self.fConst34 = F32::floor(0.125 * self.fConst0 + 0.5);
		self.fConst35 = self.fConst34 / self.fConst0;
		self.fConst36 = 8.913233 * self.fConst35;
		self.fConst37 = 6.9077554 * self.fConst35;
		self.fConst38 = F32::floor(0.013458 * self.fConst0 + 0.5);
		self.iConst39 = (F32::min(32768.0, F32::max(0.0, self.fConst34 - self.fConst38))) as i32;
		self.iConst40 = (F32::min(4096.0, F32::max(0.0, self.fConst38 + -1.0))) as i32;
		self.fConst41 = F32::floor(0.210389 * self.fConst0 + 0.5);
		self.fConst42 = self.fConst41 / self.fConst0;
		self.fConst43 = 8.913233 * self.fConst42;
		self.fConst44 = 6.9077554 * self.fConst42;
		self.fConst45 = F32::floor(0.024421 * self.fConst0 + 0.5);
		self.iConst46 = (F32::min(65536.0, F32::max(0.0, self.fConst41 - self.fConst45))) as i32;
		self.iConst47 = (F32::min(8192.0, F32::max(0.0, self.fConst45 + -1.0))) as i32;
		self.fConst48 = F32::floor(0.192303 * self.fConst0 + 0.5);
		self.fConst49 = self.fConst48 / self.fConst0;
		self.fConst50 = 8.913233 * self.fConst49;
		self.fConst51 = 6.9077554 * self.fConst49;
		self.fConst52 = F32::floor(0.029291 * self.fConst0 + 0.5);
		self.iConst53 = (F32::min(32768.0, F32::max(0.0, self.fConst48 - self.fConst52))) as i32;
		self.iConst54 = (F32::min(8192.0, F32::max(0.0, self.fConst52 + -1.0))) as i32;
		self.fConst55 = F32::floor(0.256891 * self.fConst0 + 0.5);
		self.fConst56 = self.fConst55 / self.fConst0;
		self.fConst57 = 8.913233 * self.fConst56;
		self.fConst58 = 6.9077554 * self.fConst56;
		self.fConst59 = F32::floor(0.027333 * self.fConst0 + 0.5);
		self.iConst60 = (F32::min(65536.0, F32::max(0.0, self.fConst55 - self.fConst59))) as i32;
		self.iConst61 = (F32::min(8192.0, F32::max(0.0, self.fConst59 + -1.0))) as i32;
		self.fConst62 = F32::floor(0.219991 * self.fConst0 + 0.5);
		self.fConst63 = self.fConst62 / self.fConst0;
		self.fConst64 = 8.913233 * self.fConst63;
		self.fConst65 = 6.9077554 * self.fConst63;
		self.fConst66 = F32::floor(0.019123 * self.fConst0 + 0.5);
		self.iConst67 = (F32::min(65536.0, F32::max(0.0, self.fConst62 - self.fConst66))) as i32;
		self.iConst68 = (F32::min(4096.0, F32::max(0.0, self.fConst66 + -1.0))) as i32;
		self.fConst69 = F32::exp(-(1e+04 / self.fConst0));
		self.fConst70 = 1.4125376 / (self.fConst3 * self.fConst7);
		self.fConst71 = 1.0 - self.fConst69;
		self.iConst72 = (0.1 * self.fConst0) as i32;
		self.fConst73 = F32::exp(-(5e+01 / self.fConst0));
		self.fConst74 = F32::exp(-(1e+02 / self.fConst0));
		self.fConst75 = F32::exp(-(2e+01 / self.fConst0));
		self.fConst76 = F32::exp(-(5e+04 / self.fConst0));
		self.fConst77 = F32::exp(-(1e+05 / self.fConst0));
		self.fConst78 = 0.95 * (1.0 - self.fConst77);
		self.fConst79 = 1.0 - self.fConst75;
	}
	fn instance_init(&mut self, sample_rate: i32) {
		self.instance_constants(sample_rate);
		self.instance_reset_params();
		self.instance_clear();
	}
	fn init(&mut self, sample_rate: i32) {
		DSP_Output::class_init(sample_rate);
		self.instance_init(sample_rate);
	}
	
	fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
		Self::build_user_interface_static(ui_interface);
	}
	
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) {
		ui_interface.open_vertical_box("OneTrick SIMIAN");
		ui_interface.open_tab_box("Globals");
		ui_interface.open_vertical_box("Globals");
		ui_interface.declare(Some(ParamIndex(0)), "0", "");
		ui_interface.declare(Some(ParamIndex(0)), "export", "Gain");
		ui_interface.declare(Some(ParamIndex(0)), "unit", "db");
		ui_interface.add_horizontal_slider("Gain_Global", ParamIndex(0), 0.0, -1e+02, 6.0, 0.1);
		ui_interface.declare(Some(ParamIndex(1)), "1", "");
		ui_interface.declare(Some(ParamIndex(1)), "export", "Drive");
		ui_interface.declare(Some(ParamIndex(1)), "unit", "db");
		ui_interface.add_horizontal_slider("Drive_Global", ParamIndex(1), -6.0, -1e+02, 6.0, 0.1);
		ui_interface.declare(Some(ParamIndex(2)), "2", "");
		ui_interface.declare(Some(ParamIndex(2)), "export", "Reverb Size");
		ui_interface.declare(Some(ParamIndex(2)), "unit", "%");
		ui_interface.add_horizontal_slider("ReverbSize_Global", ParamIndex(2), 5e+01, 0.0, 1e+02, 0.01);
		ui_interface.declare(Some(ParamIndex(3)), "3", "");
		ui_interface.declare(Some(ParamIndex(3)), "export", "Reverb Gate");
		ui_interface.declare(Some(ParamIndex(3)), "unit", "db");
		ui_interface.add_horizontal_slider("ReverbGate_Global", ParamIndex(3), -1e+02, -1e+02, 0.0, 0.01);
		ui_interface.declare(Some(ParamIndex(4)), "5", "");
		ui_interface.declare(Some(ParamIndex(4)), "export", "Saturation");
		ui_interface.declare(Some(ParamIndex(4)), "unit", "%");
		ui_interface.add_horizontal_slider("Saturation_Global", ParamIndex(4), 0.0, 0.0, 1e+02, 0.01);
		ui_interface.close_box();
		ui_interface.close_box();
		ui_interface.add_vertical_bargraph("VuMeterLeft", ParamIndex(5), -4e+01, 0.0);
		ui_interface.add_vertical_bargraph("VuMeterRight", ParamIndex(6), -4e+01, 0.0);
		ui_interface.add_button("WakeUp", ParamIndex(7));
		ui_interface.close_box();
	}
	
	fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
		match param.0 {
			7 => Some(self.fButton0),
			2 => Some(self.fHslider0),
			3 => Some(self.fHslider1),
			1 => Some(self.fHslider2),
			4 => Some(self.fHslider3),
			0 => Some(self.fHslider4),
			5 => Some(self.fVbargraph0),
			6 => Some(self.fVbargraph1),
			_ => None,
		}
	}
	
	fn set_param(&mut self, param: ParamIndex, value: Self::T) {
		match param.0 {
			7 => { self.fButton0 = value }
			2 => { self.fHslider0 = value }
			3 => { self.fHslider1 = value }
			1 => { self.fHslider2 = value }
			4 => { self.fHslider3 = value }
			0 => { self.fHslider4 = value }
			5 => { self.fVbargraph0 = value }
			6 => { self.fVbargraph1 = value }
			_ => {}
		}
	}
	
	fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut[&mut[Self::T]]) {
		let (inputs0, inputs1, inputs2) = if let [inputs0, inputs1, inputs2, ..] = inputs {
			let inputs0 = inputs0[..count as usize].iter();
			let inputs1 = inputs1[..count as usize].iter();
			let inputs2 = inputs2[..count as usize].iter();
			(inputs0, inputs1, inputs2)
		} else {
			panic!("wrong number of inputs");
		};
		let (outputs0, outputs1) = if let [outputs0, outputs1, ..] = outputs {
			let outputs0 = outputs0[..count as usize].iter_mut();
			let outputs1 = outputs1[..count as usize].iter_mut();
			(outputs0, outputs1)
		} else {
			panic!("wrong number of outputs");
		};
		let mut fSlow0: F32 = 0.015 * self.fHslider0 + 0.5;
		let mut fSlow1: F32 = F32::exp(-(self.fConst11 / fSlow0));
		let mut fSlow2: F32 = DSP_Output_faustpower2_f(fSlow1);
		let mut fSlow3: F32 = 1.0 - fSlow2;
		let mut fSlow4: F32 = 1.0 - self.fConst12 * fSlow2;
		let mut fSlow5: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow4) / DSP_Output_faustpower2_f(fSlow3) + -1.0));
		let mut fSlow6: F32 = fSlow4 / fSlow3;
		let mut fSlow7: F32 = fSlow6 - fSlow5;
		let mut fSlow8: F32 = F32::exp(-(self.fConst15 / fSlow0)) / fSlow1 + -1.0;
		let mut fSlow9: F32 = fSlow1 * (fSlow5 + (1.0 - fSlow6));
		let mut fSlow10: F32 = F32::exp(-(self.fConst22 / fSlow0));
		let mut fSlow11: F32 = DSP_Output_faustpower2_f(fSlow10);
		let mut fSlow12: F32 = 1.0 - fSlow11;
		let mut fSlow13: F32 = 1.0 - self.fConst12 * fSlow11;
		let mut fSlow14: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow13) / DSP_Output_faustpower2_f(fSlow12) + -1.0));
		let mut fSlow15: F32 = fSlow13 / fSlow12;
		let mut fSlow16: F32 = fSlow15 - fSlow14;
		let mut fSlow17: F32 = F32::exp(-(self.fConst23 / fSlow0)) / fSlow10 + -1.0;
		let mut fSlow18: F32 = fSlow10 * (fSlow14 + (1.0 - fSlow15));
		let mut fSlow19: F32 = F32::exp(-(self.fConst29 / fSlow0));
		let mut fSlow20: F32 = DSP_Output_faustpower2_f(fSlow19);
		let mut fSlow21: F32 = 1.0 - fSlow20;
		let mut fSlow22: F32 = 1.0 - self.fConst12 * fSlow20;
		let mut fSlow23: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow22) / DSP_Output_faustpower2_f(fSlow21) + -1.0));
		let mut fSlow24: F32 = fSlow22 / fSlow21;
		let mut fSlow25: F32 = fSlow24 - fSlow23;
		let mut fSlow26: F32 = F32::exp(-(self.fConst30 / fSlow0)) / fSlow19 + -1.0;
		let mut fSlow27: F32 = fSlow19 * (fSlow23 + (1.0 - fSlow24));
		let mut fSlow28: F32 = F32::exp(-(self.fConst36 / fSlow0));
		let mut fSlow29: F32 = DSP_Output_faustpower2_f(fSlow28);
		let mut fSlow30: F32 = 1.0 - fSlow29;
		let mut fSlow31: F32 = 1.0 - self.fConst12 * fSlow29;
		let mut fSlow32: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow31) / DSP_Output_faustpower2_f(fSlow30) + -1.0));
		let mut fSlow33: F32 = fSlow31 / fSlow30;
		let mut fSlow34: F32 = fSlow33 - fSlow32;
		let mut fSlow35: F32 = F32::exp(-(self.fConst37 / fSlow0)) / fSlow28 + -1.0;
		let mut fSlow36: F32 = fSlow28 * (fSlow32 + (1.0 - fSlow33));
		let mut fSlow37: F32 = F32::exp(-(self.fConst43 / fSlow0));
		let mut fSlow38: F32 = DSP_Output_faustpower2_f(fSlow37);
		let mut fSlow39: F32 = 1.0 - fSlow38;
		let mut fSlow40: F32 = 1.0 - self.fConst12 * fSlow38;
		let mut fSlow41: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow40) / DSP_Output_faustpower2_f(fSlow39) + -1.0));
		let mut fSlow42: F32 = fSlow40 / fSlow39;
		let mut fSlow43: F32 = fSlow42 - fSlow41;
		let mut fSlow44: F32 = F32::exp(-(self.fConst44 / fSlow0)) / fSlow37 + -1.0;
		let mut fSlow45: F32 = fSlow37 * (fSlow41 + (1.0 - fSlow42));
		let mut fSlow46: F32 = F32::exp(-(self.fConst50 / fSlow0));
		let mut fSlow47: F32 = DSP_Output_faustpower2_f(fSlow46);
		let mut fSlow48: F32 = 1.0 - fSlow47;
		let mut fSlow49: F32 = 1.0 - self.fConst12 * fSlow47;
		let mut fSlow50: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow49) / DSP_Output_faustpower2_f(fSlow48) + -1.0));
		let mut fSlow51: F32 = fSlow49 / fSlow48;
		let mut fSlow52: F32 = fSlow51 - fSlow50;
		let mut fSlow53: F32 = F32::exp(-(self.fConst51 / fSlow0)) / fSlow46 + -1.0;
		let mut fSlow54: F32 = fSlow46 * (fSlow50 + (1.0 - fSlow51));
		let mut fSlow55: F32 = F32::exp(-(self.fConst57 / fSlow0));
		let mut fSlow56: F32 = DSP_Output_faustpower2_f(fSlow55);
		let mut fSlow57: F32 = 1.0 - fSlow56;
		let mut fSlow58: F32 = 1.0 - self.fConst12 * fSlow56;
		let mut fSlow59: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow58) / DSP_Output_faustpower2_f(fSlow57) + -1.0));
		let mut fSlow60: F32 = fSlow58 / fSlow57;
		let mut fSlow61: F32 = fSlow60 - fSlow59;
		let mut fSlow62: F32 = F32::exp(-(self.fConst58 / fSlow0)) / fSlow55 + -1.0;
		let mut fSlow63: F32 = fSlow55 * (fSlow59 + (1.0 - fSlow60));
		let mut fSlow64: F32 = F32::exp(-(self.fConst64 / fSlow0));
		let mut fSlow65: F32 = DSP_Output_faustpower2_f(fSlow64);
		let mut fSlow66: F32 = 1.0 - fSlow65;
		let mut fSlow67: F32 = 1.0 - self.fConst12 * fSlow65;
		let mut fSlow68: F32 = F32::sqrt(F32::max(0.0, DSP_Output_faustpower2_f(fSlow67) / DSP_Output_faustpower2_f(fSlow66) + -1.0));
		let mut fSlow69: F32 = fSlow67 / fSlow66;
		let mut fSlow70: F32 = fSlow69 - fSlow68;
		let mut fSlow71: F32 = F32::exp(-(self.fConst65 / fSlow0)) / fSlow64 + -1.0;
		let mut fSlow72: F32 = fSlow64 * (fSlow68 + (1.0 - fSlow69));
		let mut fSlow73: F32 = F32::powf(1e+01, 0.05 * self.fHslider1);
		let mut fSlow74: F32 = self.fButton0;
		let mut fSlow75: F32 = F32::powf(1e+01, 0.05 * F32::max(-1e+02, self.fHslider2));
		let mut fSlow76: F32 = 0.01 * self.fHslider3;
		let mut fSlow77: F32 = F32::powf(1e+01, 0.05 * F32::max(-1e+02, self.fHslider4));
		let zipped_iterators = inputs0.zip(inputs1).zip(inputs2).zip(outputs0).zip(outputs1);
		for ((((input0, input1), input2), output0), output1) in zipped_iterators {
			self.fRec18[0] = -(self.fConst14 * (self.fConst13 * self.fRec18[1] - (self.fRec11[1] + self.fRec11[2])));
			self.fRec17[0] = fSlow9 * (self.fRec11[1] + fSlow8 * self.fRec18[0]) + fSlow7 * self.fRec17[1];
			self.fVec0[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec17[0] + 1e-20;
			self.fVec1[(self.IOTA0 & 2047) as usize] = *input2;
			let mut fTemp0: F32 = 0.3 * self.fVec1[((i32::wrapping_sub(self.IOTA0, self.iConst18)) & 2047) as usize];
			let mut fTemp1: F32 = fTemp0 + self.fVec0[((i32::wrapping_sub(self.IOTA0, self.iConst17)) & 65535) as usize] - 0.6 * self.fRec15[1];
			self.fVec2[(self.IOTA0 & 8191) as usize] = fTemp1;
			self.fRec15[0] = self.fVec2[((i32::wrapping_sub(self.IOTA0, self.iConst19)) & 8191) as usize];
			let mut fRec16: F32 = 0.6 * fTemp1;
			self.fRec22[0] = -(self.fConst14 * (self.fConst13 * self.fRec22[1] - (self.fRec7[1] + self.fRec7[2])));
			self.fRec21[0] = fSlow18 * (self.fRec7[1] + fSlow17 * self.fRec22[0]) + fSlow16 * self.fRec21[1];
			self.fVec3[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec21[0] + 1e-20;
			let mut fTemp2: F32 = self.fVec3[((i32::wrapping_sub(self.IOTA0, self.iConst25)) & 32767) as usize] + fTemp0 - 0.6 * self.fRec19[1];
			self.fVec4[(self.IOTA0 & 4095) as usize] = fTemp2;
			self.fRec19[0] = self.fVec4[((i32::wrapping_sub(self.IOTA0, self.iConst26)) & 4095) as usize];
			let mut fRec20: F32 = 0.6 * fTemp2;
			let mut fTemp3: F32 = fRec20 + fRec16;
			self.fRec26[0] = -(self.fConst14 * (self.fConst13 * self.fRec26[1] - (self.fRec9[1] + self.fRec9[2])));
			self.fRec25[0] = fSlow27 * (self.fRec9[1] + fSlow26 * self.fRec26[0]) + fSlow25 * self.fRec25[1];
			self.fVec5[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec25[0] + 1e-20;
			let mut fTemp4: F32 = self.fVec5[((i32::wrapping_sub(self.IOTA0, self.iConst32)) & 32767) as usize] - (fTemp0 + 0.6 * self.fRec23[1]);
			self.fVec6[(self.IOTA0 & 8191) as usize] = fTemp4;
			self.fRec23[0] = self.fVec6[((i32::wrapping_sub(self.IOTA0, self.iConst33)) & 8191) as usize];
			let mut fRec24: F32 = 0.6 * fTemp4;
			self.fRec30[0] = -(self.fConst14 * (self.fConst13 * self.fRec30[1] - (self.fRec13[1] + self.fRec13[2])));
			self.fRec29[0] = fSlow36 * (self.fRec13[1] + fSlow35 * self.fRec30[0]) + fSlow34 * self.fRec29[1];
			self.fVec7[(self.IOTA0 & 32767) as usize] = 0.35355338 * self.fRec29[0] + 1e-20;
			let mut fTemp5: F32 = self.fVec7[((i32::wrapping_sub(self.IOTA0, self.iConst39)) & 32767) as usize] - (fTemp0 + 0.6 * self.fRec27[1]);
			self.fVec8[(self.IOTA0 & 4095) as usize] = fTemp5;
			self.fRec27[0] = self.fVec8[((i32::wrapping_sub(self.IOTA0, self.iConst40)) & 4095) as usize];
			let mut fRec28: F32 = 0.6 * fTemp5;
			let mut fTemp6: F32 = fRec28 + fRec24 + fTemp3;
			self.fRec34[0] = -(self.fConst14 * (self.fConst13 * self.fRec34[1] - (self.fRec8[1] + self.fRec8[2])));
			self.fRec33[0] = fSlow45 * (self.fRec8[1] + fSlow44 * self.fRec34[0]) + fSlow43 * self.fRec33[1];
			self.fVec9[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec33[0] + 1e-20;
			let mut fTemp7: F32 = self.fVec9[((i32::wrapping_sub(self.IOTA0, self.iConst46)) & 65535) as usize] + fTemp0 + 0.6 * self.fRec31[1];
			self.fVec10[(self.IOTA0 & 8191) as usize] = fTemp7;
			self.fRec31[0] = self.fVec10[((i32::wrapping_sub(self.IOTA0, self.iConst47)) & 8191) as usize];
			let mut fRec32: F32 = -(0.6 * fTemp7);
			self.fRec38[0] = -(self.fConst14 * (self.fConst13 * self.fRec38[1] - (self.fRec12[1] + self.fRec12[2])));
			self.fRec37[0] = fSlow54 * (self.fRec12[1] + fSlow53 * self.fRec38[0]) + fSlow52 * self.fRec37[1];
			self.fVec11[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec37[0] + 1e-20;
			let mut fTemp8: F32 = self.fVec11[((i32::wrapping_sub(self.IOTA0, self.iConst53)) & 65535) as usize] + fTemp0 + 0.6 * self.fRec35[1];
			self.fVec12[(self.IOTA0 & 8191) as usize] = fTemp8;
			self.fRec35[0] = self.fVec12[((i32::wrapping_sub(self.IOTA0, self.iConst54)) & 8191) as usize];
			let mut fRec36: F32 = -(0.6 * fTemp8);
			self.fRec42[0] = -(self.fConst14 * (self.fConst13 * self.fRec42[1] - (self.fRec10[1] + self.fRec10[2])));
			self.fRec41[0] = fSlow63 * (self.fRec10[1] + fSlow62 * self.fRec42[0]) + fSlow61 * self.fRec41[1];
			self.fVec13[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec41[0] + 1e-20;
			let mut fTemp9: F32 = 0.6 * self.fRec39[1] + self.fVec13[((i32::wrapping_sub(self.IOTA0, self.iConst60)) & 65535) as usize];
			self.fVec14[(self.IOTA0 & 8191) as usize] = fTemp9 - fTemp0;
			self.fRec39[0] = self.fVec14[((i32::wrapping_sub(self.IOTA0, self.iConst61)) & 8191) as usize];
			let mut fRec40: F32 = 0.6 * (fTemp0 - fTemp9);
			self.fRec46[0] = -(self.fConst14 * (self.fConst13 * self.fRec46[1] - (self.fRec14[1] + self.fRec14[2])));
			self.fRec45[0] = fSlow72 * (self.fRec14[1] + fSlow71 * self.fRec46[0]) + fSlow70 * self.fRec45[1];
			self.fVec15[(self.IOTA0 & 65535) as usize] = 0.35355338 * self.fRec45[0] + 1e-20;
			let mut fTemp10: F32 = 0.6 * self.fRec43[1] + self.fVec15[((i32::wrapping_sub(self.IOTA0, self.iConst67)) & 65535) as usize];
			self.fVec16[(self.IOTA0 & 4095) as usize] = fTemp10 - fTemp0;
			self.fRec43[0] = self.fVec16[((i32::wrapping_sub(self.IOTA0, self.iConst68)) & 4095) as usize];
			let mut fRec44: F32 = 0.6 * (fTemp0 - fTemp10);
			self.fRec7[0] = self.fRec43[1] + self.fRec39[1] + self.fRec35[1] + self.fRec31[1] + self.fRec27[1] + self.fRec23[1] + self.fRec15[1] + self.fRec19[1] + fRec44 + fRec40 + fRec36 + fRec32 + fTemp6;
			self.fRec8[0] = self.fRec27[1] + self.fRec23[1] + self.fRec15[1] + self.fRec19[1] + fTemp6 - (self.fRec43[1] + self.fRec39[1] + self.fRec35[1] + self.fRec31[1] + fRec44 + fRec40 + fRec32 + fRec36);
			let mut fTemp11: F32 = fRec24 + fRec28;
			self.fRec9[0] = self.fRec35[1] + self.fRec31[1] + self.fRec15[1] + self.fRec19[1] + fRec36 + fRec32 + fTemp3 - (self.fRec43[1] + self.fRec39[1] + self.fRec27[1] + self.fRec23[1] + fRec44 + fRec40 + fTemp11);
			self.fRec10[0] = self.fRec43[1] + self.fRec39[1] + self.fRec15[1] + self.fRec19[1] + fRec44 + fRec40 + fTemp3 - (self.fRec35[1] + self.fRec31[1] + self.fRec27[1] + self.fRec23[1] + fRec36 + fRec32 + fTemp11);
			let mut fTemp12: F32 = fRec16 + fRec28;
			let mut fTemp13: F32 = fRec20 + fRec24;
			self.fRec11[0] = self.fRec39[1] + self.fRec31[1] + self.fRec23[1] + self.fRec19[1] + fRec40 + fRec32 + fTemp13 - (self.fRec43[1] + self.fRec35[1] + self.fRec27[1] + self.fRec15[1] + fRec44 + fRec36 + fTemp12);
			self.fRec12[0] = self.fRec43[1] + self.fRec35[1] + self.fRec23[1] + self.fRec19[1] + fRec44 + fRec36 + fTemp13 - (self.fRec39[1] + self.fRec31[1] + self.fRec27[1] + self.fRec15[1] + fRec40 + fRec32 + fTemp12);
			let mut fTemp14: F32 = fRec16 + fRec24;
			let mut fTemp15: F32 = fRec20 + fRec28;
			self.fRec13[0] = self.fRec43[1] + self.fRec31[1] + self.fRec27[1] + self.fRec19[1] + fRec44 + fRec32 + fTemp15 - (self.fRec39[1] + self.fRec35[1] + self.fRec23[1] + self.fRec15[1] + fRec40 + fRec36 + fTemp14);
			self.fRec14[0] = self.fRec39[1] + self.fRec35[1] + self.fRec27[1] + self.fRec19[1] + fRec40 + fRec36 + fTemp15 - (self.fRec43[1] + self.fRec31[1] + self.fRec23[1] + self.fRec15[1] + fRec44 + fRec32 + fTemp14);
			self.fRec6[0] = 0.37 * (self.fRec8[0] - self.fRec9[0]) - self.fConst8 * (self.fConst6 * self.fRec6[2] + self.fConst4 * self.fRec6[1]);
			let mut fTemp16: F32 = 2.0 * self.fRec6[1];
			let mut fTemp17: F32 = self.fRec6[2] + (self.fRec6[0] - fTemp16);
			self.fRec49[0] = 0.37 * (self.fRec8[0] + self.fRec9[0]) - self.fConst8 * (self.fConst6 * self.fRec49[2] + self.fConst4 * self.fRec49[1]);
			let mut fTemp18: F32 = 2.0 * self.fRec49[1];
			let mut fTemp19: F32 = self.fRec49[2] + (self.fRec49[0] - fTemp18);
			self.fRec48[0] = self.fConst71 * F32::abs(F32::abs(self.fConst70 * fTemp19) + F32::abs(self.fConst70 * fTemp17)) + self.fConst69 * self.fRec48[1];
			let mut iTemp20: i32 = (self.fRec48[0] > fSlow73) as i32;
			self.iVec17[0] = iTemp20;
			self.iRec50[0] = std::cmp::max(i32::wrapping_mul(self.iConst72, (iTemp20 < self.iVec17[1]) as i32), i32::wrapping_add(self.iRec50[1], -1));
			let mut fTemp21: F32 = F32::abs(F32::max((iTemp20) as F32, ((self.iRec50[0] > 0) as i32) as u32 as F32));
			let mut fTemp22: F32 = if (fTemp21 > self.fRec47[1]) as i32 != 0 {self.fConst69} else {self.fConst73};
			self.fRec47[0] = fTemp21 * (1.0 - fTemp22) + self.fRec47[1] * fTemp22;
			let mut fTemp23: F32 = *input1;
			self.fVec18[0] = fSlow74;
			let mut fTemp24: F32 = ((fSlow74 <= self.fVec18[1]) as i32) as u32 as F32;
			let mut fTemp25: F32 = 1.0 - self.fConst74 * fTemp24;
			self.fRec51[0] = fSlow75 * fTemp25 + self.fConst74 * fTemp24 * self.fRec51[1];
			let mut fTemp26: F32 = self.fRec51[0] * (fTemp23 + self.fConst70 * self.fRec47[0] * fTemp17);
			let mut fTemp27: F32 = *input0;
			let mut fTemp28: F32 = self.fRec51[0] * (fTemp27 + self.fConst70 * self.fRec47[0] * fTemp19);
			let mut fTemp29: F32 = F32::abs(F32::abs(fTemp28) + F32::abs(fTemp26));
			let mut fTemp30: F32 = if (fTemp29 > self.fRec5[1]) as i32 != 0 {self.fConst76} else {self.fConst75};
			self.fRec5[0] = fTemp29 * (1.0 - fTemp30) + self.fRec5[1] * fTemp30;
			self.fRec4[0] = self.fConst77 * self.fRec4[1] - self.fConst78 * F32::max(2e+01 * F32::log10(F32::max(1.1754944e-38, self.fRec5[0])) + 1.0, 0.0);
			let mut fTemp31: F32 = F32::powf(1e+01, 0.05 * self.fRec4[0]);
			let mut fTemp32: F32 = fTemp28 * fTemp31;
			let mut fTemp33: F32 = F32::abs(fTemp32);
			self.fRec52[0] = fSlow76 * fTemp25 + self.fConst74 * fTemp24 * self.fRec52[1];
			let mut fTemp34: F32 = 1.0 / F32::min(F32::max(0.4 * self.fRec52[0] + 0.5, 0.001), 0.999) + -2.0;
			let mut fTemp35: F32 = 1.0 - 0.7 * self.fRec52[0];
			self.fRec53[0] = fSlow77 * fTemp25 + self.fConst74 * fTemp24 * self.fRec53[1];
			let mut fTemp36: F32 = F32::abs(self.fRec53[0] * fTemp33 * (1.0 - (i32::wrapping_mul(2, (fTemp32 < 0.0) as i32)) as F32) * fTemp35 / (fTemp34 * (1.0 - fTemp33) + 1.0));
			let mut iTemp37: i32 = ((fTemp36 >= self.fRec3[1]) as i32) | (((self.iRec2[1]) as F32 >= self.fConst1) as i32);
			self.iRec2[0] = if iTemp37 != 0 {0} else {i32::wrapping_add(self.iRec2[1], 1)};
			self.fRec3[0] = if iTemp37 != 0 {fTemp36} else {self.fRec3[1]};
			let mut fTemp38: F32 = F32::abs(self.fRec3[0]);
			self.fRec1[0] = F32::max(fTemp38, self.fConst75 * self.fRec1[1] + self.fConst79 * fTemp38);
			let mut fRec0: F32 = self.fRec1[0];
			self.fVbargraph0 = 0.025 * (2e+01 * F32::log10(F32::max(1.1754944e-38, fRec0)) + 4e+01);
			let mut fTemp39: F32 = self.fRec51[0] * fTemp31;
			let mut fTemp40: F32 = fTemp39 * (fTemp27 + self.fConst70 * self.fRec47[0] * (self.fRec49[0] + self.fRec49[2] - fTemp18));
			let mut fTemp41: F32 = F32::abs(fTemp40);
			let mut fTemp42: F32 = self.fRec53[0] * fTemp35;
			*output0 = fTemp42 * fTemp41 * (1.0 - (i32::wrapping_mul(2, (fTemp40 < 0.0) as i32)) as F32) / (fTemp34 * (1.0 - fTemp41) + 1.0);
			let mut fTemp43: F32 = fTemp26 * fTemp31;
			let mut fTemp44: F32 = F32::abs(fTemp43);
			let mut fTemp45: F32 = F32::abs(fTemp42 * fTemp44 * (1.0 - (i32::wrapping_mul(2, (fTemp43 < 0.0) as i32)) as F32) / (fTemp34 * (1.0 - fTemp44) + 1.0));
			let mut iTemp46: i32 = ((fTemp45 >= self.fRec57[1]) as i32) | (((self.iRec56[1]) as F32 >= self.fConst1) as i32);
			self.iRec56[0] = if iTemp46 != 0 {0} else {i32::wrapping_add(self.iRec56[1], 1)};
			self.fRec57[0] = if iTemp46 != 0 {fTemp45} else {self.fRec57[1]};
			let mut fTemp47: F32 = F32::abs(self.fRec57[0]);
			self.fRec55[0] = F32::max(fTemp47, self.fConst75 * self.fRec55[1] + self.fConst79 * fTemp47);
			let mut fRec54: F32 = self.fRec55[0];
			self.fVbargraph1 = 0.025 * (2e+01 * F32::log10(F32::max(1.1754944e-38, fRec54)) + 4e+01);
			let mut fTemp48: F32 = fTemp39 * (fTemp23 + self.fConst70 * self.fRec47[0] * (self.fRec6[0] + self.fRec6[2] - fTemp16));
			let mut fTemp49: F32 = F32::abs(fTemp48);
			*output1 = fTemp42 * fTemp49 * (1.0 - (i32::wrapping_mul(2, (fTemp48 < 0.0) as i32)) as F32) / (fTemp34 * (1.0 - fTemp49) + 1.0);
			self.fRec18[1] = self.fRec18[0];
			self.fRec17[1] = self.fRec17[0];
			self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
			self.fRec15[1] = self.fRec15[0];
			self.fRec22[1] = self.fRec22[0];
			self.fRec21[1] = self.fRec21[0];
			self.fRec19[1] = self.fRec19[0];
			self.fRec26[1] = self.fRec26[0];
			self.fRec25[1] = self.fRec25[0];
			self.fRec23[1] = self.fRec23[0];
			self.fRec30[1] = self.fRec30[0];
			self.fRec29[1] = self.fRec29[0];
			self.fRec27[1] = self.fRec27[0];
			self.fRec34[1] = self.fRec34[0];
			self.fRec33[1] = self.fRec33[0];
			self.fRec31[1] = self.fRec31[0];
			self.fRec38[1] = self.fRec38[0];
			self.fRec37[1] = self.fRec37[0];
			self.fRec35[1] = self.fRec35[0];
			self.fRec42[1] = self.fRec42[0];
			self.fRec41[1] = self.fRec41[0];
			self.fRec39[1] = self.fRec39[0];
			self.fRec46[1] = self.fRec46[0];
			self.fRec45[1] = self.fRec45[0];
			self.fRec43[1] = self.fRec43[0];
			self.fRec7[2] = self.fRec7[1];
			self.fRec7[1] = self.fRec7[0];
			self.fRec8[2] = self.fRec8[1];
			self.fRec8[1] = self.fRec8[0];
			self.fRec9[2] = self.fRec9[1];
			self.fRec9[1] = self.fRec9[0];
			self.fRec10[2] = self.fRec10[1];
			self.fRec10[1] = self.fRec10[0];
			self.fRec11[2] = self.fRec11[1];
			self.fRec11[1] = self.fRec11[0];
			self.fRec12[2] = self.fRec12[1];
			self.fRec12[1] = self.fRec12[0];
			self.fRec13[2] = self.fRec13[1];
			self.fRec13[1] = self.fRec13[0];
			self.fRec14[2] = self.fRec14[1];
			self.fRec14[1] = self.fRec14[0];
			self.fRec6[2] = self.fRec6[1];
			self.fRec6[1] = self.fRec6[0];
			self.fRec49[2] = self.fRec49[1];
			self.fRec49[1] = self.fRec49[0];
			self.fRec48[1] = self.fRec48[0];
			self.iVec17[1] = self.iVec17[0];
			self.iRec50[1] = self.iRec50[0];
			self.fRec47[1] = self.fRec47[0];
			self.fVec18[1] = self.fVec18[0];
			self.fRec51[1] = self.fRec51[0];
			self.fRec5[1] = self.fRec5[0];
			self.fRec4[1] = self.fRec4[0];
			self.fRec52[1] = self.fRec52[0];
			self.fRec53[1] = self.fRec53[0];
			self.iRec2[1] = self.iRec2[0];
			self.fRec3[1] = self.fRec3[0];
			self.fRec1[1] = self.fRec1[0];
			self.iRec56[1] = self.iRec56[0];
			self.fRec57[1] = self.fRec57[0];
			self.fRec55[1] = self.fRec55[0];
		}
	}

}

