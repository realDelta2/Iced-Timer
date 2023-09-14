use iced::{Sandbox, Settings};
use iced::widget::{text, Container, Row, TextInput};

struct Timer {
    current_page: Pages,
    hour_input: u8,
    minute_input: u16,
    second_input: u16, // in seconds

    hour_str_input: String,
    minute_str_input: String,
    second_str_input: String

}

#[derive(Debug, Clone, Copy)]
enum Pages {
    TimerSelecting,
    TimerFinished,
    TimerLive
}

#[derive(Debug, Clone)]
enum Messages {
    ChangePage(Pages),

    HourStrInput(String),
    MinuteStrInput(String),
    SecondStrInput(String),

    HourInput,
    MinuteInput,
    SecondInput,
}

impl Sandbox for Timer {
    type Message = Messages;

    fn new() -> Self {
        Timer {
            current_page: Pages::TimerSelecting,
            hour_input: 0,
            minute_input: 0,
            second_input: 0,
            hour_str_input: String::from(""),
            minute_str_input: String::from(""),
            second_str_input: String::from("")
        }
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Messages::ChangePage(page) => {}

            Messages::HourStrInput(input) => {self.hour_str_input = input}
            Messages::MinuteStrInput(input) => {self.minute_str_input = input}
            Messages::SecondStrInput(input) => {self.second_str_input = input}

            Messages::HourInput => {
                let hour: u8 = self.hour_str_input.parse().unwrap();
                self.hour_input = hour;
            }
            Messages::MinuteInput => {
                let minute: u16 = self.minute_str_input.parse().unwrap();
                self.minute_input = minute;
            }
            Messages::SecondInput => {
                let second: u16 = self.second_str_input.parse().unwrap();
                self.second_input = second
            }
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match &self.current_page {
            Pages::TimerSelecting => {
            let hour_selector = TextInput::new("0", &self.hour_str_input)
            .on_input(|input| {Messages::HourStrInput(input)})
            .on_submit(Messages::HourInput);


            let minute_selector = TextInput::new("0", &self.minute_str_input)
            .on_input(|input| {Messages::MinuteStrInput(input)})
            .on_submit(Messages::MinuteInput);


            let second_selector = TextInput::new("0", &self.second_str_input)
            .on_input(|input| {Messages::SecondStrInput(input)})
            .on_submit(Messages::SecondInput);

            Row::new().push(hour_selector).push(minute_selector).push(second_selector).into()

            
            }
            Pages::TimerLive => {text("Timer live").into()}
            Pages::TimerFinished => {text("timer finished").into()}
        }
    }
}

fn main() {
    Timer::run(Settings::default()).expect("this is boilerplate")
}
