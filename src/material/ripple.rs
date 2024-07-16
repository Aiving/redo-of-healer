use freya::prelude::*;
use material_colors::color::Argb;

use crate::util::{ColorConversion, Transition};

const INITIAL_ORIGIN_SCALE: f32 = 0.2;
const PADDING: f32 = 10.0;
const SOFT_EDGE_MINIMUM_SIZE: f32 = 75.0;
const SOFT_EDGE_CONTAINER_RATIO: f32 = 0.35;

#[derive(Default, Clone, PartialEq, Eq)]
pub enum RippleState {
    #[default]
    Inactive,
    Hovering,
    Holding,
}

#[derive(Debug, Default, PartialEq, Clone)]
struct RipplePosition {
    start: Point2D,
    end: Point2D,
}

impl RipplePosition {
    fn new(cursor: Point2D, size: f32, (height, width): (f32, f32)) -> Self {
        Self {
            start: Point2D::new(cursor.x - size / 2.0, cursor.y - size / 2.0),
            end: Point2D::new(width / 2.0, height / 2.0),
        }
    }
}

#[derive(Debug, Default, PartialEq, Clone)]
struct RippleInfo {
    position: RipplePosition,
    radius: f32,
    scale: f32,
}

impl RippleInfo {
    fn new(cursor: Point2D, area: Rect<f32, Measure>) -> Self {
        let (size, origin) = ((area.height(), area.width()), area.origin);
        let (radius, scale) = Self::get_size(size);
        let position = RipplePosition::new((cursor - origin).to_point(), radius, size);

        Self {
            position,
            radius,
            scale,
        }
    }

    fn get_size((height, width): (f32, f32)) -> (f32, f32) {
        let max_dim = height.max(width);
        let soft_edge_size = SOFT_EDGE_MINIMUM_SIZE.max(SOFT_EDGE_CONTAINER_RATIO * max_dim);

        let initial_size = (max_dim * INITIAL_ORIGIN_SCALE).floor();
        let hypotenuse = width.hypot(height);
        let max_radius = hypotenuse + PADDING;

        (initial_size, (max_radius + soft_edge_size) / initial_size)
    }
}

#[derive(Props, Default, PartialEq, Eq, Clone)]
pub struct RippleProps {
    width: Option<String>,
    height: Option<String>,
    color: Option<String>,
}

#[component]
#[allow(clippy::needless_pass_by_value)]
pub fn Ripple(props: RippleProps) -> Element {
    let (node_ref, size) = use_node_signal();

    let mut state = use_signal(Transition::<RippleState>::default);
    let mut info = use_signal(RippleInfo::default);

    let opacity_animations = use_animation(move |ctx| ctx.with(AnimNum::new(0.0, 1.0).time(450)));

    let bg = props.color.unwrap_or_else(|| Argb::new(255, 255, 255, 255).to_rgb());
    let (default_bg, hover_bg) = (format!("rgb({bg}, 0)"), format!("rgb({bg}, 25)"));

    let background_animation = use_animation(move |ctx| {
        ctx.with(AnimColor::new(default_bg.as_str(), hover_bg.as_str()).time(300))
    });

    let animations = use_animation(move |ctx| {
        let info = info();

        (
            ctx.with(AnimNum::new(info.radius, info.radius * info.scale).time(450)),
            ctx.with(
                AnimNum::new(
                    info.position.start.x,
                    info.position.end.x - ((info.radius * info.scale) / 2.0),
                )
                .time(450),
            ),
            ctx.with(
                AnimNum::new(
                    info.position.start.y, /*  - (info.radius / 2.0) */
                    info.position.end.y - ((info.radius * info.scale) / 2.0),
                )
                .time(450),
            ),
        )
    });

    let opacity = opacity_animations.get();

    let background = background_animation.get();

    let (radius, x, y) = animations.get();

    let pointerdown = move |event: PointerEvent| {
        info.set(RippleInfo::new(
            event.get_screen_coordinates().to_f32(),
            size.read().area,
        ));

        state.write().to(RippleState::Holding);
    };

    let pointerup = move |_| state.write().to(RippleState::Hovering);
    let pointerenter = move |_| state.write().to(RippleState::Hovering);
    let pointerleave = move |_| state.write().to(RippleState::Inactive);

    use_effect(use_reactive!(|state| {
        let state = state.read();

        match (&state.from, &state.to) {
            (RippleState::Inactive, RippleState::Hovering) => background_animation.start(),
            (RippleState::Inactive | RippleState::Hovering, RippleState::Holding) => {
                opacity_animations.start();
                animations.start();
            }
            (RippleState::Hovering, RippleState::Inactive) => background_animation.reverse(),
            (RippleState::Holding, RippleState::Inactive | RippleState::Hovering) => {
                if state.to == RippleState::Inactive {
                    background_animation.reverse();
                }

                opacity_animations.reverse();
            }
            _ => {}
        }
    }));

    rsx! {
        rect {
            background: "{background.read().as_string()}",
            width: props.width.as_deref().unwrap_or("fill"),
            height: props.height.as_deref().unwrap_or("fill"),
            overflow: "clip",
            position: "absolute",

            onpointerdown: pointerdown,
            onpointerup: pointerup,
            onpointerenter: pointerenter,
            onpointerleave: pointerleave,

            reference: node_ref,

            rect {
                background: "rgb({bg}, 0.5)",
                width: "{radius.read().as_f32()}",
                height: "{radius.read().as_f32()}",
                opacity: "{opacity.read().as_f32()}",
                position: "absolute",
                position_left: "{x.read().as_f32()}",
                position_top: "{y.read().as_f32()}",
                corner_radius: "{radius.read().as_f32() / 2.0}",
            }
        }
    }
}
