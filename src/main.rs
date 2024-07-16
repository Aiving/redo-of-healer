#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::too_many_lines,
    clippy::too_many_arguments,
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::unreadable_literal
)]

pub mod animation;
pub mod fluent;
pub mod material;
pub mod util;

use freya::prelude::*;
use material_colors::{color::Argb, scheme::Scheme, theme::ThemeBuilder as MaterialThemeBuilder};
use once_cell::sync::Lazy;

pub mod icons {
    pub const FERRIS: &[u8] = include_bytes!("../ferris.svg");
    const SETTINGS: &[u8] = include_bytes!("../settings_24px.svg");
    const SETTINGS_FILLED: &[u8] = include_bytes!("../settings_filled_24px.svg");

    #[must_use]
    pub const fn settings(filled: bool) -> &'static [u8] {
        if filled {
            SETTINGS_FILLED
        } else {
            SETTINGS
        }
    }
}

static THEME: Lazy<Scheme> = Lazy::new(|| {
    MaterialThemeBuilder::with_source(Argb::from_u32(0x00FF00))
        .build()
        .schemes
        .dark
});

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    material::gallery()
}
