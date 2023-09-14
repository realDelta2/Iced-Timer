use iced::{Sandbox, Settings};
use iced::widget::{text, Container, Row, TextInput}

struct Timer {
    current_page: Pages,
    hour_input: u8,
    minute_input: u16,
    second_input: u16, // in seconds
}

#[derive(Debug, Clone, Copy)]
enum Pages {
    TimerSelecting,
    TimerFinished,
    TimerLive
}

#[derive(Debug, Clone, Copy)]
enum Messages {
    ChangePage(Pages)
}

impl Sandbox for Timer {
    type Message = Messages;

    fn new() -> Self {
        Timer {
            current_page: Pages::TimerSelecting,
            hour_input: 0,
            minute_input: 0,
            second_input: 0
        }
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) {
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match &self.current_page {
            Pages::TimerSelecting => {





            }
            Pages::TimerLive => {text("Timer live")}
            Pages::TimerFinished => {text("timer finished")}
        }.into()
    }
}

fn main() {
    Timer::run(Settings::default()).expect("this is boilerplate")
}
