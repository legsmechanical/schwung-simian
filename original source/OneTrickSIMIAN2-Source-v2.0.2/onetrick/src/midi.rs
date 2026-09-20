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

use num_enum::TryFromPrimitive;


/// List of General MIDI drum notes
#[derive(PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum GeneralMidiDrums {
    AcousticBassDrum = 35,
    BassDrum = 36,
    SideStick = 37,
    AcousticSnare = 38,
    HandClap = 39,
    ElectricSnare = 40,
    LowFloorTom = 41,
    ClosedHihat = 42,
    HighFloorTom = 43,
    PedalHihat = 44,
    LowTom = 45,
    OpenHihat = 46,
    LowMidTom = 47,
    HighMidTom = 48,
    CrashCymbal = 49,
    HighTom = 50,
    RideCymbal = 51,
    ChineseCymbal = 52,
    RideBell = 53,
    Tambourine = 54,
    SplashCymbal = 55,
    Cowbell = 56,
    CrashCymbal2 = 57,
    VibraSlap = 58,
    RideCymbal2 = 59,
}


/// List of General MIDI percussion notes
#[derive(PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum GeneralMidiPercussion {
    HighBongo = 60,
    LowBongo = 61,
    MuteHighConga = 62,
    OpenHighConga = 63,
    LowConga = 64,
    HighTimbal = 65,
    LowTimbal = 66,
    HighAgogo = 67,
    LowAgogo = 68,
    Cabasa = 69,
    Maracas = 70,
    ShortWhistle = 71,
    LongWhistle = 72,
    ShortGuiro = 73,
    LongGuiro = 74,
    Claves = 75,
    HighWoodBlock = 76,
    LowWoodBlock = 77,
    MuteCuica = 78,
    OpenCuica = 79,
    MuteTriangle = 80,
    OpenTriangle = 81,
    Cabasa2 = 82,
}


/// List of extended General MIDI percussion notes
#[derive(PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum GeneralMidiPercussionEx {
    JingleBell = 83,
    BellTree = 84,
    Castanet = 85,
    SideStick = 86,
    TaikoLow = 87,
}


/// List of General MIDI notes
#[derive(PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum MidiNote {
    CN1 = 0,
    CsN1 = 1,
    DN1 = 2,
    DsN1 = 3,
    EN1 = 4,
    FN1 = 5,
    FsN1 = 6,
    GN1 = 7,
    GsN1 = 8,
    AN1 = 9,
    AsN1 = 10,
    BN1 = 11,
    C0 = 12,
    Cs0 = 13,
    D0 = 14,
    Ds0 = 15,
    E0 = 16,
    F0 = 17,
    Fs0 = 18,
    G0 = 19,
    Gs0 = 20,
    A0 = 21, // Lowest on a Piano
    As0 = 22,
    B0 = 23,
    C1 = 24,
    Cs1 = 25,
    D1 = 26,
    Ds1 = 27,
    E1 = 28,
    F1 = 29,
    Fs1 = 30,
    G1 = 31,
    Gs1 = 32,
    A1 = 33,
    As1 = 34,
    B1 = 35,
    C2 = 36,
    Cs2 = 37,
    D2 = 38,
    Ds2 = 39,
    E2 = 40,
    F2 = 41,
    Fs2 = 42,
    G2 = 43,
    Gs2 = 44,
    A2 = 45,
    As2 = 46,
    B2 = 47,
    C3 = 48,
    Cs3 = 49,
    D3 = 50,
    Ds3 = 51,
    E3 = 52,
    F3 = 53,
    Fs3 = 54,
    G3 = 55,
    Gs3 = 56,
    A3 = 57,
    As3 = 58,
    B3 = 59,
    C4 = 60,
    Cs4 = 61,
    D4 = 62,
    Ds4 = 63,
    E4 = 64,
    F4 = 65,
    Fs4 = 66,
    G4 = 67,
    Gs4 = 68,
    A4 = 69,
    As4 = 70,
    B4 = 71,
    C5 = 72,
    Cs5 = 73,
    D5 = 74,
    Ds5 = 75,
    E5 = 76,
    F5 = 77,
    Fs5 = 78,
    G5 = 79,
    Gs5 = 80,
    A5 = 81,
    As5 = 82,
    B5 = 83,
    C6 = 84,
    Cs6 = 85,
    D6 = 86,
    Ds6 = 87,
    E6 = 88,
    F6 = 89,
    Fs6 = 90,
    G6 = 91,
    Gs6 = 92,
    A6 = 93,
    As6 = 94,
    B6 = 95,
    C7 = 96,
    Cs7 = 97,
    D7 = 98,
    Ds7 = 99,
    E7 = 100,
    F7 = 101,
    Fs7 = 102,
    G7 = 103,
    Gs7 = 104,
    A7 = 105,
    As7 = 106,
    B7 = 107,
    C8 = 108, // Highest on a Piano
    Cs8 = 109,
    D8 = 110,
    Ds8 = 111,
    E8 = 112,
    F8 = 113,
    Fs8 = 114,
    G8 = 115,
    Gs8 = 116,
    A8 = 117,
    As8 = 118,
    B8 = 119,
}
/// List of General MIDI CC Message
#[derive(PartialEq, TryFromPrimitive)]
#[repr(u8)]
pub enum MidiCC {
    BankSelect = 0,
    ModWheel = 1,
    Breath = 2,
    FootPedal = 4,
    PortamentoTime = 5,
    DataEntry = 6,
    Volume = 7,
    Balance = 8,
    Pan = 10,
    Expression = 11,
    EffectController1 = 12,
    EffectController2 = 13,
    GeneralPurpose1 = 16,
    GeneralPurpose2 = 17,
    GeneralPurpose3 = 18,
    GeneralPurpose4 = 19,
    BankSelectLSB = 32,
    ModWheelLSB = 33,
    BreathController = 34,
    FootPedalLSB = 36,
    PortamentoTimeLSB = 37,
    DataEntryLSB = 38,
    VolumeLSB = 39,
    BalanceLSB = 40,
    PanLSB = 42,
    ExpressionLSB = 43,
    EffectControl1LSB = 44,
    EffectControl2LSB = 45,
    Sustain = 64, // on/off
    Portamentao = 65, // on/off
    Sostenuto = 66, // on/off
    SoftPedal = 67, // on/off
    Legato = 68, // on/off
    HoldPedal2 = 69, // alternative sustain
    SoundController1 = 70,
    SoundController2 = 71, //Resonance,
    SoundController3 = 72,
    SoundController4 = 73,
    SoundController5 = 74, //FrequencyCutoff,
    SoundController6 = 75,
    SoundController7 = 76,
    SoundController8 = 77,
    SoundController9 = 78,
    SoundController10 = 79,
    GeneralPurpose5 = 80, // on/off
    GeneralPurpose6 = 81, // on/off
    GeneralPurpose7 = 82, // on/off
    GeneralPurpose8 = 83, // on/off
    GeneralPurpose9 = 84, // on/off
    Effect1Depth = 91,
    Effect2Depth = 92,
    Effect3Depth = 93,
    Effect4Depth = 94,
    Effect5Depth = 95,
    DataBoundIncrement = 96,
    DataBoundDecrement = 97,
    NrpnLSB1 = 98,
    NrpnLSB2 = 99,
    RpnLSB1 = 100,
    RpnLSB2 = 101,
    ChannelMute = 120, //panic
    ResetAllControllers = 121,
    LocalKeyboard = 122, // on/off
    AllNotes = 123, // on/off
    OmniModeOff = 124,
    OmniModeOn = 125,
    MonoMode = 126,
    PolyMode = 127,
}
