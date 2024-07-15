pub mod styles;

use crate::{
    animation::{self, curves::Curve},
    util::{ColorConversion, Direction, Transition, WithSpacing},
};
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

#[derive(Default, PartialEq, Eq, Clone, Copy)]
enum ButtonKind {
    #[default]
    Standard,
    Accent,
    Subtle,
}

impl ButtonState {
    fn as_color(&self, kind: ButtonKind) -> (String, String, String) {
        match (kind, self) {
            (ButtonKind::Standard, Self::Rest) => (
                styles::theme::fill_color::control::DEFAULT.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::control::BORDER),
            ),
            (ButtonKind::Accent, Self::Rest) => (
                styles::theme::fill_color::accent::DEFAULT.to_color(),
                styles::theme::fill_color::text_on_accent::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::accent_control::BORDER),
            ),
            (ButtonKind::Subtle, Self::Rest) => (
                styles::theme::fill_color::subtle::TRANSPARENT.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                styles::theme::fill_color::subtle::TRANSPARENT.to_color(),
            ),
            (ButtonKind::Standard, Self::Hover) => (
                styles::theme::fill_color::control::SECONDARY.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::control::BORDER),
            ),
            (ButtonKind::Accent, Self::Hover) => (
                styles::theme::fill_color::accent::SECONDARY.to_color(),
                styles::theme::fill_color::text_on_accent::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::accent_control::BORDER),
            ),
            (ButtonKind::Subtle, Self::Hover) => (
                styles::theme::fill_color::subtle::SECONDARY.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                styles::theme::fill_color::subtle::TRANSPARENT.to_color(),
            ),
            (ButtonKind::Standard, Self::Pressed) => (
                styles::theme::fill_color::control::TERTIARY.to_color(),
                styles::theme::fill_color::text::SECONDARY.to_color(),
                styles::theme::stroke_color::control_stroke::DEFAULT.to_color(),
            ),
            (ButtonKind::Accent, Self::Pressed) => (
                styles::theme::fill_color::accent::TERTIARY.to_color(),
                styles::theme::fill_color::text_on_accent::SECONDARY.to_color(),
                styles::theme::stroke_color::control_stroke::ON_ACCENT_DEFAULT.to_color(),
            ),
            (ButtonKind::Subtle, Self::Pressed) => (
                styles::theme::fill_color::subtle::TERTIARY.to_color(),
                styles::theme::fill_color::text::SECONDARY.to_color(),
                styles::theme::fill_color::subtle::TRANSPARENT.to_color(),
            ),
        }
    }
}

#[derive(Props, PartialEq, Eq, Clone, Copy)]
pub struct ButtonProps {
    kind: Option<ButtonKind>,
}

#[component]
pub fn Button(props: ButtonProps) -> Element {
    let mut state = use_signal(ButtonState::default);

    let kind = props.kind.unwrap_or_default();

    let animation = animation::use_animation(move |ctx| {
        let (background, color, border) = state.read().as_color(kind);

        ctx.add_tween("background", background, Curve::LINEAR, 83);
        ctx.add_tween("color", color, Curve::LINEAR, 83);
        ctx.add_tween("border", border, Curve::LINEAR, 83);
    });

    let onpointerdown = move |e: PointerEvent| {
        e.stop_propagation();

        state.set(ButtonState::Pressed);
    };

    let onpointerleave = move |e: PointerEvent| {
        e.stop_propagation();

        state.set(ButtonState::Rest);
    };

    let onpointerenter = move |e: PointerEvent| {
        e.stop_propagation();

        state.set(ButtonState::Hover);
    };

    let onpointerup = move |e: PointerEvent| {
        e.stop_propagation();

        state.set(ButtonState::Hover);
    };

    use_effect(use_reactive!(|state| {
        let (background, color, border) = state.read().as_color(kind);

        animation.set("background", background);
        animation.set("color", color);
        animation.set("border", border);

        animation.run();
    }));

    let [background, color, border]: [String; 3] = [
        animation.get("background"),
        animation.get("color"),
        animation.get("border"),
    ];

    rsx! {
        rect {
            height: "32",
            width: "120",
            background: background.as_str(),
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
                color: color.as_str(),
                font_size: "{styles::text::body::SIZE}",

                "Text"
            }
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone)]
enum SwitchState {
    #[default]
    Rest,
    Hover,
    Pressed,
}

