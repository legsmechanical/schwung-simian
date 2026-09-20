/* ------------------------------------------------------------
author: "Oren Kurtz"
copyright: "Punk Labs LLC 2024"
license: "GPLv3-or-later"
name: "OneTrick SIMIAN"
version: "2.00"
Code generated with Faust 2.88.0 (https://faust.grame.fr)
Compilation options: -lang cpp -fpga-mem-th 4 -ct 0 -cn DSP_Output -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */

#ifndef  __DSP_Output_H__
#define  __DSP_Output_H__

#ifndef FAUSTFLOAT
#define FAUSTFLOAT float
#endif 

#include <algorithm>
#include <cmath>
#include <cstdint>
#include <math.h>
#ifndef FAUST_INT_WRAP
#define FAUST_INT_WRAP
inline int faust_wrap_add(int a, int b) { return int((unsigned int)a + (unsigned int)b); }
inline int faust_wrap_sub(int a, int b) { return int((unsigned int)a - (unsigned int)b); }
inline int faust_wrap_mul(int a, int b) { return int((unsigned int)a * (unsigned int)b); }
#endif


#ifndef FAUSTCLASS 
#define FAUSTCLASS DSP_Output
#endif

#ifdef __APPLE__ 
#define exp10f __exp10f
#define exp10 __exp10
#endif

#if defined(_WIN32)
#define RESTRICT __restrict
#else
#define RESTRICT __restrict__
#endif

static float DSP_Output_faustpower2_f(float value) {
	return value * value;
}

class DSP_Output : public dsp {
	
 private:
	
	int fSampleRate;
	float fConst0;
	float fConst1;
	float fConst2;
	float fConst3;
	float fConst4;
	float fConst5;
	float fConst6;
	float fConst7;
	float fRec5[2];
	float fRec6[2];
	float fRec7[2];
	float fRec8[2];
	float fRec9[2];
	float fRec10[2];
	float fConst8;
	float fConst9;
	float fConst10;
	FAUSTFLOAT fHslider0;
	float fConst11;
	float fConst12;
	float fRec11[2];
	float fRec12[2];
	float fConst13;
	float fConst14;
	float fConst15;
	float fConst16;
	float fRec13[2];
	int IOTA0;
	float fVec0[65536];
	float fConst17;
	int iConst18;
	float fVec1[2048];
	int iConst19;
	float fVec2[8192];
	int iConst20;
	float fRec14[2];
	float fRec15[2];
	float fConst21;
	float fConst22;
	float fConst23;
	float fConst24;
	float fRec16[2];
	float fVec3[65536];
	float fConst25;
	int iConst26;
	float fVec4[8192];
	int iConst27;
	float fRec17[2];
	float fVec5[32768];
	float fConst28;
	int iConst29;
	float fVec6[4096];
	float fConst30;
	float fConst31;
	float fConst32;
	float fConst33;
	float fRec19[2];
	int iConst34;
	float fRec21[2];
	float fConst35;
	float fConst36;
	float fConst37;
	float fConst38;
	float fRec22[2];
	float fConst39;
	float fConst40;
	float fConst41;
	float fConst42;
	float fRec23[2];
	float fVec7[32768];
	float fConst43;
	int iConst44;
	float fVec8[4096];
	float fConst45;
	float fConst46;
	float fConst47;
	float fConst48;
	float fRec25[2];
	float fVec9[65536];
	float fConst49;
	int iConst50;
	float fVec10[8192];
	float fVec11[65536];
	float fConst51;
	int iConst52;
	float fVec12[8192];
	int iConst53;
	float fRec28[2];
	int iConst54;
	float fRec29[2];
	float fConst55;
	float fConst56;
	float fConst57;
	float fConst58;
	float fRec30[2];
	float fVec13[65536];
	float fConst59;
	int iConst60;
	float fVec14[4096];
	int iConst61;
	float fRec31[2];
	int iConst62;
	float fRec33[2];
	float fVec15[32768];
	float fConst63;
	int iConst64;
	float fVec16[8192];
	int iConst65;
	float fRec35[2];
	float fRec37[3];
	float fRec38[3];
	float fRec39[3];
	float fRec40[3];
	float fRec41[3];
	float fRec42[3];
	float fRec43[3];
	float fRec44[3];
	float fConst66;
	float fConst67;
	float fConst68;
	float fRec4[3];
	float fConst69;
	float fConst70;
	float fRec47[3];
	float fRec46[2];
	FAUSTFLOAT fHslider1;
	int iVec17[2];
	int iConst71;
	int iRec48[2];
	float fConst72;
	float fRec45[2];
	FAUSTFLOAT fHslider2;
	float fConst73;
	FAUSTFLOAT fButton0;
	float fVec18[2];
	float fRec49[2];
	float fConst74;
	float fConst75;
	float fConst76;
	float fConst77;
	float fRec51[2];
	float fRec50[2];
	FAUSTFLOAT fHslider3;
	float fRec52[2];
	FAUSTFLOAT fHslider4;
	float fRec53[2];
	float fConst78;
	int iRec2[2];
	float fRec3[2];
	float fConst79;
	float fRec1[2];
	FAUSTFLOAT fVbargraph0;
	float fRec56[2];
	int iRec57[2];
	float fRec55[2];
	FAUSTFLOAT fVbargraph1;
	
 public:
	DSP_Output() {
	}
	
	DSP_Output(const DSP_Output&) = default;
	
	virtual ~DSP_Output() = default;
	
	DSP_Output& operator=(const DSP_Output&) = default;
	
