pub mod text {
    pub mod caption {
        pub const SIZE: usize = 12;
        pub const LINE_HEIGHT: usize = 12;
    }

    pub mod body {
        pub const SIZE: usize = 14;
        pub const LINE_HEIGHT: usize = 20;
    }

    pub mod body_strong {
        pub const SIZE: usize = 14;
        pub const LINE_HEIGHT: usize = 20;
    }

    pub mod body_large {
        pub const SIZE: usize = 18;
        pub const LINE_HEIGHT: usize = 24;
    }

    pub mod subtitle {
        pub const SIZE: usize = 20;
        pub const LINE_HEIGHT: usize = 28;
    }

    pub mod title {
        pub const SIZE: usize = 28;
        pub const LINE_HEIGHT: usize = 36;
    }

    pub mod title_large {
        pub const SIZE: usize = 40;
        pub const LINE_HEIGHT: usize = 52;
    }

    pub mod display {
        pub const SIZE: usize = 68;
        pub const LINE_HEIGHT: usize = 92;
    }
}

pub mod theme {
    pub mod fill_color {
        pub mod text {
            use material_colors::color::Argb;

            pub const PRIMARY: Argb = Argb::new(255, 255, 255, 255);
            pub const SECONDARY: Argb = Argb::new(200, 255, 255, 255);
            pub const TERTIARY: Argb = Argb::new(139, 255, 255, 255);
            pub const DISABLED: Argb = Argb::new(93, 255, 255, 255);
        }

        pub mod accent_text {
            use material_colors::color::Argb;

            pub const PRIMARY: Argb = Argb::new(255, 153, 235, 255);
            pub const SECONDARY: Argb = Argb::new(200, 153, 235, 255);
            pub const TERTIARY: Argb = Argb::new(139, 96, 205, 255);
            pub const DISABLED: Argb = Argb::new(93, 255, 255, 255);
        }

        pub mod text_on_accent {
            use material_colors::color::Argb;

            pub const PRIMARY: Argb = Argb::new(255, 0, 0, 0);
            pub const SECONDARY: Argb = Argb::new(128, 0, 0, 0);
            pub const TERTIARY: Argb = Argb::new(135, 255, 255, 255);
            pub const SELECTED_TEXT: Argb = Argb::new(255, 255, 255, 255);
        }

        pub mod control {
            use material_colors::color::Argb;

            pub const TRANSPARENT: Argb = Argb::new(0, 255, 255, 255);
            pub const DEFAULT: Argb = Argb::new(15, 255, 255, 255);
            pub const SECONDARY: Argb = Argb::new(21, 255, 255, 255);
            pub const TERTIARY: Argb = Argb::new(8, 255, 255, 255);
            pub const QUATERNARY: Argb = Argb::new(15, 255, 255, 255);
            pub const DISABLED: Argb = Argb::new(11, 255, 255, 255);
            pub const INPUT_ACTIVE: Argb = Argb::new(179, 30, 30, 30);
        }

        pub mod control_strong {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(139, 255, 255, 255);
            pub const DISABLED: Argb = Argb::new(63, 255, 255, 255);
        }

        pub mod control_alt {
            use material_colors::color::Argb;

            pub const TRANSPARENT: Argb = Argb::new(0, 255, 255, 255);
            pub const SECONDARY: Argb = Argb::new(26, 0, 0, 0);
            pub const TERTIARY: Argb = Argb::new(11, 255, 255, 255);
            pub const QUATERNARY: Argb = Argb::new(18, 255, 255, 255);
            pub const DISABLED: Argb = Argb::new(0, 255, 255, 255);
        }

        pub mod subtle {
            use material_colors::color::Argb;

            pub const TRANSPARENT: Argb = Argb::new(0, 255, 255, 255);
            pub const SECONDARY: Argb = Argb::new(15, 255, 255, 255);
            pub const TERTIARY: Argb = Argb::new(11, 255, 255, 255);
            pub const DISABLED: Argb = Argb::new(0, 255, 255, 255);
        }

        pub mod accent {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(255, 96, 205, 255);
            pub const SECONDARY: Argb = Argb::new(230, 96, 205, 255);
            pub const TERTIARY: Argb = Argb::new(204, 96, 205, 255);
            pub const DISABLED: Argb = Argb::new(40, 255, 255, 255);
            pub const SELECTED_TEXT_BACKGROUND: Argb = Argb::new(255, 0, 120, 212);
        }

        pub mod control_solid {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(255, 69, 69, 69);
        }

        pub mod system {
            use material_colors::color::Argb;

