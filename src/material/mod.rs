pub mod button;
pub mod fab;
pub mod ripple;
pub mod switch;

use crate::THEME;
use fab::FAB;
use freya::prelude::*;
use ripple::Ripple;
use switch::Switch;
use button::{Button, ButtonKind};

pub fn gallery() -> Element {
    let mut enabled = use_signal(bool::default);

    rsx! {
        rect {
            height: "100%",
            width: "100%",
            background: "{THEME.surface_container}",
            padding: "8",

            label {
                font_size: "24",
                font_weight: "bold",
                color: "{THEME.primary}",

                "Components Gallery"
            }

            rect {
                direction: "horizontal",

                rect {
                    direction: "vertical",

                    label {
                        font_size: "20",
                        color: "{THEME.secondary}",

                        "Button"
                    }

                    Button {
                        onclick: move |()| {

                        },

                        "HI!"
                    }

                    Button {
                        kind: ButtonKind::Tonal,
                        onclick: move |()| {

                        },

                        "HI!"
                    }

                    Button {
                        kind: ButtonKind::Outlined,
                        onclick: move |()| {

                        },

                        "HI!"
                    }

                    Button {
                        kind: ButtonKind::Text,
                        onclick: move |()| {

                        },

                        "what you know about rollin' down in the deep"
                    }

                    FAB {
                        extended: enabled(),
                        onclick: move |()| {

                        },
                        icon: "A",
                        label: "hiiiiiidasdasdasda"
                    }
                }

                rect {
                    margin: "16 0 0 0",
                    direction: "vertical",

                    label {
                        font_size: "20",
                        color: "{THEME.secondary}",

                        "Switch"
                    }

                    rect {
                        direction: "horizontal",

                        rect {
                            direction: "vertical",

                            label {
                                color: "{THEME.on_surface}",
                                margin: "0 8 0 0",

                                "Default Size"
                            }

                            rect {
                                margin: "8 0 0 0",

                                Switch {
                                    enabled: *enabled.read(),
                                    ontoggled: move |()| enabled.toggle()
                                }
                            }
                        }

                        rect {
                            direction: "vertical",
                            margin: "0 0 0 16",

                            label {
                                color: "{THEME.on_surface}",

                                "Custom Sizes"
                            }

                            rect {
                                width: "148",
                                direction: "vertical",
                                margin: "8 0 0 0",

                                rect {
                                    width: "fill",
                                    direction: "horizontal",
                                    cross_align: "center",

                                    label {
                                        color: "{THEME.on_surface}",
                                        margin: "0 8 0 0",

                                        "24"
                                    }

                                    rect {
                                        width: "fill",
                                        direction: "horizontal",
                                        main_align: "end",
                                        margin: "0 26.5 0 0",

                                        Switch {
                                            height: "24",
                                            enabled: *enabled.read(),
                                            ontoggled: move |()| enabled.toggle()
                                        }
                                    }
                                }

                                rect {
                                    width: "fill",
                                    direction: "horizontal",
                                    cross_align: "center",
                                    margin: "8 0 0 0",

                                    label {
                                        color: "{THEME.on_surface}",
                                        margin: "0 8 0 0",

                                        "48"
                                    }

                                    rect {
                                        width: "fill",
                                        direction: "horizontal",
                                        main_align: "end",
                                        margin: "0 7 0 0",

                                        Switch {
                                            height: "48",
                                            enabled: *enabled.read(),
                                            ontoggled: move |()| enabled.toggle()
                                        }
                                    }
                                }

                                rect {
                                    width: "fill",
                                    direction: "horizontal",
                                    cross_align: "center",
                                    margin: "8 0 0 0",

                                    label {
                                        color: "{THEME.on_surface}",
                                        margin: "0 8 0 0",

                                        "92x48"
                                    }

                                    rect {
                                        width: "fill",
                                        direction: "horizontal",
                                        main_align: "end",

                                        Switch {
                                            width: "92",
                                            height: "48",
                                            enabled: *enabled.read(),
                                            ontoggled: move |()| enabled.toggle()
                                        }
                                    }
                                }
                            }
                        }
                    }

                    label {
                        font_size: "20",
                        margin: "16 0 0 0",
                        color: "{THEME.secondary}",

                        "Ripple (W.I.P)"
                    }

                    rect {
                        width: "256",
                        height: "192",
                        background: "{THEME.primary_container}",
                        corner_radius: "24",
                        overflow: "clip",
                        margin: "8 0 0 0",

                        Ripple { }
                    }
                }
            }
        }
    }
}
