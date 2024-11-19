use std::fmt::Debug;

use iced::{widget::*, Element};
use iced_audio::{Normal, NormalParam};
use raug::prelude::*;

/// A trait for converting widget parameter structs into a vector of [`Param`]s.
pub trait IntoParamVec: Send + Sync + Debug + Clone {
    fn into_param_vec(self) -> Vec<Param>;
}

/// A trait for a widget that can be used in both the audio graph and the GUI.
pub trait Widget: 'static {
    type Message: Send + Sync + Debug + Clone + 'static;
    type Params: IntoParamVec;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
    fn params(&self) -> Self::Params;
}

/// Parameters for the button widget.
#[derive(Debug, Clone)]
pub struct ButtonParams {
    pub pressed: Param,
}

impl IntoParamVec for ButtonParams {
    fn into_param_vec(self) -> Vec<Param> {
        vec![self.pressed]
    }
}

/// A simple button widget that sends a boolean value when pressed.
pub struct Button {
    label: String,
    param: Param,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            param: Param::new::<bool>(label, None),
        }
    }
}

impl Widget for Button {
    type Message = ();
    type Params = ButtonParams;
    fn view(&self) -> Element<()> {
        button(self.label.as_str()).on_press(()).into()
    }

    fn update(&mut self, _message: ()) {
        self.param.send(true);
    }

    fn params(&self) -> Self::Params {
        ButtonParams {
            pressed: self.param.clone(),
        }
    }
}

/// Parameters for the knob widget.
#[derive(Debug, Clone)]
pub struct KnobParams {
    pub value: Param,
}

impl Default for KnobParams {
    fn default() -> Self {
        Self {
            value: Param::new::<Float>("knob", None),
        }
    }
}

impl IntoParamVec for KnobParams {
    fn into_param_vec(self) -> Vec<Param> {
        vec![self.value]
    }
}

/// A simple knob widget that sends a float value between 0.0 and 1.0.
#[derive(Default)]
pub struct Knob {
    params: KnobParams,
    normal_param: NormalParam,
    tick_marks: iced_audio::tick_marks::Group,
}

impl Knob {
    pub fn new() -> Self {
        Self {
            params: KnobParams::default(),
            normal_param: NormalParam::default(),
            tick_marks: iced_audio::tick_marks::Group::default(),
        }
    }
}

impl Widget for Knob {
    type Message = Normal;
    type Params = KnobParams;
    fn view(&self) -> Element<Normal> {
        iced_audio::Knob::new(self.normal_param, |value| value)
            .tick_marks(&self.tick_marks)
            .into()
    }

    fn update(&mut self, message: Normal) {
        self.normal_param.update(message);
        self.params
            .value
            .send(self.normal_param.value.as_f32() as Float);
    }

    fn params(&self) -> Self::Params {
        self.params.clone()
    }
}
