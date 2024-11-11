use std::fmt::Debug;

use crate::builder::IcedGraphBuilder;
use iced::{widget::*, Element};
use iced_audio::{Normal, NormalParam};
use raug::prelude::*;

pub trait IntoParamVec: Send + Sync + Debug + Clone {
    fn into_param_vec(self) -> Vec<Param>;
}

pub trait Widget: 'static {
    type Message: Send + Sync + Debug + Clone + 'static;
    type Params: IntoParamVec;
    fn view(&self) -> Element<Self::Message>;
    fn update(&mut self, message: Self::Message);
    fn params(&self) -> Self::Params;
}

#[derive(Debug, Clone)]
pub struct ButtonParams {
    pub pressed: Param,
}

impl IntoParamVec for ButtonParams {
    fn into_param_vec(self) -> Vec<Param> {
        vec![self.pressed]
    }
}

pub struct Button {
    label: String,
    param: Param,
}

impl Button {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
            param: Param::new(label, None),
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
        self.param.set(Message::Bang);
    }

    fn params(&self) -> Self::Params {
        ButtonParams {
            pressed: self.param.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct KnobParams {
    pub value: Param,
}

impl Default for KnobParams {
    fn default() -> Self {
        Self {
            value: Param::new("knob", None),
        }
    }
}

impl IntoParamVec for KnobParams {
    fn into_param_vec(self) -> Vec<Param> {
        vec![self.value]
    }
}

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
            .set(self.normal_param.value.as_f32() as Sample);
    }

    fn params(&self) -> Self::Params {
        self.params.clone()
    }
}

impl IcedGraphBuilder {
    pub fn knob(&self) -> (Knob, Vec<Node>) {
        self.add_widget(Knob::new())
    }
}

#[derive(Debug, Clone)]
pub struct DragNumberParams {
    pub value: Param,
}

impl IntoParamVec for DragNumberParams {
    fn into_param_vec(self) -> Vec<Param> {
        vec![self.value]
    }
}

impl Default for DragNumberParams {
    fn default() -> Self {
        Self {
            value: Param::new("drag_number", None),
        }
    }
}

/// A simple widget that allows the user to drag a number.
///
/// The widget can be dragged horizontally to change the value, or double-clicked to type in a new value.
#[derive(Default)]
pub struct DragNumber {
    params: DragNumberParams,
    value: Sample,
}

impl DragNumber {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Widget for DragNumber {
    type Message = Sample;
    type Params = DragNumberParams;
    fn view(&self) -> Element<Sample> {
        let value = self.value.to_string();

        let wid = mouse_area(text_input("", &value));

        wid.into()
    }

    fn update(&mut self, message: Sample) {
        self.value = message;
        self.params.value.set(message);
    }

    fn params(&self) -> Self::Params {
        self.params.clone()
    }
}

impl IcedGraphBuilder {
    pub fn number_dialer(&self) -> (DragNumber, Vec<Node>) {
        self.add_widget(DragNumber::new())
    }
}