impl SwitchState {
    fn as_color(&self, enabled: bool) -> (String, String, String) {
        match (enabled, self) {
            (true, Self::Rest) => (
                styles::theme::fill_color::accent::DEFAULT.to_color(),
                styles::theme::fill_color::text_on_accent::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::circle::BORDER),
            ),
            (true, Self::Hover) => (
                styles::theme::fill_color::accent::SECONDARY.to_color(),
                styles::theme::fill_color::text_on_accent::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::circle::BORDER),
            ),
            (true, Self::Pressed) => (
                styles::theme::fill_color::accent::TERTIARY.to_color(),
                styles::theme::fill_color::text_on_accent::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::circle::BORDER),
            ),
            (false, Self::Rest) => (
                styles::theme::fill_color::control_alt::SECONDARY.to_color(),
                styles::theme::fill_color::text::SECONDARY.to_color(),
                styles::theme::stroke_color::control_strong_stroke::DEFAULT.to_color(),
            ),
            (false, Self::Hover) => (
                styles::theme::fill_color::control_alt::TERTIARY.to_color(),
                styles::theme::fill_color::text::SECONDARY.to_color(),
                styles::theme::stroke_color::control_strong_stroke::DEFAULT.to_color(),
            ),
            (false, Self::Pressed) => (
                styles::theme::fill_color::control_alt::QUATERNARY.to_color(),
                styles::theme::fill_color::text::SECONDARY.to_color(),
                styles::theme::stroke_color::control_strong_stroke::DEFAULT.to_color(),
            ),
        }
    }
}

#[component]
pub fn Switch() -> Element {
    let mut state = use_signal(Transition::<SwitchState>::default);

    let mut enabled = use_signal(bool::default);

    let animation = animation::use_animation(move |ctx| {
        let enabled = enabled();
        let rest = SwitchState::Rest.as_color(enabled);

        ctx.add_tween("background", rest.0.as_str(), Curve::LINEAR, 83);
        ctx.add_tween("color", rest.1.as_str(), Curve::LINEAR, 83);
        ctx.add_tween("height", 12.0, Curve::LINEAR, 100);
        ctx.add_tween("Width", 12.0, Curve::LINEAR, 100);
    });

    let offset_animation = animation::use_animation(move |ctx| {
        ctx.add_tween("offset", 0.0, Curve::cubic(0.175, 0.885, 0.32, 1.275), 300);
    });

    let onpointerdown = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(SwitchState::Pressed);

        let (background, color, _) = SwitchState::Pressed.as_color(enabled());

        animation.set("background", background);
        animation.set("color", color);
        animation.set("height", 8.0);
        animation.set("Width", 8.0);

        animation.run();
    };

    let onpointerleave = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(SwitchState::Rest);

        let (background, color, _) = SwitchState::Rest.as_color(enabled());

        animation.set("background", background);
        animation.set("color", color);
        animation.set("height", 12.0);
        animation.set("Width", 12.0);

        animation.run();
    };

    let onpointerenter = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(SwitchState::Hover);

        let (background, color, _) = SwitchState::Hover.as_color(enabled());

        animation.set("background", background);
        animation.set("color", color);
        animation.set("height", 14.0);
        animation.set("Width", 14.0);

        animation.run();
    };

    let onpointerup = move |e: PointerEvent| {
        e.stop_propagation();

        enabled.toggle();

        state.write().to(SwitchState::Hover);

        let (background, color, _) = SwitchState::Hover.as_color(enabled());

        animation.set("background", background);
        animation.set("color", color);
        animation.set("height", 14.0);
        animation.set("Width", 14.0);

        if enabled() {
            offset_animation.set("offset", 22.0);
        } else {
            offset_animation.set("offset", 0.0);
        }

        animation.run();
        offset_animation.run();
    };

    let (background, color, height, width, offset_x, border) = (
        animation.get::<_, String>("background"),
        animation.get::<_, String>("color"),
        animation.get::<_, f32>("height"),
        animation.get::<_, f32>("width"),
        offset_animation.get::<_, f32>("offset"),
        state.read().to.as_color(enabled()).2,
    );

    let alt_border = if enabled() {
        background.as_str()
    } else {
        color.as_str()
    };

    let thickness: f32 = if enabled() { 1.0 } else { 0.0 };

    rsx! {
        rect {
            width: "40",
            height: "20",
            corner_radius: "10",
            main_align: "center",
            cross_align: "center",
            background: "{background}",
            border: "1 solid {alt_border}",

            onpointerenter,
            onpointerleave,
            onpointerdown,
            onpointerup,

            rect {
                width: "100%",
                offset_x: "{offset_x}",
                padding: "2",

                rect {
                    width: "16",
                    height: "16",
                    main_align: "center",
                    cross_align: "center",
                    corner_radius: "8",

                    rect {
                        background: "{color}",
                        width: "{width}",
                        height: "{height}",
                        corner_radius: "{height / 2.0}",
                        border: "{thickness} solid {border}",
                    }
                }
            }
        }
    }
}

#[derive(Default, PartialEq, Eq, Clone)]
enum SegmentedButtonState {
    #[default]
    Rest,
    Hover,
    Pressed,
}

