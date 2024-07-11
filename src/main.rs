#![warn(clippy::all, clippy::pedantic, clippy::nursery)]
#![allow(
    clippy::too_many_lines,
    clippy::module_name_repetitions,
    clippy::missing_panics_doc,
    clippy::unreadable_literal
)]

pub mod fluent;
pub mod material;
pub mod util;

use freya::prelude::*;
use material_colors::{color::Argb, scheme::Scheme, theme::ThemeBuilder as MaterialThemeBuilder};
use once_cell::sync::Lazy;

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
    fluent::gallery()
}
