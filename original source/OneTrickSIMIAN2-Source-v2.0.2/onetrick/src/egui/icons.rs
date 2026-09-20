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

#[cfg(feature = "egui_extras")]
use {
        egui_extras::image::RetainedImage,
        nih_plug_egui::egui::{
            Context,
            Shape,
            Vec2,
        },
    };

use nih_plug_egui::egui::{
    Align2, Color32, FontData,
    /*
    epaint::{
        PathShape,
    },
    */
    //Shape,
    //Stroke,
    //Ui,
    FontDefinitions, FontFamily, FontId, FontTweak, Painter, Pos2, Rect,
};

/// Stores a list of built-in Dingbats from various icon fonts
/// These can be used with some painter using the IconDrawer attribute
#[derive(Clone, Copy)]
pub enum Dingbat {
    // OneTrickIcons
    Circle,
    KickDrum,
    SnareDrum,
    Claves,
    DrumSticks,
    TomDrumSmall,
    TomDrumMedium,
    TomDrumLarge,
    Cymbal,
    Hihat,
    OneTrickIcon,
    Upgrade,
    LogoBlockerBasic,
    LogoBlockerCutout,
    LogoBlockerSplatter,
    Grunge1,
    Grunge2,

    //PluginIcons
    Plugin1,

    //FontAwesome
    New,
    New2,
    Duplicate,
    Open,
    Save,
    Edit,
    Delete,
    Delete2,
    Refresh,
    Add,
    Remove,
    Accept,
    Cancel,
    Help,
    Info,
    Settings,
    Settings2,
    Settings3,
    Settings4,
    Settings5,
    ZoomIn,
    ZoomOut,
    Up,
    Down,
    Left,
    Right,
    Up2,
    Down2,
    Left2,
    Right2,
    Up3,
    Down3,
    Left3,
    Right3,
    Up4,
    Down4,
    Left4,
    Right4,
    Up5,
    Down5,
    Left5,
    Right5,
    Up6,
    Down6,
    Left6,
    Right6,
    KickDrum2,
    SnareDrum2,
    SnareRim,
    Wand,
    MagicWand,
    SteelDrum,
    GenericDrum,
    Rim,
    Cymbal2,
    Hand,
    Clap,
    Clap2,
    Cow,
    Bell,
    Guitar,
    Disc,
    Favorite,
    Flag,
    Circle2,
    Copyright,
    Trademark,
    Registered,
    Home,
    Cut,
    Share,
    Freeze,
    SquareWave,
    Volume1,
    Volume3,
    VolumeMute,
    Mic,
    MicMute,
    PaintBrush,
    PaintBrush2,
    PaintRoller,
    Spraypaint,
    Stamp,
    Palette,
    Background,
    Image,
    Fire,
    Screwdriver,
    Icons,
}

/// Adds some additional rendering functions to Painter
pub trait IconDrawer {
    /// Draws a dingbat
    fn dingbat(
        &self,
        pos: Pos2,
        anchor: Align2,
        dingbat: Dingbat,
        size: f32,
        text_color: Color32,
    ) -> Rect;

    /// Draws a RetainedImage
    #[cfg(feature = "egui_extras")]
    fn img(
        &self,
        ctx: &Context,
        icon: &RetainedImage,
        pos: Pos2,
        size: f32,
        color: Color32,
    ) -> Rect;
}

impl IconDrawer for Painter {
    fn dingbat(
        &self,
        pos: Pos2,
        anchor: Align2,
        dingbat: Dingbat,
        size: f32,
        text_color: Color32,
    ) -> Rect {
        let (font, chr) = Icons::dingbat_char(dingbat);

        // Quantizing helps with caching, but this may already be done for us:
        //let quantize_size = 0.25;
        //let size = (size / quantize_size).floor() * quantize_size;

        let font_id = FontId::new(size, FontFamily::Name(font.into()));
        self.text(pos, anchor, chr.to_string(), font_id, text_color)
    }
    #[cfg(feature = "egui_extras")]
    fn img(
        &self,
        ctx: &Context,
        icon: &RetainedImage,
        pos: Pos2,
        size: f32,
        color: Color32,
    ) -> Rect {
        let aspect = icon.width() as f32 / icon.height() as f32;
        let rect = Rect::from_center_size(pos, Vec2::new(size * aspect, size));
        self.add(Shape::image(
            icon.texture_id(ctx),
            rect,
            Rect::from_min_size(Pos2::new(0.0, 0.0), Vec2::new(1.0, 1.0)),
            color,
        ));

        rect
    }
}

