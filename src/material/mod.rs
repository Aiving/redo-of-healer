pub mod button;
pub mod fab;
pub mod icon_button;
pub mod rail;
pub mod ripple;
pub mod side_sheet;
pub mod switch;

use crate::THEME;
use button::{Button, ButtonKind};
use icon_button::{IconButton, IconButtonKind};
use crate::util::{WithSpacing, Direction};
use freya::prelude::*;
use rail::NavigationRail;
use ripple::Ripple;
use side_sheet::{SideSheet, SideSheetKind};
use switch::Switch;

pub fn gallery() -> Element {
    let mut enabled = use_signal(bool::default);
    let mut open = use_signal(bool::default);

    rsx! {
        rect {
            height: "100%",
            width: "100%",
            background: "{THEME.surface}",
            direction: "horizontal",

            NavigationRail { },

            rect {
                height: "100%",
                width: "fill",
                direction: "vertical",

                rect {
                    height: "48",
                    width: "fill",
                    cross_align: "center",
                    main_align: "space-between",
                    direction: "horizontal",
                    corner_radius: "16",
                    padding: "8",
                    margin: "16 16 0 16",
                    background: "{THEME.surface_container_low}",

                    label {
                        font_size: "24",
                        font_weight: "bold",
                        color: "{THEME.primary}",

                        "Components Gallery"
                    }

                    IconButton {
                        selected: *open.read(),
                        kind: IconButtonKind::Standard,
                        icon: "settings",
                        onclick: move |()| {
                            open.toggle();
                        },
                    }
                }

                rect {
                    height: "calc(100% - 64)",
                    width: "fill",
                    direction: "horizontal",
        
                    SideSheet {
                        kind: SideSheetKind::Detached,
                        open: *open.read(),

                        rect {
                            height: "100%",
                            width: "100%",
                            padding: "8",
                            margin: "16 0 16 16",
                            corner_radius: "16",
                            background: "{THEME.surface_container_low}",
        
                            rect {
                                direction: "horizontal",
        
                                rect {
                                    direction: "vertical",
        
                                    label {
                                        font_size: "20",
                                        color: "{THEME.secondary}",
        
                                        "Button"
                                    }

                                    rect {
                                        direction: "horizontal",

                                        WithSpacing {
                                            direction: Direction::Horizontal,
                                            spacing: 16,
                                            elements: [
                                                rsx!(Button {
                                                    onclick: move |()| {
                    
                                                    },
                    
                                                    "HI!"
                                                }),

                                                rsx!(Button {
                                                    kind: ButtonKind::Tonal,
                                                    onclick: move |()| {
                    
                                                    },
                    
                                                    "HI!"
                                                }),

                                                rsx!(Button {
                                                    kind: ButtonKind::Outlined,
                                                    onclick: move |()| {
                    
                                                    },
                    
                                                    "HI!"
                                                })
                                            ]
                                        }
                                    }
                                    
                                    rect {
                                        margin: "16 0 0 0",

                                        Button {
                                            kind: ButtonKind::Text,
                                            onclick: move |()| {
            
                                            },
            
                                            "what you know about rollin' down in the deep"
                                        }
                                    }
        
                                    
                                    label {
                                        font_size: "20",
                                        color: "{THEME.secondary}",
        
                                        "Icon Button"
                                    }

                                    rect {
                                        direction: "horizontal",

                                        WithSpacing {
                                            direction: Direction::Horizontal,
                                            spacing: 16,
                                            elements: [
                                                rsx!(IconButton {
                                                    selected: *enabled.read(),
                                                    kind: IconButtonKind::Standard,
                                                    icon: "settings",
                                                    onclick: move |()| {
                                                        open.toggle();
                                                    },
                                                }),

                                                rsx!(IconButton {
                                                    selected: *enabled.read(),
                                                    kind: IconButtonKind::Filled,
                                                    icon: "settings",
                                                    onclick: move |()| {
                                                        open.toggle();
                                                    },
                                                }),

                                                rsx!(IconButton {
                                                    selected: *enabled.read(),
                                                    kind: IconButtonKind::Tonal,
                                                    icon: "settings",
                                                    onclick: move |()| {
                                                        open.toggle();
                                                    },
                                                }),

                                                rsx!(IconButton {
                                                    selected: *enabled.read(),
                                                    kind: IconButtonKind::Outlined,
                                                    icon: "settings",
                                                    onclick: move |()| {
                                                        open.toggle();
                                                    },
                                                })
                                            ]
                                        }
                                    }
                                }
        
                                rect {
                                    margin: "0 0 0 8",
                                    direction: "vertical",
        
                                    label {
                                        font_size: "20",
                                        color: "{THEME.secondary}",
        
                                        "Switch"
                                    }
        
                                    rect {
                                        margin: "8 0 0 0",

                                        Switch {
                                            enabled: *enabled.read(),
                                            ontoggled: move |()| enabled.toggle()
                                        }
                                    }

                                    label {
                                        font_size: "20",
                                        margin: "16 0 0 0",
                                        color: "{THEME.secondary}",

                                        "Ripple (W.I.P)"
                                    }

                                    rect {
                                        width: "240",
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
            }
        }
    }
}
