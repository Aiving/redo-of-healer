#![allow(
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    clippy::cast_possible_truncation
)]

use std::{collections::HashMap, time::Instant};

use curves::{Curve, ICurve};
use dioxus::{
    hooks::{use_memo, use_signal},
    signals::{Memo, Readable, Signal, Writable},
};
use freya::{
    dioxus_core::Task,
    hooks::{use_platform, UsePlatform},
    prelude::spawn,
};
use freya_node_state::Parse;
use skia_safe::Color;

pub mod curves;
// pub mod tweens;

#[derive(Clone, Copy)]
pub enum TransitionDirection {
    Forward,
    Reverse,
}

#[derive(Default, PartialEq, Eq)]
pub struct Context {
    tweens: HashMap<String, Signal<Tween>>,
}

impl Context {
    pub fn add_tween<K: Into<String>, V: Into<TweenValue>>(
        &mut self,
        key: K,
        value: V,
        curve: Curve,
        duration: u64,
    ) {
        let value = value.into();

        self.tweens.insert(
            key.into(),
            Signal::new(Tween::new(value, value).curve(curve).duration(duration)),
        );
    }
}

#[derive(PartialEq, Clone, Copy)]
pub struct Transition {
    context: Memo<Context>,
    is_running: Signal<bool>,
    has_run_yet: Signal<bool>,
    platform: UsePlatform,
    task: Signal<Option<Task>>,
}

impl Transition {
    pub fn set<K: AsRef<str>, V: Into<TweenValue>>(&self, key: K, value: V) {
        let context = self.context.peek();
        let mut tween = *context.tweens.get(key.as_ref()).unwrap();

        tween.write().to(value.into());
    }

    #[must_use]
    pub fn get<V: From<TweenValue>>(&self, key: impl AsRef<str>) -> V {
        self.context
            .read()
            .tweens
            .get(key.as_ref())
            .unwrap()
            .read()
            .value
            .into()
    }

    #[must_use]
    pub fn is_playing(&self) -> bool {
        *self.is_running.read()
    }

    #[must_use]
    pub fn peek_has_run_yet(&self) -> bool {
        *self.has_run_yet.peek()
    }

    pub fn run(&self /* , direction: AnimationDirection */) {
        let ctx = self.context.peek();
        let platform = self.platform;
        let mut is_running = self.is_running;
        let mut ticker = platform.new_ticker();
        let mut values = ctx.tweens.clone();
        let mut has_run_yet = self.has_run_yet;
        // let on_finish = ctx.on_finish;
        let mut task = self.task;

        // Cancel previous animations
        if let Some(task) = task.write().take() {
            task.cancel();
        }

        if !self.peek_has_run_yet() {
            *has_run_yet.write() = true;
        }

        is_running.set(true);

        let animation_task = spawn(async move {
            platform.request_animation_frame();

            let mut index = 0;
            let mut prev_frame = Instant::now();

            // Prepare the animations with the the proper direction
            // for value in values.iter_mut() {
            //     value.write().prepare(direction);
            // }

            loop {
                // Wait for the event loop to tick
                ticker.tick().await;

                platform.request_animation_frame();

                index += prev_frame.elapsed().as_millis();

                let is_finished = values.values().all(|value| value.peek().is_done(index));

                if is_finished {
                    for value in values.values_mut() {
                        value.write().advance(index as f32);
                    }
                        // if OnFinish::Reverse == on_finish {
                    //     // Toggle direction
                    //     direction.toggle();
                    // }
                    // match on_finish {
                    //     OnFinish::Restart | OnFinish::Reverse => {
                    //         index = 0;

                    //         // Restart the animation
                    //         for value in values.iter_mut() {
                    //             value.write().prepare(direction);
                    //         }
                    //     }
                    //     OnFinish::Stop => {
                    //         // Stop if all the animations are finished
                    //         break;
                    //     }
                    // }
                    break;
                }

                // Advance the animations
                for value in values.values_mut() {
                    value.write().advance(index as f32);
                }

                prev_frame = Instant::now();
            }

            is_running.set(false);
            task.write().take();
        });

        // Cancel previous animations
        task.write().replace(animation_task);
    }
}