/// Helpers to work with Dingbat fonts
pub struct Icons {}

impl Icons {

    /// Adds required Dingbat fonts to a FontDefinitions
    pub fn add_fonts(fonts: &mut FontDefinitions) {
        let name = "Icons"; //OneTrickIcons
        fonts.font_data.insert(
            name.to_owned(),
            FontData::from_static(include_bytes!(
                "../assets/fonts/OneTrickIcons/OneTrickIcons.ttf"
            ))
            .tweak(FontTweak {
                scale: 1.0,
                y_offset_factor: 0.07,
                ..Default::default()
            }),
        );
        fonts
            .families
            .insert(FontFamily::Name(name.into()), vec![name.to_owned()]);

        let name = "PluginIcons"; //PluginIcons
        fonts.font_data.insert(
            name.to_owned(),
            FontData::from_static(include_bytes!(
                "../../../src/assets/fonts/PluginIcons/PluginIcons.ttf"
            ))
            .tweak(FontTweak {
                scale: 1.0,
                y_offset_factor: 0.0,
                ..Default::default()
            }),
        );
        fonts
            .families
            .insert(FontFamily::Name(name.into()), vec![name.to_owned()]);

        let name = "FontAwesome"; //FontAwesome
        fonts.font_data.insert(
            name.to_owned(),
            FontData::from_static(include_bytes!(
                "../assets/fonts/Font-Awesome-6.x/webfonts/fa-solid-900.ttf"
            ))
            .tweak(FontTweak {
                scale: 1.0,
                y_offset_factor: 0.0,
                ..Default::default()
            }),
        );
        fonts
            .families
            .insert(FontFamily::Name(name.into()), vec![name.to_owned()]);
    }
    fn dingbat_char(dingbat: Dingbat) -> (&'static str, &'static str) {
        match dingbat {
            // OneTrickIcons
            Dingbat::Circle => ("Icons", "A"),
            Dingbat::KickDrum => ("Icons", "B"),
            Dingbat::SnareDrum => ("Icons", "C"),
            Dingbat::Claves => ("Icons", "D"),
            Dingbat::DrumSticks => ("Icons", "E"),
            Dingbat::TomDrumSmall => ("Icons", "F"),
            Dingbat::TomDrumMedium => ("Icons", "G"),
            Dingbat::TomDrumLarge => ("Icons", "H"),
            Dingbat::Cymbal => ("Icons", "I"),
            Dingbat::Hihat => ("Icons", "J"),
            Dingbat::OneTrickIcon => ("Icons", "K"),
            Dingbat::Upgrade => ("Icons", "L"),
            Dingbat::LogoBlockerBasic => ("Icons", "M"),
            Dingbat::LogoBlockerCutout => ("Icons", "N"),
            Dingbat::LogoBlockerSplatter => ("Icons", "O"),
            Dingbat::Grunge1 => ("Icons", "P"),
            Dingbat::Grunge2 => ("Icons", "Q"),

            // PluginIcons
            Dingbat::Plugin1 => ("PluginIcons", "B"),

            // FontAwesome
            Dingbat::New => ("FontAwesome", "📄"), //📄🗋
            Dingbat::New2 => ("FontAwesome", ""),
            Dingbat::Duplicate => ("FontAwesome", ""),
            Dingbat::Open => ("FontAwesome", "📂"), //📂🗁
            Dingbat::Save => ("FontAwesome", ""),  //💾
            Dingbat::Edit => ("FontAwesome", "📝"),
            Dingbat::Delete => ("FontAwesome", ""), //
            Dingbat::Delete2 => ("FontAwesome", "⌫"),
            Dingbat::Refresh => ("FontAwesome", "🗘"), //🗘
            Dingbat::Add => ("FontAwesome", ""),
            Dingbat::Remove => ("FontAwesome", ""),
            Dingbat::Accept => ("FontAwesome", ""), //
            Dingbat::Cancel => ("FontAwesome", ""), //
            Dingbat::Help => ("FontAwesome", ""),
            Dingbat::Info => ("FontAwesome", ""),
            Dingbat::Settings => ("FontAwesome", ""),
            Dingbat::Settings2 => ("FontAwesome", ""),
            Dingbat::Settings3 => ("FontAwesome", ""),
            Dingbat::Settings4 => ("FontAwesome", ""),
            Dingbat::Settings5 => ("FontAwesome", ""),
            Dingbat::ZoomIn => ("FontAwesome", ""),
            Dingbat::ZoomOut => ("FontAwesome", ""),
            Dingbat::Up => ("FontAwesome", ""),
            Dingbat::Down => ("FontAwesome", ""),
            Dingbat::Left => ("FontAwesome", ""),
            Dingbat::Right => ("FontAwesome", ""),
            Dingbat::Up2 => ("FontAwesome", ""),
            Dingbat::Down2 => ("FontAwesome", ""),
            Dingbat::Left2 => ("FontAwesome", ""),
            Dingbat::Right2 => ("FontAwesome", ""),
            Dingbat::Up3 => ("FontAwesome", ""),
            Dingbat::Down3 => ("FontAwesome", ""),
            Dingbat::Left3 => ("FontAwesome", ""),
            Dingbat::Right3 => ("FontAwesome", ""),
            Dingbat::Up4 => ("FontAwesome", ""),
            Dingbat::Down4 => ("FontAwesome", ""),
            Dingbat::Left4 => ("FontAwesome", ""),
            Dingbat::Right4 => ("FontAwesome", ""),
            Dingbat::Up5 => ("FontAwesome", ""),
            Dingbat::Down5 => ("FontAwesome", ""),
            Dingbat::Left5 => ("FontAwesome", ""),
            Dingbat::Right5 => ("FontAwesome", ""),
            Dingbat::Up6 => ("FontAwesome", ""),
            Dingbat::Down6 => ("FontAwesome", ""),
            Dingbat::Left6 => ("FontAwesome", ""),
            Dingbat::Right6 => ("FontAwesome", ""),
            Dingbat::KickDrum2 => ("FontAwesome", ""),
            Dingbat::SnareDrum2 => ("FontAwesome", "🥁"), //🥁
            Dingbat::SnareRim => ("FontAwesome", "🔘"),
            Dingbat::Wand => ("FontAwesome", ""),
            Dingbat::MagicWand => ("FontAwesome", ""),
            Dingbat::SteelDrum => ("FontAwesome", ""),
            Dingbat::GenericDrum => ("FontAwesome", ""),
            Dingbat::Rim => ("FontAwesome", ""),
            Dingbat::Cymbal2 => ("FontAwesome", ""),
            Dingbat::Hand => ("FontAwesome", "✋"),
            Dingbat::Clap => ("FontAwesome", ""),
            Dingbat::Clap2 => ("FontAwesome", ""),
            Dingbat::Cow => ("FontAwesome", ""),
            Dingbat::Bell => ("FontAwesome", "🔔"),
            Dingbat::Guitar => ("FontAwesome", ""),
            Dingbat::Disc => ("FontAwesome", "💿"),
            Dingbat::Favorite => ("FontAwesome", "♥"), //♥❤🤎💙💚💛🧡💜🖤🤍
            Dingbat::Flag => ("FontAwesome", ""),
            Dingbat::Circle2 => ("FontAwesome", ""), //🟠🟡🟢🟣🟤
            Dingbat::Copyright => ("FontAwesome", ""),
            Dingbat::Trademark => ("FontAwesome", ""),
            Dingbat::Registered => ("FontAwesome", ""),
            Dingbat::Home => ("FontAwesome", ""),
            Dingbat::Cut => ("FontAwesome", "✀"), //✀✂✄
            Dingbat::Share => ("FontAwesome", ""),
            Dingbat::Freeze => ("FontAwesome", ""),
            Dingbat::SquareWave => ("FontAwesome", ""),
            Dingbat::Volume1 => ("FontAwesome", "🔈"),
            Dingbat::Volume3 => ("FontAwesome", "🔊"),
            Dingbat::VolumeMute => ("FontAwesome", ""),
            Dingbat::Mic => ("FontAwesome", ""),
            Dingbat::MicMute => ("FontAwesome", ""),
            Dingbat::PaintBrush => ("FontAwesome", ""),
            Dingbat::PaintBrush2 => ("FontAwesome", ""),
            Dingbat::PaintRoller => ("FontAwesome", ""),
            Dingbat::Spraypaint => ("FontAwesome", ""),
            Dingbat::Stamp => ("FontAwesome", ""),
            Dingbat::Palette => ("FontAwesome", ""),
            Dingbat::Background => ("FontAwesome", ""),
            Dingbat::Image => ("FontAwesome", ""),
            Dingbat::Fire => ("FontAwesome", "🔥"),
            Dingbat::Screwdriver => ("FontAwesome", "🪛"),
            Dingbat::Icons => ("FontAwesome", ""),
        }
    }
}
