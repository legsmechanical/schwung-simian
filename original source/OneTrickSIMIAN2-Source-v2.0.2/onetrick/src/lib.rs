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

pub mod buffer;
pub mod faust;
pub mod midi;
pub mod params;
pub mod preset;
pub mod preset_manager;
pub mod process;

#[cfg(feature = "egui")]
pub mod egui;
#[cfg(feature = "egui")]
pub mod palette;
#[cfg(feature = "egui")]
pub mod color;

#[allow(dead_code)]
pub mod prelude {
    pub use super::buffer::*;
    pub use super::faust::*;
    pub use super::midi::*;
    pub use super::params::*;
    pub use super::preset::*;
    pub use super::preset_manager::*;
    pub use super::process::*;

    #[cfg(feature = "egui")]
    pub use super::egui::*;
    #[cfg(feature = "egui")]
    pub use super::palette::*;
    #[cfg(feature = "egui")]
    pub use super::color::*;
}