pub fn use_transition(run: impl Fn(&mut Context) + 'static) -> Transition {
    let platform = use_platform();
    let is_running = use_signal(|| false);
    let has_run_yet = use_signal(|| false);
    let task = use_signal(|| None);

    let context = use_memo(move || {
        let mut context = Context::default();

        run(&mut context);

        context
    });

    Transition {
        context,
        is_running,
        has_run_yet,
        platform,
        task,
    }
}

#[derive(Debug, Clone, Copy)]
pub enum TweenValue {
    Color(Color),
    Number(f32),
}

trait Lerp {
    fn lerp(&self, end: &Self, x: f32) -> Self;
}

impl Lerp for f32 {
    fn lerp(&self, end: &Self, x: f32) -> Self {
        self * (1.0 - x) + end * x
    }
}

impl Lerp for Color {
    #[allow(clippy::cast_sign_loss, clippy::cast_possible_truncation)]
    fn lerp(&self, end: &Self, x: f32) -> Self {
        Self::from_argb(
            f32::from(self.a()).lerp(&f32::from(end.a()), x) as u8,
            f32::from(self.r()).lerp(&f32::from(end.r()), x) as u8,
            f32::from(self.g()).lerp(&f32::from(end.g()), x) as u8,
            f32::from(self.b()).lerp(&f32::from(end.b()), x) as u8,
        )
    }
}

impl Lerp for TweenValue {
    fn lerp(&self, end: &Self, x: f32) -> Self {
        match (self, end) {
            (Self::Color(start), Self::Color(end)) => Self::Color(start.lerp(end, x)),
            (Self::Number(start), Self::Number(end)) => Self::Number(start.lerp(end, x)),
            _ => unimplemented!("haha"),
        }
    }
}

impl From<TweenValue> for String {
    fn from(value: TweenValue) -> Self {
        match value {
            TweenValue::Color(color) => format!(
                "rgb({}, {}, {}, {})",
                color.r(),
                color.g(),
                color.b(),
                color.a()
            ),
            TweenValue::Number(_) => unimplemented!(),
        }
    }
}

impl From<TweenValue> for Color {
    fn from(value: TweenValue) -> Self {
        match value {
            TweenValue::Color(color) => color,
            TweenValue::Number(_) => unimplemented!(),
        }
    }
}

impl From<TweenValue> for f32 {
    fn from(value: TweenValue) -> Self {
        match value {
            TweenValue::Color(_) => unimplemented!(),
            TweenValue::Number(number) => number,
        }
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<&str> for TweenValue {
    fn from(value: &str) -> Self {
        Self::Color(Color::parse(value).unwrap())
    }
}

#[allow(clippy::fallible_impl_from)]
impl From<String> for TweenValue {
    fn from(value: String) -> Self {
        Self::Color(Color::parse(value.as_str()).unwrap())
    }
}

impl From<Color> for TweenValue {
    fn from(value: Color) -> Self {
        Self::Color(value)
    }
}

impl From<f32> for TweenValue {
    fn from(value: f32) -> Self {
        Self::Number(value)
    }
}

pub struct Tween {
    origin: TweenValue,
    destination: TweenValue,
    value: TweenValue,
    duration: f32,
    curve: Curve,
}

impl Tween {
    #[must_use]
    const fn new(origin: TweenValue, destination: TweenValue) -> Self {
        Self {
            origin,
            destination,
            value: origin,
            duration: 0.0,
            curve: Curve::LINEAR,
        }
    }

    fn to(&mut self, value: TweenValue) {
        self.origin = self.value;
        self.destination = value;
    }

    #[must_use]
    const fn curve(mut self, curve: Curve) -> Self {
        self.curve = curve;

        self
    }

    #[must_use]
    const fn duration(mut self, millis: u64) -> Self {
        self.duration = millis as f32;

        self
    }

    #[must_use]
    const fn is_done(&self, time: u128) -> bool {
        time >= self.duration as u128
    }

    fn advance(&mut self, time: f32) {
        self.value = self.origin.lerp(
            &self.destination,
            self.curve
                .transform(time.min(self.duration) / self.duration),
        );
    }
}
