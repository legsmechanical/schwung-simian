/*
   ┏━━━┓╱╱╱╱╱┏┓╱╱┏┓╱╱╱╱╱┏┓╱╱   
   ┃┏━┓┃╱╱╱╱╱┃┃╱╱┃┃╱╱╱╱╱┃┃╱╱╱  
   ┃┗━┛┣┓┏┳━━┫┃┏┓┃┃╱╱┏━━┫┗━┳━━┓
   ┃┏━━┫┃┃┃┏┓┃┗┛┃┃┃╱┏┫┏┓┃┏┓┃━━┫
   ┃┃╱╱┃┗┛┃┃┃┃┏┓┃┃┗━┛┃┏┓┃┗┛┣━━┃
   ┗┛╱╱┗━━┻┛┗┻┛┗┛┗━━━┻┛┗┻━━┻━━┛
    ━━━━━━━━━━━━━━━━━━━━━━━━━━ 

	Copyright (c) 2024 Punk Labs LLC

	This file is part of OneTrick SIMIAN

	OneTrick SIMIAN is free software: you can redistribute it and/or modify it
	under the terms of the GNU General Public License as published by the Free
	Software Foundation, either version 3 of the License, or (at your option)
	any later version.

	OneTrick SIMIAN is distributed in the hope that it will be useful, but WITHOUT
	ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or
	FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License for
	more details.

	You should have received a copy of the GNU General Public License along with
	OneTrick SIMIAN.  If not, see <http://www.gnu.org/licenses/>.
*/

declare name "OneTrick SIMIAN";
declare author "Oren Kurtz";
declare copyright "Punk Labs LLC 2024";
declare version "2.00";
declare license "GPLv3-or-later";

import("shared.lib");

globalGain = ba.db2linear(max(-100, hslider("t:Globals/v:Globals/[0]Gain_Global[unit:db][export:Gain]", 0, -100, 6, 0.1))) : ot.smoothParam;
globalDrive = ba.db2linear(max(-100, hslider("t:Globals/v:Globals/[1]Drive_Global[unit:db][export:Drive]", -6, -100, 6, 0.1))) : ot.smoothParam;
globalReverbSize = hslider("t:Globals/v:Globals/[2]ReverbSize_Global[unit:%][export:Reverb Size]", 50, 0, 100, 0.01) * 0.01;
globalReverbGate = hslider("t:Globals/v:Globals/[3]ReverbGate_Global[unit:db][export:Reverb Gate]", -100, -100, 0, 0.01);
globalSensitivity = hslider("t:Globals/v:Globals/[4]Sensitivity_Global[unit:%][export:Sensitivity]", 100, 0, 100, 0.01) * 0.01;
globalSaturation = hslider("t:Globals/v:Globals/[5]Saturation_Global[unit:%][export:Saturation]", 0, 0, 100, 0.01) * 0.01 : ot.smoothParam;

globalReverb = result with {
	reverbDamp = 0.25;
	t60 = it.interpolate_linear(globalReverbSize, 0.5, 2.0);
	rev = ot.reverbZitaStereo(10, t60, reverbDamp) : ot.stereo(fi.highpass(2, 200));
	gate = ef.gate_stereo(globalReverbGate, 0.1/1000, 100/1000, 20/1000);
	matchLegacy = ot.stereo(*(ba.db2linear(3)));
	result = rev : matchLegacy : gate;
};


finalizeOutput = ot.stereo(*(globalDrive)) : applyLimit : ot.stereo(applySaturation(globalSaturation) : *(globalGain));

processOutput(l,r,fx) =
	getModuleStereo(modules) : //stereo(*(0)) :
	addStereo(getModuleSidechain(modules) <: globalReverb) :
 	finalizeOutput /* <: si.bus(2) */
 	with { 
		modules = (l,r,fx);
	} : ot.output.vuMeterStereo;


process = processOutput;