impl SegmentedButtonState {
    fn as_color(&self, selected: bool) -> (String, String, String) {
        match (selected, self) {
            (true, _) => (
                styles::theme::fill_color::control::DEFAULT.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                linear_gradient(styles::theme::elevation::control::BORDER),
            ),
            (false, Self::Rest) => (
                styles::theme::fill_color::subtle::TRANSPARENT.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                styles::theme::fill_color::subtle::TRANSPARENT.to_color(),
            ),
            (false, Self::Hover) => (
                styles::theme::fill_color::subtle::SECONDARY.to_color(),
                styles::theme::fill_color::text::PRIMARY.to_color(),
                styles::theme::fill_color::subtle::SECONDARY.to_color(),
            ),
            (false, Self::Pressed) => (
                styles::theme::fill_color::subtle::TERTIARY.to_color(),
                styles::theme::fill_color::text::SECONDARY.to_color(),
                styles::theme::fill_color::subtle::TERTIARY.to_color(),
            ),
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum Position {
    Start,
    Between,
    End,
}

#[derive(Props, PartialEq, Clone)]
pub struct SegmentedButtonProps {
    position: Position,
    selected: bool,
    onclick: EventHandler,
    label: String,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SegmentedButton(props: SegmentedButtonProps) -> Element {
    let mut state = use_signal(Transition::<SegmentedButtonState>::default);

    let animation = use_animation(move |ctx| {
        let state = state.read();
        let (from, to) = (
            state.from.as_color(props.selected),
            state.to.as_color(props.selected),
        );

        (
            ctx.with(AnimColor::new(&from.0, &to.0).time(83)),
            ctx.with(AnimColor::new(&from.1, &to.1).time(83)),
            to.2,
        )
    });

    let onpointerdown = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(SegmentedButtonState::Pressed);
    };

    let onpointerleave = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(SegmentedButtonState::Rest);
    };

    let onpointerenter = move |e: PointerEvent| {
        e.stop_propagation();

        state.write().to(SegmentedButtonState::Hover);
    };

    let onpointerup = move |e: PointerEvent| {
        e.stop_propagation();

        props.onclick.call(());

        state.write().to(SegmentedButtonState::Hover);
    };

    use_effect(use_reactive(&props.selected, move |selected| {
        if !selected {
            state.write().to(SegmentedButtonState::Rest);
        }
    }));

    use_effect(use_reactive!(|state| {
        let state = state.read();

        match (&state.from, &state.to) {
            (
                SegmentedButtonState::Pressed | SegmentedButtonState::Rest,
                SegmentedButtonState::Hover,
            )
            | (
                SegmentedButtonState::Pressed | SegmentedButtonState::Hover,
                SegmentedButtonState::Rest,
            )
            | (SegmentedButtonState::Hover, SegmentedButtonState::Pressed) => animation.start(),
            _ => {}
        }
    }));

    let (background, color, border) = animation.get();

    let (padding, inner_padding) = if props.selected {
        ("0", "5 10 5 10")
    } else {
        ("3 2 3 2", "2 8 2 8")
    };

    let corner_radius = match props.position {
        Position::Start => "4 0 4 0",
        Position::Between => "0",
        Position::End => "0 4 0 4",
    };

    rsx! {
        rect {
            background: "{styles::theme::fill_color::control_alt::SECONDARY.to_color()}",
            padding,
            corner_radius,
            min_width: "40",
            height: "32",

            rect {
                height: "100%",
                min_width: "40",
                background: "{background.read().as_string()}",
                border: "1 solid {border}",
                corner_radius: "4",
                main_align: "center",
                cross_align: "center",
                padding: inner_padding,

                onpointerenter,
                onpointerleave,
                onpointerdown,
                onpointerup,

                label {
                    color: "{color.read().as_string()}",
                    font_size: "{styles::text::body::SIZE}",

                    {props.label.as_str()}
                }
            }
        }
    }
}


#[derive(Props, PartialEq, Eq, Clone)]
pub struct SegmentedControlProps {
    variants: Vec<String>,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn SegmentedControl(props: SegmentedControlProps) -> Element {
    let mut selected = use_signal(usize::default);
    let variants_count = props.variants.len();

    rsx! {
        rect {
            height: "32",
            border: "1 solid {styles::theme::stroke_color::control_stroke::DEFAULT.to_color()}",
            corner_radius: "4",
            direction: "horizontal",

            // onpointerenter,
            // onpointerleave,
            // onpointerdown,
            // onpointerup,

            for (index, variant) in props.variants.iter().enumerate() {
                SegmentedButton {
                    key: "{index}",
                    position: if index == 0 {
                        Position::Start
                    } else if index == (variants_count - 1) {
                        Position::End
                    } else {
                        Position::Between
                    },
                    label: "{variant}",
                    selected: selected() == index,
                    onclick: move |()| selected.set(index)
                }
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

            WithSpacing {
                spacing: 16,
                direction: Direction::Vertical,
                elements: [
                    rsx!(Button {}),
                    rsx!(Button {
                        kind: ButtonKind::Accent
                    }),
                    rsx!(Button {
                        kind: ButtonKind::Subtle
                    }),
                    rsx!(Switch {}),
                    rsx!(SegmentedControl {
                        variants: vec![
                            "Hello".into(),
                            "From".into(),
                            "This".into(),
                            "Wonderful".into(),
                            "World".into(),
                        ]
                    })
                ],
            }
        }
    }
}