	void metadata(Meta* m) { 
		m->declare("analyzers.lib/amp_follower_ar:author", "Jonatan Liljedahl, revised by Romain Michon");
		m->declare("analyzers.lib/name", "Faust Analyzer Library");
		m->declare("analyzers.lib/version", "1.4.0");
		m->declare("author", "Oren Kurtz");
		m->declare("basics.lib/name", "Faust Basic Element Library");
		m->declare("basics.lib/peakholder:author", "Dario Sanfilippo");
		m->declare("basics.lib/peakholder:copyright", "Copyright (C) 2022 Dario Sanfilippo <sanfilippo.dario@gmail.com>");
		m->declare("basics.lib/peakholder:license", "LicenseRef-STK-4.3");
		m->declare("basics.lib/version", "1.23.0");
		m->declare("compile_options", "-lang cpp -fpga-mem-th 4 -ct 0 -cn DSP_Output -dtl 65536 -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
		m->declare("compressors.lib/compression_gain_mono:author", "Julius O. Smith III");
		m->declare("compressors.lib/compression_gain_mono:copyright", "Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("compressors.lib/compression_gain_mono:license", "LicenseRef-STK-4.3");
		m->declare("compressors.lib/compressor_stereo:author", "Julius O. Smith III");
		m->declare("compressors.lib/compressor_stereo:copyright", "Copyright (C) 2014-2020 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("compressors.lib/compressor_stereo:license", "LicenseRef-STK-4.3");
		m->declare("compressors.lib/name", "Faust Compressor Effect Library");
		m->declare("compressors.lib/version", "1.6.0");
		m->declare("copyright", "Punk Labs LLC 2024");
		m->declare("delays.lib/name", "Faust Delay Library");
		m->declare("delays.lib/version", "1.2.0");
		m->declare("filename", "output.dsp");
		m->declare("filters.lib/allpass_comb:author", "Julius O. Smith III");
		m->declare("filters.lib/allpass_comb:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/allpass_comb:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/fir:author", "Julius O. Smith III");
		m->declare("filters.lib/fir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/fir:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/highpass:author", "Julius O. Smith III");
		m->declare("filters.lib/highpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/highpass:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/iir:author", "Julius O. Smith III");
		m->declare("filters.lib/iir:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/iir:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/lowpass0_highpass1:author", "Julius O. Smith III");
		m->declare("filters.lib/lowpass0_highpass1:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/lowpass0_highpass1:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/lowpass:author", "Julius O. Smith III");
		m->declare("filters.lib/lowpass:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/lowpass:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/name", "Faust Filters Library");
		m->declare("filters.lib/tf1:author", "Julius O. Smith III");
		m->declare("filters.lib/tf1:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf1:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/tf1s:author", "Julius O. Smith III");
		m->declare("filters.lib/tf1s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf1s:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/tf2:author", "Julius O. Smith III");
		m->declare("filters.lib/tf2:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf2:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/tf2s:author", "Julius O. Smith III");
		m->declare("filters.lib/tf2s:copyright", "Copyright (C) 2003-2019 by Julius O. Smith III <jos@ccrma.stanford.edu>");
		m->declare("filters.lib/tf2s:license", "LicenseRef-STK-4.3");
		m->declare("filters.lib/version", "1.9.0");
		m->declare("interpolators.lib/interpolate_linear:author", "Stéphane Letz");
		m->declare("interpolators.lib/interpolate_linear:licence", "MIT");
		m->declare("interpolators.lib/name", "Faust Interpolator Library");
		m->declare("interpolators.lib/remap:author", "David Braun");
		m->declare("interpolators.lib/version", "1.6.0");
		m->declare("license", "GPLv3-or-later");
		m->declare("maths.lib/author", "GRAME");
		m->declare("maths.lib/copyright", "GRAME");
		m->declare("maths.lib/license", "LicenseRef-LGPL-2.1-or-later-with-Faust-exception");
		m->declare("maths.lib/name", "Faust Math Library");
		m->declare("maths.lib/version", "2.9.0");
		m->declare("misceffects.lib/gate_gain_mono:author", "Julius O. Smith III");
		m->declare("misceffects.lib/gate_gain_mono:license", "LicenseRef-STK-4.3");
		m->declare("misceffects.lib/gate_stereo:author", "Julius O. Smith III");
		m->declare("misceffects.lib/gate_stereo:license", "LicenseRef-STK-4.3");
		m->declare("misceffects.lib/name", "Misc Effects Library");
		m->declare("misceffects.lib/version", "2.6.0");
		m->declare("name", "OneTrick SIMIAN");
		m->declare("onetrick.lib/copyright", "Copyright (c) 2023 Punk Labs LLC");
		m->declare("onetrick.lib/license", "GPLv3 (or later)");
		m->declare("onetrick.lib/name", "OneTrick DSP Library");
		m->declare("platform.lib/name", "Generic Platform Library");
		m->declare("platform.lib/version", "1.3.0");
		m->declare("reverbs.lib/name", "Faust Reverb Library");
		m->declare("reverbs.lib/version", "1.5.1");
		m->declare("routes.lib/hadamard:author", "Remy Muller, revised by Romain Michon");
		m->declare("routes.lib/name", "Faust Signal Routing Library");
		m->declare("routes.lib/version", "1.4.0");
		m->declare("shared.lib/author", "Oren Kurtz");
		m->declare("shared.lib/copyright", "Punk Labs LLC 2024");
		m->declare("shared.lib/license", "GPLv3-or-later");
		m->declare("shared.lib/name", "OneTrick SIMIAN");
		m->declare("shared.lib/options", "[midi:on]");
		m->declare("shared.lib/version", "2.00");
		m->declare("signals.lib/name", "Faust Routing Library");
		m->declare("signals.lib/onePoleSwitching:author", "Jonatan Liljedahl, revised by Dario Sanfilippo");
		m->declare("signals.lib/onePoleSwitching:licence", "LicenseRef-STK-4.3");
		m->declare("signals.lib/version", "1.7.0");
		m->declare("version", "2.00");
	}

	virtual int getNumInputs() {
		return 3;
	}
	virtual int getNumOutputs() {
		return 2;
	}
	
	static void classInit(int sample_rate) {
	}
	
	virtual void instanceConstants(int sample_rate) {
		fSampleRate = sample_rate;
		fConst0 = std::min<float>(1.92e+05f, std::max<float>(1.0f, static_cast<float>(fSampleRate)));
		fConst1 = std::tan(628.31854f / fConst0);
		fConst2 = DSP_Output_faustpower2_f(fConst1);
		fConst3 = 1.0f / fConst1;
		fConst4 = (fConst3 + 1.4142135f) / fConst1 + 1.0f;
		fConst5 = 1.4125376f / (fConst2 * fConst4);
		fConst6 = 1.0f / (fConst3 + 1.0f);
		fConst7 = 1.0f - fConst3;
		fConst8 = std::floor(0.174713f * fConst0 + 0.5f);
		fConst9 = fConst8 / fConst0;
		fConst10 = 8.913233f * fConst9;
		fConst11 = std::cos(37699.113f / fConst0);
		fConst12 = 6.9077554f * fConst9;
		fConst13 = std::floor(0.256891f * fConst0 + 0.5f);
		fConst14 = fConst13 / fConst0;
		fConst15 = 8.913233f * fConst14;
		fConst16 = 6.9077554f * fConst14;
		fConst17 = std::floor(0.027333f * fConst0 + 0.5f);
		iConst18 = static_cast<int>(std::min<float>(65536.0f, std::max<float>(0.0f, fConst13 - fConst17)));
		iConst19 = static_cast<int>(std::min<float>(8192.0f, std::max<float>(0.0f, 0.01f * fConst0)));
		iConst20 = static_cast<int>(std::min<float>(8192.0f, std::max<float>(0.0f, fConst17 + -1.0f)));
		fConst21 = std::floor(0.153129f * fConst0 + 0.5f);
		fConst22 = fConst21 / fConst0;
		fConst23 = 8.913233f * fConst22;
		fConst24 = 6.9077554f * fConst22;
		fConst25 = std::floor(0.022904f * fConst0 + 0.5f);
		iConst26 = static_cast<int>(std::min<float>(32768.0f, std::max<float>(0.0f, fConst8 - fConst25)));
		iConst27 = static_cast<int>(std::min<float>(8192.0f, std::max<float>(0.0f, fConst25 + -1.0f)));
		fConst28 = std::floor(0.020346f * fConst0 + 0.5f);
		iConst29 = static_cast<int>(std::min<float>(32768.0f, std::max<float>(0.0f, fConst21 - fConst28)));
		fConst30 = std::floor(0.192303f * fConst0 + 0.5f);
		fConst31 = fConst30 / fConst0;
		fConst32 = 8.913233f * fConst31;
		fConst33 = 6.9077554f * fConst31;
		iConst34 = static_cast<int>(std::min<float>(4096.0f, std::max<float>(0.0f, fConst28 + -1.0f)));
		fConst35 = std::floor(0.125f * fConst0 + 0.5f);
		fConst36 = fConst35 / fConst0;
		fConst37 = 8.913233f * fConst36;
		fConst38 = 6.9077554f * fConst36;
		fConst39 = std::floor(0.210389f * fConst0 + 0.5f);
		fConst40 = fConst39 / fConst0;
		fConst41 = 8.913233f * fConst40;
		fConst42 = 6.9077554f * fConst40;
		fConst43 = std::floor(0.013458f * fConst0 + 0.5f);
		iConst44 = static_cast<int>(std::min<float>(32768.0f, std::max<float>(0.0f, fConst35 - fConst43)));
		fConst45 = std::floor(0.219991f * fConst0 + 0.5f);
		fConst46 = fConst45 / fConst0;
		fConst47 = 8.913233f * fConst46;
		fConst48 = 6.9077554f * fConst46;
		fConst49 = std::floor(0.024421f * fConst0 + 0.5f);
		iConst50 = static_cast<int>(std::min<float>(65536.0f, std::max<float>(0.0f, fConst39 - fConst49)));
		fConst51 = std::floor(0.029291f * fConst0 + 0.5f);
		iConst52 = static_cast<int>(std::min<float>(32768.0f, std::max<float>(0.0f, fConst30 - fConst51)));
		iConst53 = static_cast<int>(std::min<float>(8192.0f, std::max<float>(0.0f, fConst51 + -1.0f)));
		iConst54 = static_cast<int>(std::min<float>(8192.0f, std::max<float>(0.0f, fConst49 + -1.0f)));
		fConst55 = std::floor(0.127837f * fConst0 + 0.5f);
		fConst56 = fConst55 / fConst0;
		fConst57 = 8.913233f * fConst56;
		fConst58 = 6.9077554f * fConst56;
		fConst59 = std::floor(0.019123f * fConst0 + 0.5f);
		iConst60 = static_cast<int>(std::min<float>(65536.0f, std::max<float>(0.0f, fConst45 - fConst59)));
		iConst61 = static_cast<int>(std::min<float>(4096.0f, std::max<float>(0.0f, fConst59 + -1.0f)));
		iConst62 = static_cast<int>(std::min<float>(4096.0f, std::max<float>(0.0f, fConst43 + -1.0f)));
		fConst63 = std::floor(0.031604f * fConst0 + 0.5f);
		iConst64 = static_cast<int>(std::min<float>(32768.0f, std::max<float>(0.0f, fConst55 - fConst63)));
		iConst65 = static_cast<int>(std::min<float>(8192.0f, std::max<float>(0.0f, fConst63 + -1.0f)));
		fConst66 = 1.0f / fConst4;
		fConst67 = (fConst3 + -1.4142135f) / fConst1 + 1.0f;
		fConst68 = 2.0f * (1.0f - 1.0f / fConst2);
		fConst69 = std::exp(-(1e+04f / fConst0));
		fConst70 = 1.0f - fConst69;
		iConst71 = static_cast<int>(0.1f * fConst0);
		fConst72 = std::exp(-(5e+01f / fConst0));
		fConst73 = std::exp(-(1e+02f / fConst0));
		fConst74 = std::exp(-(1e+05f / fConst0));
		fConst75 = 0.95f * (1.0f - fConst74);
		fConst76 = std::exp(-(2e+01f / fConst0));
		fConst77 = std::exp(-(5e+04f / fConst0));
		fConst78 = 0.05f * fConst0;
		fConst79 = 1.0f - fConst76;
	}
	
	virtual void instanceResetUserInterface() {
		fHslider0 = static_cast<FAUSTFLOAT>(5e+01f);
		fHslider1 = static_cast<FAUSTFLOAT>(-1e+02f);
		fHslider2 = static_cast<FAUSTFLOAT>(-6.0f);
		fButton0 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider3 = static_cast<FAUSTFLOAT>(0.0f);
		fHslider4 = static_cast<FAUSTFLOAT>(0.0f);
	}
	
	virtual void instanceClear() {
		for (int l0 = 0; l0 < 2; l0 = faust_wrap_add(l0, 1)) {
			fRec5[l0] = 0.0f;
		}
		for (int l1 = 0; l1 < 2; l1 = faust_wrap_add(l1, 1)) {
			fRec6[l1] = 0.0f;
		}
		for (int l2 = 0; l2 < 2; l2 = faust_wrap_add(l2, 1)) {
			fRec7[l2] = 0.0f;
		}
		for (int l3 = 0; l3 < 2; l3 = faust_wrap_add(l3, 1)) {
			fRec8[l3] = 0.0f;
		}
		for (int l4 = 0; l4 < 2; l4 = faust_wrap_add(l4, 1)) {
			fRec9[l4] = 0.0f;
		}
		for (int l5 = 0; l5 < 2; l5 = faust_wrap_add(l5, 1)) {
			fRec10[l5] = 0.0f;
		}
		for (int l6 = 0; l6 < 2; l6 = faust_wrap_add(l6, 1)) {
			fRec11[l6] = 0.0f;
		}
		for (int l7 = 0; l7 < 2; l7 = faust_wrap_add(l7, 1)) {
			fRec12[l7] = 0.0f;
		}
		for (int l8 = 0; l8 < 2; l8 = faust_wrap_add(l8, 1)) {
			fRec13[l8] = 0.0f;
		}
		IOTA0 = 0;
		for (int l9 = 0; l9 < 65536; l9 = faust_wrap_add(l9, 1)) {
			fVec0[l9] = 0.0f;
		}
		for (int l10 = 0; l10 < 2048; l10 = faust_wrap_add(l10, 1)) {
			fVec1[l10] = 0.0f;
		}
		for (int l11 = 0; l11 < 8192; l11 = faust_wrap_add(l11, 1)) {
			fVec2[l11] = 0.0f;
		}
		for (int l12 = 0; l12 < 2; l12 = faust_wrap_add(l12, 1)) {
			fRec14[l12] = 0.0f;
		}
		for (int l13 = 0; l13 < 2; l13 = faust_wrap_add(l13, 1)) {
			fRec15[l13] = 0.0f;
		}
		for (int l14 = 0; l14 < 2; l14 = faust_wrap_add(l14, 1)) {
			fRec16[l14] = 0.0f;
		}
		for (int l15 = 0; l15 < 65536; l15 = faust_wrap_add(l15, 1)) {
			fVec3[l15] = 0.0f;
		}
		for (int l16 = 0; l16 < 8192; l16 = faust_wrap_add(l16, 1)) {
			fVec4[l16] = 0.0f;
		}
		for (int l17 = 0; l17 < 2; l17 = faust_wrap_add(l17, 1)) {
			fRec17[l17] = 0.0f;
		}
		for (int l18 = 0; l18 < 32768; l18 = faust_wrap_add(l18, 1)) {
			fVec5[l18] = 0.0f;
		}
		for (int l19 = 0; l19 < 4096; l19 = faust_wrap_add(l19, 1)) {
			fVec6[l19] = 0.0f;
		}
		for (int l20 = 0; l20 < 2; l20 = faust_wrap_add(l20, 1)) {
			fRec19[l20] = 0.0f;
		}
		for (int l21 = 0; l21 < 2; l21 = faust_wrap_add(l21, 1)) {
			fRec21[l21] = 0.0f;
		}
		for (int l22 = 0; l22 < 2; l22 = faust_wrap_add(l22, 1)) {
			fRec22[l22] = 0.0f;
		}
		for (int l23 = 0; l23 < 2; l23 = faust_wrap_add(l23, 1)) {
			fRec23[l23] = 0.0f;
		}
		for (int l24 = 0; l24 < 32768; l24 = faust_wrap_add(l24, 1)) {
			fVec7[l24] = 0.0f;
		}
		for (int l25 = 0; l25 < 4096; l25 = faust_wrap_add(l25, 1)) {
			fVec8[l25] = 0.0f;
		}
		for (int l26 = 0; l26 < 2; l26 = faust_wrap_add(l26, 1)) {
			fRec25[l26] = 0.0f;
		}
		for (int l27 = 0; l27 < 65536; l27 = faust_wrap_add(l27, 1)) {
			fVec9[l27] = 0.0f;
		}
		for (int l28 = 0; l28 < 8192; l28 = faust_wrap_add(l28, 1)) {
			fVec10[l28] = 0.0f;
		}
		for (int l29 = 0; l29 < 65536; l29 = faust_wrap_add(l29, 1)) {
			fVec11[l29] = 0.0f;
		}
		for (int l30 = 0; l30 < 8192; l30 = faust_wrap_add(l30, 1)) {
			fVec12[l30] = 0.0f;
		}
		for (int l31 = 0; l31 < 2; l31 = faust_wrap_add(l31, 1)) {
			fRec28[l31] = 0.0f;
		}
		for (int l32 = 0; l32 < 2; l32 = faust_wrap_add(l32, 1)) {
			fRec29[l32] = 0.0f;
		}
		for (int l33 = 0; l33 < 2; l33 = faust_wrap_add(l33, 1)) {
			fRec30[l33] = 0.0f;
		}
		for (int l34 = 0; l34 < 65536; l34 = faust_wrap_add(l34, 1)) {
			fVec13[l34] = 0.0f;
		}
		for (int l35 = 0; l35 < 4096; l35 = faust_wrap_add(l35, 1)) {
			fVec14[l35] = 0.0f;
		}
		for (int l36 = 0; l36 < 2; l36 = faust_wrap_add(l36, 1)) {
			fRec31[l36] = 0.0f;
		}
		for (int l37 = 0; l37 < 2; l37 = faust_wrap_add(l37, 1)) {
			fRec33[l37] = 0.0f;
		}
		for (int l38 = 0; l38 < 32768; l38 = faust_wrap_add(l38, 1)) {
			fVec15[l38] = 0.0f;
		}
		for (int l39 = 0; l39 < 8192; l39 = faust_wrap_add(l39, 1)) {
			fVec16[l39] = 0.0f;
		}
		for (int l40 = 0; l40 < 2; l40 = faust_wrap_add(l40, 1)) {
			fRec35[l40] = 0.0f;
		}
		for (int l41 = 0; l41 < 3; l41 = faust_wrap_add(l41, 1)) {
			fRec37[l41] = 0.0f;
		}
		for (int l42 = 0; l42 < 3; l42 = faust_wrap_add(l42, 1)) {
			fRec38[l42] = 0.0f;
		}
		for (int l43 = 0; l43 < 3; l43 = faust_wrap_add(l43, 1)) {
			fRec39[l43] = 0.0f;
		}
		for (int l44 = 0; l44 < 3; l44 = faust_wrap_add(l44, 1)) {
			fRec40[l44] = 0.0f;
		}
		for (int l45 = 0; l45 < 3; l45 = faust_wrap_add(l45, 1)) {
			fRec41[l45] = 0.0f;
		}
		for (int l46 = 0; l46 < 3; l46 = faust_wrap_add(l46, 1)) {
			fRec42[l46] = 0.0f;
		}
		for (int l47 = 0; l47 < 3; l47 = faust_wrap_add(l47, 1)) {
			fRec43[l47] = 0.0f;
		}
		for (int l48 = 0; l48 < 3; l48 = faust_wrap_add(l48, 1)) {
			fRec44[l48] = 0.0f;
		}
		for (int l49 = 0; l49 < 3; l49 = faust_wrap_add(l49, 1)) {
			fRec4[l49] = 0.0f;
		}
		for (int l50 = 0; l50 < 3; l50 = faust_wrap_add(l50, 1)) {
			fRec47[l50] = 0.0f;
		}
		for (int l51 = 0; l51 < 2; l51 = faust_wrap_add(l51, 1)) {
			fRec46[l51] = 0.0f;
		}
		for (int l52 = 0; l52 < 2; l52 = faust_wrap_add(l52, 1)) {
			iVec17[l52] = 0;
		}
		for (int l53 = 0; l53 < 2; l53 = faust_wrap_add(l53, 1)) {
			iRec48[l53] = 0;
		}
		for (int l54 = 0; l54 < 2; l54 = faust_wrap_add(l54, 1)) {
			fRec45[l54] = 0.0f;
		}
		for (int l55 = 0; l55 < 2; l55 = faust_wrap_add(l55, 1)) {
			fVec18[l55] = 0.0f;
		}
		for (int l56 = 0; l56 < 2; l56 = faust_wrap_add(l56, 1)) {
			fRec49[l56] = 0.0f;
		}
		for (int l57 = 0; l57 < 2; l57 = faust_wrap_add(l57, 1)) {
			fRec51[l57] = 0.0f;
		}
		for (int l58 = 0; l58 < 2; l58 = faust_wrap_add(l58, 1)) {
			fRec50[l58] = 0.0f;
		}
		for (int l59 = 0; l59 < 2; l59 = faust_wrap_add(l59, 1)) {
			fRec52[l59] = 0.0f;
		}
		for (int l60 = 0; l60 < 2; l60 = faust_wrap_add(l60, 1)) {
			fRec53[l60] = 0.0f;
		}
		for (int l61 = 0; l61 < 2; l61 = faust_wrap_add(l61, 1)) {
			iRec2[l61] = 0;
		}
		for (int l62 = 0; l62 < 2; l62 = faust_wrap_add(l62, 1)) {
			fRec3[l62] = 0.0f;
		}
		for (int l63 = 0; l63 < 2; l63 = faust_wrap_add(l63, 1)) {
			fRec1[l63] = 0.0f;
		}
		for (int l64 = 0; l64 < 2; l64 = faust_wrap_add(l64, 1)) {
			fRec56[l64] = 0.0f;
		}
		for (int l65 = 0; l65 < 2; l65 = faust_wrap_add(l65, 1)) {
			iRec57[l65] = 0;
		}
		for (int l66 = 0; l66 < 2; l66 = faust_wrap_add(l66, 1)) {
			fRec55[l66] = 0.0f;
		}
	}
	
	virtual void init(int sample_rate) {
		classInit(sample_rate);
		instanceInit(sample_rate);
	}
	
	virtual void instanceInit(int sample_rate) {
		instanceConstants(sample_rate);
		instanceResetUserInterface();
		instanceClear();
	}
	
	virtual DSP_Output* clone() {
		return new DSP_Output(*this);
	}
	
	virtual int getSampleRate() {
		return fSampleRate;
	}
	
	virtual void buildUserInterface(UI* ui_interface) {
		ui_interface->openVerticalBox("OneTrick SIMIAN");
		ui_interface->openTabBox("Globals");
		ui_interface->openVerticalBox("Globals");
		ui_interface->declare(&fHslider4, "0", "");
		ui_interface->declare(&fHslider4, "export", "Gain");
		ui_interface->declare(&fHslider4, "unit", "db");
		ui_interface->addHorizontalSlider("Gain_Global", &fHslider4, FAUSTFLOAT(0.0f), FAUSTFLOAT(-1e+02f), FAUSTFLOAT(6.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider2, "1", "");
		ui_interface->declare(&fHslider2, "export", "Drive");
		ui_interface->declare(&fHslider2, "unit", "db");
		ui_interface->addHorizontalSlider("Drive_Global", &fHslider2, FAUSTFLOAT(-6.0f), FAUSTFLOAT(-1e+02f), FAUSTFLOAT(6.0f), FAUSTFLOAT(0.1f));
		ui_interface->declare(&fHslider0, "2", "");
		ui_interface->declare(&fHslider0, "export", "Reverb Size");
		ui_interface->declare(&fHslider0, "unit", "%");
		ui_interface->addHorizontalSlider("ReverbSize_Global", &fHslider0, FAUSTFLOAT(5e+01f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1e+02f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider1, "3", "");
		ui_interface->declare(&fHslider1, "export", "Reverb Gate");
		ui_interface->declare(&fHslider1, "unit", "db");
		ui_interface->addHorizontalSlider("ReverbGate_Global", &fHslider1, FAUSTFLOAT(-1e+02f), FAUSTFLOAT(-1e+02f), FAUSTFLOAT(0.0f), FAUSTFLOAT(0.01f));
		ui_interface->declare(&fHslider3, "5", "");
		ui_interface->declare(&fHslider3, "export", "Saturation");
		ui_interface->declare(&fHslider3, "unit", "%");
		ui_interface->addHorizontalSlider("Saturation_Global", &fHslider3, FAUSTFLOAT(0.0f), FAUSTFLOAT(0.0f), FAUSTFLOAT(1e+02f), FAUSTFLOAT(0.01f));
		ui_interface->closeBox();
		ui_interface->closeBox();
		ui_interface->addVerticalBargraph("VuMeterLeft", &fVbargraph0, FAUSTFLOAT(-4e+01f), FAUSTFLOAT(0.0f));
		ui_interface->addVerticalBargraph("VuMeterRight", &fVbargraph1, FAUSTFLOAT(-4e+01f), FAUSTFLOAT(0.0f));
		ui_interface->addButton("WakeUp", &fButton0);
		ui_interface->closeBox();
	}
	
	virtual void compute(int count, FAUSTFLOAT** RESTRICT inputs, FAUSTFLOAT** RESTRICT outputs) {
		FAUSTFLOAT* input0 = inputs[0];
		FAUSTFLOAT* input1 = inputs[1];
		FAUSTFLOAT* input2 = inputs[2];
		FAUSTFLOAT* output0 = outputs[0];
		FAUSTFLOAT* output1 = outputs[1];
		float fSlow0 = 0.015f * static_cast<float>(fHslider0) + 0.5f;
		float fSlow1 = std::exp(-(fConst10 / fSlow0));
		float fSlow2 = DSP_Output_faustpower2_f(fSlow1);
		float fSlow3 = 1.0f - fConst11 * fSlow2;
		float fSlow4 = 1.0f - fSlow2;
		float fSlow5 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow3) / DSP_Output_faustpower2_f(fSlow4) + -1.0f));
		float fSlow6 = fSlow3 / fSlow4;
		float fSlow7 = fSlow1 * (fSlow5 + (1.0f - fSlow6));
		float fSlow8 = std::exp(-(fConst12 / fSlow0)) / fSlow1 + -1.0f;
		float fSlow9 = fSlow6 - fSlow5;
		float fSlow10 = std::exp(-(fConst15 / fSlow0));
		float fSlow11 = DSP_Output_faustpower2_f(fSlow10);
		float fSlow12 = 1.0f - fConst11 * fSlow11;
		float fSlow13 = 1.0f - fSlow11;
		float fSlow14 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow12) / DSP_Output_faustpower2_f(fSlow13) + -1.0f));
		float fSlow15 = fSlow12 / fSlow13;
		float fSlow16 = fSlow10 * (fSlow14 + (1.0f - fSlow15));
		float fSlow17 = std::exp(-(fConst16 / fSlow0)) / fSlow10 + -1.0f;
		float fSlow18 = fSlow15 - fSlow14;
		float fSlow19 = std::exp(-(fConst23 / fSlow0));
		float fSlow20 = DSP_Output_faustpower2_f(fSlow19);
		float fSlow21 = 1.0f - fConst11 * fSlow20;
		float fSlow22 = 1.0f - fSlow20;
		float fSlow23 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow21) / DSP_Output_faustpower2_f(fSlow22) + -1.0f));
		float fSlow24 = fSlow21 / fSlow22;
		float fSlow25 = fSlow19 * (fSlow23 + (1.0f - fSlow24));
		float fSlow26 = std::exp(-(fConst24 / fSlow0)) / fSlow19 + -1.0f;
		float fSlow27 = fSlow24 - fSlow23;
		float fSlow28 = std::exp(-(fConst32 / fSlow0));
		float fSlow29 = DSP_Output_faustpower2_f(fSlow28);
		float fSlow30 = 1.0f - fConst11 * fSlow29;
		float fSlow31 = 1.0f - fSlow29;
		float fSlow32 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow30) / DSP_Output_faustpower2_f(fSlow31) + -1.0f));
		float fSlow33 = fSlow30 / fSlow31;
		float fSlow34 = fSlow28 * (fSlow32 + (1.0f - fSlow33));
		float fSlow35 = std::exp(-(fConst33 / fSlow0)) / fSlow28 + -1.0f;
		float fSlow36 = fSlow33 - fSlow32;
		float fSlow37 = std::exp(-(fConst37 / fSlow0));
		float fSlow38 = DSP_Output_faustpower2_f(fSlow37);
		float fSlow39 = 1.0f - fConst11 * fSlow38;
		float fSlow40 = 1.0f - fSlow38;
		float fSlow41 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow39) / DSP_Output_faustpower2_f(fSlow40) + -1.0f));
		float fSlow42 = fSlow39 / fSlow40;
		float fSlow43 = fSlow37 * (fSlow41 + (1.0f - fSlow42));
		float fSlow44 = std::exp(-(fConst38 / fSlow0)) / fSlow37 + -1.0f;
		float fSlow45 = fSlow42 - fSlow41;
		float fSlow46 = std::exp(-(fConst41 / fSlow0));
		float fSlow47 = DSP_Output_faustpower2_f(fSlow46);
		float fSlow48 = 1.0f - fConst11 * fSlow47;
		float fSlow49 = 1.0f - fSlow47;
		float fSlow50 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow48) / DSP_Output_faustpower2_f(fSlow49) + -1.0f));
		float fSlow51 = fSlow48 / fSlow49;
		float fSlow52 = fSlow46 * (fSlow50 + (1.0f - fSlow51));
		float fSlow53 = std::exp(-(fConst42 / fSlow0)) / fSlow46 + -1.0f;
		float fSlow54 = fSlow51 - fSlow50;
		float fSlow55 = std::exp(-(fConst47 / fSlow0));
		float fSlow56 = DSP_Output_faustpower2_f(fSlow55);
		float fSlow57 = 1.0f - fConst11 * fSlow56;
		float fSlow58 = 1.0f - fSlow56;
		float fSlow59 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow57) / DSP_Output_faustpower2_f(fSlow58) + -1.0f));
		float fSlow60 = fSlow57 / fSlow58;
		float fSlow61 = fSlow55 * (fSlow59 + (1.0f - fSlow60));
		float fSlow62 = std::exp(-(fConst48 / fSlow0)) / fSlow55 + -1.0f;
		float fSlow63 = fSlow60 - fSlow59;
		float fSlow64 = std::exp(-(fConst57 / fSlow0));
		float fSlow65 = DSP_Output_faustpower2_f(fSlow64);
		float fSlow66 = 1.0f - fConst11 * fSlow65;
		float fSlow67 = 1.0f - fSlow65;
		float fSlow68 = std::sqrt(std::max<float>(0.0f, DSP_Output_faustpower2_f(fSlow66) / DSP_Output_faustpower2_f(fSlow67) + -1.0f));
		float fSlow69 = fSlow66 / fSlow67;
		float fSlow70 = fSlow64 * (fSlow68 + (1.0f - fSlow69));
		float fSlow71 = std::exp(-(fConst58 / fSlow0)) / fSlow64 + -1.0f;
		float fSlow72 = fSlow69 - fSlow68;
		float fSlow73 = std::pow(1e+01f, 0.05f * static_cast<float>(fHslider1));
		float fSlow74 = std::pow(1e+01f, 0.05f * std::max<float>(-1e+02f, static_cast<float>(fHslider2)));
		float fSlow75 = static_cast<float>(fButton0);
		float fSlow76 = 0.01f * static_cast<float>(fHslider3);
		float fSlow77 = std::pow(1e+01f, 0.05f * std::max<float>(-1e+02f, static_cast<float>(fHslider4)));
		for (int i0 = 0; i0 < count; i0 = faust_wrap_add(i0, 1)) {
			fRec5[0] = -(fConst6 * (fConst7 * fRec5[1] - (fRec39[1] + fRec39[2])));
			fRec6[0] = -(fConst6 * (fConst7 * fRec6[1] - (fRec43[1] + fRec43[2])));
			fRec7[0] = -(fConst6 * (fConst7 * fRec7[1] - (fRec38[1] + fRec38[2])));
			fRec8[0] = -(fConst6 * (fConst7 * fRec8[1] - (fRec41[1] + fRec41[2])));
			fRec9[0] = -(fConst6 * (fConst7 * fRec9[1] - (fRec37[1] + fRec37[2])));
			fRec10[0] = -(fConst6 * (fConst7 * fRec10[1] - (fRec42[1] + fRec42[2])));
			fRec11[0] = fSlow7 * (fRec43[1] + fSlow8 * fRec6[0]) + fSlow9 * fRec11[1];
			fRec12[0] = -(fConst6 * (fConst7 * fRec12[1] - (fRec44[1] + fRec44[2])));
			fRec13[0] = fSlow16 * (fRec38[1] + fSlow17 * fRec7[0]) + fSlow18 * fRec13[1];
			fVec0[IOTA0 & 65535] = 1e-20f + 0.35355338f * fRec13[0];
			float fTemp0 = 0.6f * fRec14[1] + fVec0[(faust_wrap_sub(IOTA0, iConst18)) & 65535];
			fVec1[IOTA0 & 2047] = static_cast<float>(input2[i0]);
			float fTemp1 = 0.3f * fVec1[(faust_wrap_sub(IOTA0, iConst19)) & 2047];
			fVec2[IOTA0 & 8191] = fTemp0 - fTemp1;
			fRec14[0] = fVec2[(faust_wrap_sub(IOTA0, iConst20)) & 8191];
			fRec15[0] = -(fConst6 * (fConst7 * fRec15[1] - (fRec40[1] + fRec40[2])));
			fRec16[0] = fSlow25 * (fRec39[1] + fSlow26 * fRec5[0]) + fSlow27 * fRec16[1];
			fVec3[IOTA0 & 65535] = 1e-20f + 0.35355338f * fRec11[0];
			float fTemp2 = fTemp1 - 0.6f * fRec17[1] + fVec3[(faust_wrap_sub(IOTA0, iConst26)) & 65535];
			fVec4[IOTA0 & 8191] = fTemp2;
			fRec17[0] = fVec4[(faust_wrap_sub(IOTA0, iConst27)) & 8191];
			fVec5[IOTA0 & 32767] = 1e-20f + 0.35355338f * fRec16[0];
			float fTemp3 = fTemp1 - 0.6f * fRec21[1] + fVec5[(faust_wrap_sub(IOTA0, iConst29)) & 32767];
			fVec6[IOTA0 & 4095] = fTemp3;
			float fRec18 = 0.6f * fTemp3;
			fRec19[0] = fSlow34 * (fRec44[1] + fSlow35 * fRec12[0]) + fSlow36 * fRec19[1];
			float fRec20 = 0.6f * fTemp2;
			fRec21[0] = fVec6[(faust_wrap_sub(IOTA0, iConst34)) & 4095];
			fRec22[0] = fSlow43 * (fRec42[1] + fSlow44 * fRec10[0]) + fSlow45 * fRec22[1];
			fRec23[0] = fSlow52 * (fRec40[1] + fSlow53 * fRec15[0]) + fSlow54 * fRec23[1];
			fVec7[IOTA0 & 32767] = 1e-20f + 0.35355338f * fRec22[0];
			float fTemp4 = fVec7[(faust_wrap_sub(IOTA0, iConst44)) & 32767] - (fTemp1 + 0.6f * fRec33[1]);
			fVec8[IOTA0 & 4095] = fTemp4;
			float fRec24 = 0.6f * fTemp4;
			fRec25[0] = fSlow61 * (fRec41[1] + fSlow62 * fRec8[0]) + fSlow63 * fRec25[1];
			float fRec26 = 0.6f * (fTemp1 - fTemp0);
			fVec9[IOTA0 & 65535] = 1e-20f + 0.35355338f * fRec23[0];
			float fTemp5 = fTemp1 + 0.6f * fRec29[1] + fVec9[(faust_wrap_sub(IOTA0, iConst50)) & 65535];
			fVec10[IOTA0 & 8191] = fTemp5;
			float fRec27 = -(0.6f * fTemp5);
			fVec11[IOTA0 & 65535] = 1e-20f + 0.35355338f * fRec19[0];
			float fTemp6 = fTemp1 + 0.6f * fRec28[1] + fVec11[(faust_wrap_sub(IOTA0, iConst52)) & 65535];
			fVec12[IOTA0 & 8191] = fTemp6;
			fRec28[0] = fVec12[(faust_wrap_sub(IOTA0, iConst53)) & 8191];
			fRec29[0] = fVec10[(faust_wrap_sub(IOTA0, iConst54)) & 8191];
			fRec30[0] = fSlow70 * (fRec37[1] + fSlow71 * fRec9[0]) + fSlow72 * fRec30[1];
			fVec13[IOTA0 & 65535] = 1e-20f + 0.35355338f * fRec25[0];
			float fTemp7 = 0.6f * fRec31[1] + fVec13[(faust_wrap_sub(IOTA0, iConst60)) & 65535];
			fVec14[IOTA0 & 4095] = fTemp7 - fTemp1;
			fRec31[0] = fVec14[(faust_wrap_sub(IOTA0, iConst61)) & 4095];
			float fRec32 = -(0.6f * fTemp6);
			fRec33[0] = fVec8[(faust_wrap_sub(IOTA0, iConst62)) & 4095];
			float fRec34 = 0.6f * (fTemp1 - fTemp7);
			fVec15[IOTA0 & 32767] = 1e-20f + 0.35355338f * fRec30[0];
			float fTemp8 = fVec15[(faust_wrap_sub(IOTA0, iConst64)) & 32767] - (fTemp1 + 0.6f * fRec35[1]);
			fVec16[IOTA0 & 8191] = fTemp8;
			fRec35[0] = fVec16[(faust_wrap_sub(IOTA0, iConst65)) & 8191];
			float fRec36 = 0.6f * fTemp8;
			float fTemp9 = fRec17[1] + fRec21[1];
			fRec37[0] = fRec32 + fRec27 + fRec20 + fRec18 + fRec28[1] + fRec29[1] + fTemp9 - (fRec34 + fRec26 + fRec24 + fRec36 + fRec35[1] + fRec31[1] + fRec14[1] + fRec33[1]);
			float fTemp10 = fRec14[1] + fRec17[1];
			float fTemp11 = fRec21[1] + fTemp10;
			fRec38[0] = fRec34 + fRec26 + fRec20 + fRec18 + fRec31[1] + fTemp11 - (fRec32 + fRec27 + fRec24 + fRec36 + fRec35[1] + fRec28[1] + fRec33[1] + fRec29[1]);
			fRec39[0] = fRec34 + fRec26 + fRec32 + fRec27 + fRec24 + fRec36 + fRec20 + fRec18 + fRec35[1] + fRec31[1] + fRec28[1] + fRec29[1] + fRec33[1] + fTemp11;
			fRec40[0] = fRec24 + fRec36 + fRec20 + fRec18 + fRec35[1] + fRec33[1] + fTemp9 - (fRec34 + fRec26 + fRec32 + fRec27 + fRec31[1] + fRec28[1] + fRec14[1] + fRec29[1]);
			float fTemp12 = fRec14[1] + fRec21[1];
			fRec41[0] = fRec26 + fRec32 + fRec24 + fRec18 + fRec28[1] + fRec33[1] + fTemp12 - (fRec34 + fRec27 + fRec36 + fRec20 + fRec35[1] + fRec31[1] + fRec17[1] + fRec29[1]);
			fRec42[0] = fRec34 + fRec27 + fRec24 + fRec18 + fRec31[1] + fRec29[1] + fRec21[1] + fRec33[1] - (fRec26 + fRec32 + fRec36 + fRec20 + fRec35[1] + fRec28[1] + fTemp10);
			fRec43[0] = fRec26 + fRec27 + fRec36 + fRec18 + fRec35[1] + fRec29[1] + fTemp12 - (fRec34 + fRec32 + fRec24 + fRec20 + fRec31[1] + fRec28[1] + fRec17[1] + fRec33[1]);
			fRec44[0] = fRec34 + fRec32 + fRec36 + fRec18 + fRec35[1] + fRec31[1] + fRec21[1] + fRec28[1] - (fRec26 + fRec27 + fRec24 + fRec20 + fRec29[1] + fRec33[1] + fTemp10);
			fRec4[0] = 0.37f * (fRec40[0] + fRec37[0]) - fConst66 * (fConst67 * fRec4[2] + fConst68 * fRec4[1]);
			float fTemp13 = fRec4[2] + (fRec4[0] - 2.0f * fRec4[1]);
			fRec47[0] = 0.37f * (fRec40[0] - fRec37[0]) - fConst66 * (fConst67 * fRec47[2] + fConst68 * fRec47[1]);
			float fTemp14 = fRec47[2] + (fRec47[0] - 2.0f * fRec47[1]);
			fRec46[0] = fConst70 * std::fabs(std::fabs(fConst5 * fTemp13) + std::fabs(fConst5 * fTemp14)) + fConst69 * fRec46[1];
			int iTemp15 = fRec46[0] > fSlow73;
			iVec17[0] = iTemp15;
			iRec48[0] = std::max<int>(faust_wrap_mul(iConst71, iTemp15 < iVec17[1]), faust_wrap_add(-1, iRec48[1]));
			float fTemp16 = std::fabs(std::max<float>(static_cast<float>(iTemp15), static_cast<float>(iRec48[0] > 0)));
			float fTemp17 = ((fTemp16 > fRec45[1]) ? fConst69 : fConst72);
			fRec45[0] = fTemp16 * (1.0f - fTemp17) + fRec45[1] * fTemp17;
			fVec18[0] = fSlow75;
			float fTemp18 = static_cast<float>(fSlow75 <= fVec18[1]);
			float fTemp19 = 1.0f - fConst73 * fTemp18;
			fRec49[0] = fSlow74 * fTemp19 + fConst73 * fTemp18 * fRec49[1];
			float fTemp20 = (static_cast<float>(input0[i0]) + fConst5 * fTemp13 * fRec45[0]) * fRec49[0];
			float fTemp21 = fRec49[0] * (static_cast<float>(input1[i0]) + fConst5 * fTemp14 * fRec45[0]);
			float fTemp22 = std::fabs(std::fabs(fTemp20) + std::fabs(fTemp21));
			float fTemp23 = ((fTemp22 > fRec51[1]) ? fConst77 : fConst76);
			fRec51[0] = fTemp22 * (1.0f - fTemp23) + fRec51[1] * fTemp23;
			fRec50[0] = fConst74 * fRec50[1] - fConst75 * std::max<float>(1.0f + 2e+01f * std::log10(std::max<float>(1.1754944e-38f, fRec51[0])), 0.0f);
			float fTemp24 = std::pow(1e+01f, 0.05f * fRec50[0]);
			float fTemp25 = fTemp20 * fTemp24;
			float fTemp26 = std::fabs(fTemp25);
			fRec52[0] = fSlow76 * fTemp19 + fConst73 * fTemp18 * fRec52[1];
			float fTemp27 = 1.0f - 0.7f * fRec52[0];
			fRec53[0] = fSlow77 * fTemp19 + fConst73 * fTemp18 * fRec53[1];
			float fTemp28 = -2.0f + 1.0f / std::min<float>(std::max<float>(0.5f + 0.4f * fRec52[0], 0.001f), 0.999f);
			float fTemp29 = fTemp26 * (1.0f - static_cast<float>(faust_wrap_mul(2, fTemp25 < 0.0f))) * fTemp27 * fRec53[0] / (1.0f + fTemp28 * (1.0f - fTemp26));
			float fTemp30 = std::fabs(fTemp29);
			int iTemp31 = (fTemp30 >= fRec3[1]) | (static_cast<float>(iRec2[1]) >= fConst78);
			iRec2[0] = ((iTemp31) ? 0 : faust_wrap_add(1, iRec2[1]));
			fRec3[0] = ((iTemp31) ? fTemp30 : fRec3[1]);
			float fTemp32 = std::fabs(fRec3[0]);
			fRec1[0] = std::max<float>(fTemp32, fConst76 * fRec1[1] + fConst79 * fTemp32);
			float fRec0 = fRec1[0];
			fVbargraph0 = static_cast<FAUSTFLOAT>(0.025f * (4e+01f + 2e+01f * std::log10(std::max<float>(1.1754944e-38f, fRec0))));
			output0[i0] = static_cast<FAUSTFLOAT>(fTemp29);
			float fTemp33 = fTemp21 * fTemp24;
			float fTemp34 = std::fabs(fTemp33);
			float fTemp35 = fTemp27 * fRec53[0] * fTemp34 * (1.0f - static_cast<float>(faust_wrap_mul(2, fTemp33 < 0.0f))) / (1.0f + fTemp28 * (1.0f - fTemp34));
			float fTemp36 = std::fabs(fTemp35);
			int iTemp37 = (fTemp36 >= fRec56[1]) | (static_cast<float>(iRec57[1]) >= fConst78);
			fRec56[0] = ((iTemp37) ? fTemp36 : fRec56[1]);
			iRec57[0] = ((iTemp37) ? 0 : faust_wrap_add(1, iRec57[1]));
			float fTemp38 = std::fabs(fRec56[0]);
			fRec55[0] = std::max<float>(fTemp38, fConst76 * fRec55[1] + fConst79 * fTemp38);
			float fRec54 = fRec55[0];
			fVbargraph1 = static_cast<FAUSTFLOAT>(0.025f * (4e+01f + 2e+01f * std::log10(std::max<float>(1.1754944e-38f, fRec54))));
			output1[i0] = static_cast<FAUSTFLOAT>(fTemp35);
			fRec5[1] = fRec5[0];
			fRec6[1] = fRec6[0];
			fRec7[1] = fRec7[0];
			fRec8[1] = fRec8[0];
			fRec9[1] = fRec9[0];
			fRec10[1] = fRec10[0];
			fRec11[1] = fRec11[0];
			fRec12[1] = fRec12[0];
			fRec13[1] = fRec13[0];
			IOTA0 = faust_wrap_add(IOTA0, 1);
			fRec14[1] = fRec14[0];
			fRec15[1] = fRec15[0];
			fRec16[1] = fRec16[0];
			fRec17[1] = fRec17[0];
			fRec19[1] = fRec19[0];
			fRec21[1] = fRec21[0];
			fRec22[1] = fRec22[0];
			fRec23[1] = fRec23[0];
			fRec25[1] = fRec25[0];
			fRec28[1] = fRec28[0];
			fRec29[1] = fRec29[0];
			fRec30[1] = fRec30[0];
			fRec31[1] = fRec31[0];
			fRec33[1] = fRec33[0];
			fRec35[1] = fRec35[0];
			fRec37[2] = fRec37[1];
			fRec37[1] = fRec37[0];
			fRec38[2] = fRec38[1];
			fRec38[1] = fRec38[0];
			fRec39[2] = fRec39[1];
			fRec39[1] = fRec39[0];
			fRec40[2] = fRec40[1];
			fRec40[1] = fRec40[0];
			fRec41[2] = fRec41[1];
			fRec41[1] = fRec41[0];
			fRec42[2] = fRec42[1];
			fRec42[1] = fRec42[0];
			fRec43[2] = fRec43[1];
			fRec43[1] = fRec43[0];
			fRec44[2] = fRec44[1];
			fRec44[1] = fRec44[0];
			fRec4[2] = fRec4[1];
			fRec4[1] = fRec4[0];
			fRec47[2] = fRec47[1];
			fRec47[1] = fRec47[0];
			fRec46[1] = fRec46[0];
			iVec17[1] = iVec17[0];
			iRec48[1] = iRec48[0];
			fRec45[1] = fRec45[0];
			fVec18[1] = fVec18[0];
			fRec49[1] = fRec49[0];
			fRec51[1] = fRec51[0];
			fRec50[1] = fRec50[0];
			fRec52[1] = fRec52[0];
			fRec53[1] = fRec53[0];
			iRec2[1] = iRec2[0];
			fRec3[1] = fRec3[0];
			fRec1[1] = fRec1[0];
			fRec56[1] = fRec56[0];
			iRec57[1] = iRec57[0];
			fRec55[1] = fRec55[0];
		}
	}

};

#endif
