use freya::prelude::*;

use crate::{
    material::ripple::Ripple,
    util::{Direction, WithSpacing},
    THEME,
};

#[component]
fn NavigationRailItem() -> Element {
    rsx! {
        rect {
            height: "56",

            rect {
                width: "56",
                height: "32",
                corner_radius: "16",
                margin: "0 12 0 12",
                main_align: "center",
                cross_align: "center",
                background: "{THEME.secondary_container}",

                rect {
                    position: "absolute",
                    position_left: "-28",
                    height: "32",
                    width: "56",

                    Ripple {
                        color: "{THEME.on_secondary_container}",
                        height: "32",
                        width: "56",
                    }
                }

                label {
                    color: "{THEME.on_secondary_container}",
                    font_size: "24",

                    "RE"
                }
            }

            label {
                margin: "4 0 0 0",
                color: "{THEME.on_surface}",

                "Label"
            }
        }
    }
}

#[component]
pub fn NavigationRail() -> Element {
    rsx! {
        rect {
            height: "100%",
            width: "80",
            background: "{THEME.surface_container_highest}",

            WithSpacing {
                spacing: 12,
                direction: Direction::Vertical,
                elements: [
                    rsx!(NavigationRailItem {}),
                    rsx!(NavigationRailItem {}),
                    rsx!(NavigationRailItem {}),
                ]
            }
        }
    }
}
