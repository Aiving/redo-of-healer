#![allow(clippy::derive_partial_eq_without_eq)]

use material_colors::color::Argb;
use freya::prelude::*;

pub trait ColorConversion {
    fn to_rgb(&self) -> String;
    fn to_rgba(&self) -> String;
    fn to_color(&self) -> String;
}

impl ColorConversion for Argb {
    fn to_rgb(&self) -> String {
        format!("{}, {}, {}", self.red, self.green, self.blue)
    }

    fn to_rgba(&self) -> String {
        format!(
            "{}, {}, {}, {}",
            self.red, self.green, self.blue, self.alpha
        )
    }

    fn to_color(&self) -> String {
        format!(
            "rgb({}, {}, {}, {})",
            self.red, self.green, self.blue, self.alpha
        )
    }
}

#[derive(Default, PartialEq, Eq)]
pub struct Transition<S> {
    pub from: S,
    pub to: S,
}

impl<S> Transition<S> {
    pub fn to(&mut self, value: S) {
        self.from = std::mem::replace(&mut self.to, value);
    }
}

#[derive(Default, Clone, PartialEq, Eq)]
pub enum Direction {
    #[default]
    Horizontal,
    Vertical,
}

#[component]
pub fn WithSpacing<T: IntoIterator<Item = Element> + PartialEq + 'static>(
    spacing: usize,
    direction: Direction,
    elements: T,
) -> Element {
    let elements = {
        let mut temp = vec![];
        let mut iterator = elements.into_iter().enumerate().peekable();

        while let Some((index, element)) = iterator.next() {
            let margin = match (iterator.peek(), &direction) {
                (None, _) => 0.to_string(),
                (_, Direction::Horizontal) => format!("0 {spacing} 0 0"),
                (_, Direction::Vertical) => format!("0 0 {spacing} 0"),
            };

            temp.push((index, margin, element));
        }

        temp
    };

    rsx! {
        for (index, margin, element) in &elements {
            rect {
                key: "{index}",
                margin: margin.as_str(),

                {&element}
            }
        }
    }
}
