use std::fmt::Debug;

use iced::{Element, widget::*};
use iced_audio::{Normal, NormalParam};
use raug::prelude::*;
use raug_ext::prelude::*;

/// A trait for a widget that can be used in both the audio graph and the GUI.
pub trait Widget: 'static {
    type Message: Send + Sync + Debug + Clone + 'static;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
    fn add_params(&self, graph: &Graph) -> Vec<Node>;
}

/// Parameters for the button widget.
#[derive(Debug, Clone)]
pub struct ButtonParams {
    pub pressed: Channel<bool>,
}

/// A simple button widget that sends a boolean value when pressed.
pub struct Button {
    label: String,
    pressed: Channel<bool>,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            pressed: Channel::new(false),
        }
    }
}

impl Widget for Button {
    type Message = ();
    fn view(&self) -> Element<()> {
        button(self.label.as_str()).on_press(()).into()
    }

    fn update(&mut self, _message: ()) {
        self.pressed.send(true).unwrap();
        self.pressed.send(false).unwrap();
    }

    fn add_params(&self, graph: &Graph) -> Vec<Node> {
        vec![graph.node(self.pressed.clone())]
    }
}

/// A simple knob widget that sends a float value between 0.0 and 1.0.
#[derive(Default)]
pub struct Knob {
    value: Channel<f32>,
    normal_param: NormalParam,
    tick_marks: iced_audio::tick_marks::Group,
}

impl Knob {
    pub fn new() -> Self {
        Self {
            value: Channel::new(0.0),
            normal_param: NormalParam::default(),
            tick_marks: iced_audio::tick_marks::Group::default(),
        }
    }
}

impl Widget for Knob {
    type Message = Normal;
    fn view(&self) -> Element<Normal> {
        iced_audio::Knob::new(self.normal_param, |value| value)
            .tick_marks(&self.tick_marks)
            .into()
    }

    fn update(&mut self, message: Normal) {
        self.normal_param.update(message);
        self.value.send(self.normal_param.value.as_f32()).unwrap();
        // self.params
        //     .value
        //     .send(self.normal_param.value.as_f32() as Float);
    }

    fn add_params(&self, graph: &Graph) -> Vec<Node> {
        vec![graph.node(self.value.clone())]
    }
}
