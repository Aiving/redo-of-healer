use material_colors::color::Argb;

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
        format!("{}, {}, {}, {}", self.red, self.green, self.blue, self.alpha)
    }

    fn to_color(&self) -> String {
        format!("rgb({}, {}, {}, {})", self.red, self.green, self.blue, self.alpha)
    }
}

#[derive(Default, PartialEq, Eq, Clone)]
pub struct Transition<T> {
    pub from: T,
    pub to: T,
}

impl<T: Clone> Transition<T> {
    pub fn new(init: T) -> Self {
        Self {
            from: init.clone(),
            to: init,
        }
    }

    pub fn to(&mut self, value: T) {
        self.from = self.to.clone();
        self.to = value;
    }
}
