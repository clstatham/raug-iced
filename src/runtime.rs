use iced::{
    Application, Command,
    widget::{button, column},
};
use raug::{
    graph::{GraphRunError, RunningGraph},
    prelude::*,
};

use crate::widgets::Widget;

#[derive(Debug, Clone)]
pub enum IcedRuntimeMessage<M> {
    StartAudio,
    StopAudio,
    Message(M),
}

#[derive(Debug, thiserror::Error)]
pub enum IcedRuntimeError {
    #[error("Processor error: {0}")]
    Processor(#[from] GraphRunError),
    #[error("Iced error: {0}")]
    Iced(#[from] iced::Error),
}

pub struct IcedRuntime<T: Widget> {
    graph: Graph,
    main_widget: T,
}

impl<T: Widget> IcedRuntime<T> {
    pub fn new(graph: Graph, main_widget: T) -> Self {
        Self { graph, main_widget }
    }

    pub fn run(self, backend: AudioBackend, device: AudioDevice) -> Result<(), IcedRuntimeError> {
        let Self { graph, main_widget } = self;

        let settings = iced::Settings {
            flags: (graph, main_widget, backend, device),
            id: None,
            window: Default::default(),
            default_font: None,
            default_text_size: 20.0,
            antialiasing: true,
            exit_on_close_request: true,
            text_multithreading: false,
            try_opengles_first: false,
        };

        IcedRuntimeApp::run(settings).map_err(IcedRuntimeError::Iced)
    }
}

pub struct IcedRuntimeApp<T: Widget> {
    handle: Option<RunningGraph>,
    backend: AudioBackend,
    device: AudioDevice,
    graph: Graph,
    main_widget: T,
}

impl<T: Widget> Application for IcedRuntimeApp<T> {
    type Executor = iced::executor::Default;
    type Message = IcedRuntimeMessage<T::Message>;
    type Theme = iced::theme::Theme;
    type Flags = (Graph, T, AudioBackend, AudioDevice);

    fn new((graph, main_widget, backend, device): Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                handle: None,
                backend,
                device,
                graph,
                main_widget,
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "raug".to_string()
    }

    fn theme(&self) -> Self::Theme {
        iced::theme::Theme::Dark
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            IcedRuntimeMessage::StartAudio => {
                if self.handle.is_none() {
                    self.handle = Some(
                        self.graph
                            .play(CpalOut::spawn(&self.backend, &self.device))
                            .unwrap(),
                    );
                }
            }
            IcedRuntimeMessage::StopAudio => {
                if let Some(handle) = self.handle.take() {
                    handle.stop().unwrap();
                }
            }
            IcedRuntimeMessage::Message(message) => {
                self.main_widget.update(message);
            }
        }

        Command::none()
    }

    fn view(&self) -> iced::Element<Self::Message> {
        column![
            button("Start Audio").on_press(IcedRuntimeMessage::StartAudio),
            button("Stop Audio").on_press(IcedRuntimeMessage::StopAudio),
            self.main_widget.view().map(IcedRuntimeMessage::Message),
        ]
        .into()
    }
}
