pub mod styles;

use crate::util::{ColorConversion, Transition};
use freya::prelude::*;
use material_colors::color::Argb;

#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
fn linear_gradient<T: IntoIterator<Item = (Argb, f32)>>(values: T) -> String {
    format!(
        "linear-gradient({})",
        values
            .into_iter()
            .map(|(color, stop)| format!("rgb({}) {}%", color.to_rgba(), (stop * 100.0) as u8))
            .collect::<Vec<_>>()
            .join(", ")
    )
}

#[derive(Default, PartialEq, Eq, Clone)]
enum ButtonState {
    #[default]
    Rest,
    Hover,
    Pressed,
}

impl ButtonState {
    fn as_color(&self) -> (String, String, String) {
        match self {
            Self::Rest => (
                styles::theme::fill_color::control::DEFAULT.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::control::BORDER),
            ),
            Self::Hover => (
                styles::theme::fill_color::control::SECONDARY.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::control::BORDER),
            ),
            Self::Pressed => (
                styles::theme::fill_color::control::TERTIARY.to_color(),
                styles::theme::fill_color::text::SECONDARY.to_color(),
                styles::theme::stroke_color::control_stroke::DEFAULT.to_color(),
            ),
        }
    }
}

#[component]
pub fn Button() -> Element {
    let mut state = use_signal(Transition::<ButtonState>::default);

    let animation = use_animation(move |ctx| {
        let state = state.read();
        let (from, to) = (state.from.as_color(), state.to.as_color());

        (
            ctx.with(AnimColor::new(&from.0, &to.0).time(83)),
            ctx.with(AnimColor::new(&from.1, &to.1).time(83)),
            to.2,
        )
    });

    let onpointerdown = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(ButtonState::Pressed);
    };

    let onpointerleave = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(ButtonState::Rest);
    };

    let onpointerenter = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(ButtonState::Hover);
    };

    let onpointerup = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(ButtonState::Hover);
    };

    use_effect(use_reactive(&state(), move |state| {
        match (state.from, state.to) {
            (ButtonState::Pressed | ButtonState::Rest, ButtonState::Hover)
            | (ButtonState::Pressed | ButtonState::Hover, ButtonState::Rest)
            | (ButtonState::Hover, ButtonState::Pressed) => animation.start(),
            _ => {}
        }
    }));

    let (background, color, border) = animation.get();

    rsx! {
        rect {
            height: "32",
            width: "120",
            background: "{background.read().as_string()}",
            border: "1 solid {border}",
            corner_radius: "4",
            main_align: "center",
            cross_align: "center",
            padding: "6 12 6 12",

            onpointerenter,
            onpointerleave,
            onpointerdown,
            onpointerup,

            label {
                color: "{color.read().as_string()}",
                font_size: "{styles::text::body::SIZE}",

                "Text"
            }
        }
    }
}

pub fn gallery() -> Element {
    rsx! {
        rect {
            height: "100%",
            width: "100%",
            background: "{styles::theme::stroke_color::card_stroke::DEFAULT_SOLID}",
            padding: "8",

            Button {}
        }
    }
}