            pub const CRTICIAL: Argb = Argb::new(255, 255, 153, 164);
            pub const SUCCESS: Argb = Argb::new(255, 108, 203, 95);
            pub const ATTENTION: Argb = Argb::new(255, 96, 205, 225);
            pub const CAUTION: Argb = Argb::new(255, 252, 225, 0);
            pub const ATTENTION_BACKGROUND: Argb = Argb::new(8, 255, 255, 255);
            pub const SUCCESS_BACKGROUND: Argb = Argb::new(255, 57, 61, 27);
            pub const CAUTION_BACKGROUND: Argb = Argb::new(255, 67, 53, 25);
            pub const CRTICIAL_BACKGROUND: Argb = Argb::new(255, 68, 39, 38);
            pub const NEUTRAL: Argb = Argb::new(139, 255, 255, 255);
            pub const NEUTRAL_BACKGROUND: Argb = Argb::new(8, 255, 255, 255);
            pub const SOLID_ATTENTION_BACKGROUND: Argb = Argb::new(255, 46, 46, 46);
            pub const SOLID_NEUTRAL: Argb = Argb::new(255, 157, 157, 157);
            pub const SOLID_NEUTRAL_BACKGROUND: Argb = Argb::new(255, 46, 46, 46);
        }

        pub mod control_on_image {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(179, 28, 28, 28);
            pub const SECONDARY: Argb = Argb::new(255, 26, 26, 26);
            pub const TERTIARY: Argb = Argb::new(255, 19, 19, 19);
            pub const DISABLED: Argb = Argb::new(0, 30, 30, 30);
        }
    }

    pub mod elevation {
        pub mod control {
            use material_colors::color::Argb;

            pub const BORDER: [(Argb, f32); 2] = [
                (Argb::new(23, 255, 255, 255), 0.0),
                (Argb::new(18, 255, 255, 255), 0.1)
            ];
        }

        pub mod circle {
            use material_colors::color::Argb;

            pub const BORDER: [(Argb, f32); 2] = [
                (Argb::new(23, 255, 255, 255), 0.0),
                (Argb::new(18, 255, 255, 255), 0.5)
            ];
        }

        pub mod text_control {
            use material_colors::color::Argb;

            pub const BORDER: [(Argb, f32); 3] = [
                (Argb::new(20, 255, 255, 255), 1.0),
                (Argb::new(138, 255, 255, 255), 1.0),
                (Argb::new(138, 255, 255, 255), 1.0)
            ];

            pub const BORDER_FOCUSED: [(Argb, f32); 2] = [
                (Argb::new(20, 255, 255, 255), 0.97),
                (Argb::new(255, 76, 194, 255), 0.97)
            ];
        }

        pub mod accent_control {
            use material_colors::color::Argb;

            pub const BORDER: [(Argb, f32); 2] = [
                (Argb::new(20, 255, 255, 255), 0.91),
                (Argb::new(36, 0, 0, 0), 1.0)
            ];
        }
    }

    pub mod stroke_color {
        pub mod control_stroke {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(18, 255, 255, 255);
            pub const ON_ACCENT_DEFAULT: Argb = Argb::new(20, 255, 255, 255);
            pub const SECONDARY: Argb = Argb::new(24, 255, 255, 255);
            pub const ON_ACCENT_SECONDARY: Argb = Argb::new(36, 0, 0, 0);
            pub const ON_ACCENT_TERTIARY: Argb = Argb::new(55, 0, 0, 0);
            pub const ON_ACCENT_DISABLED: Argb = Argb::new(51, 0, 0, 0);
            pub const FOR_STRONG_FILL_WHEN_ON_IMAGE: Argb = Argb::new(107, 0, 0, 0);
        }

        pub mod control_strong_stroke {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(154, 255, 255, 255);
            pub const DISABLED: Argb = Argb::new(40, 255, 255, 255);
        }

        pub mod divider_stroke {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(21, 255, 255, 255);
        }

        pub mod surface_stroke {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(102, 117, 117, 117);
            pub const FLYOUT: Argb = Argb::new(51, 0, 0, 0);
        }

        pub mod card_stroke {
            use material_colors::color::Argb;

            pub const DEFAULT: Argb = Argb::new(26, 0, 0, 0);
            pub const DEFAULT_SOLID: Argb = Argb::new(255, 28, 28, 28);
        }

        pub mod focus_stroke {
            use material_colors::color::Argb;

            pub const OUTER: Argb = Argb::new(255, 255, 255, 255);
            pub const INNER: Argb = Argb::new(179, 0, 0, 0);
        }
    }
}