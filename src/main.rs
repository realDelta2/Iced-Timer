use iced::{Application, Settings, Theme, executor, Command};
use iced::widget::{text, Container, Row, TextInput, Button, Column};

struct Timer {
    current_page: Pages,
    hour_input: u8,
    minute_input: u16,
    second_input: u16,

    hour_str_input: String,
    minute_str_input: String,
    second_str_input: String,

    current_time: u32

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

impl Application for Timer {
    type Message = Messages;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();


    fn new(_flags: ()) -> (Timer, Command<Messages>) {
        (Timer {
            current_page: Pages::TimerSelecting,
            hour_input: 0,
            minute_input: 0,
            second_input: 0,
            hour_str_input: String::from(""),
            minute_str_input: String::from(""),
            second_str_input: String::from(""),
            current_time: 0
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) -> Command<Messages> {
        match message {
            Messages::ChangePage(page) => {
                self.current_page = page
            }

            Messages::HourStrInput(input) => {self.hour_str_input = input}
            Messages::MinuteStrInput(input) => {self.minute_str_input = input}
            Messages::SecondStrInput(input) => {self.second_str_input = input}

            Messages::HourInput => {
                let hour: u8 = self.hour_str_input.parse().unwrap();
                self.hour_input = hour;
                self.current_time += (hour as u32 * 60) * 60;
            }
            Messages::MinuteInput => {
                let minute: u16 = self.minute_str_input.parse().unwrap();
                self.minute_input = minute;
                self.current_time += minute as u32 * 60;
            }
            Messages::SecondInput => {
                let second: u16 = self.second_str_input.parse().unwrap();
                self.second_input = second;
                self.current_time += second as u32;
            }
        }
        Command::none()
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

            let time_enter = Button::new("finalize time!").on_press(Messages::ChangePage(Pages::TimerLive));

            let input_row = Row::new().push(hour_selector).push(minute_selector).push(second_selector);
            
            Column::new().push(input_row).push(time_enter).into()

            
            }
            Pages::TimerLive => {

                let time_display = text(format!("time in seconds: {}", &self.current_time));
                time_display.into()



            }
            Pages::TimerFinished => {text("timer finished").into()}
        }
    }
}

fn main() {
    Timer::run(Settings::default()).expect("this is boilerplate")
}
