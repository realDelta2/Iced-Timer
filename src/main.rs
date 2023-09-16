use iced::{Application, Settings, Theme, executor, Command, Subscription};
use iced::widget::{text, Row, TextInput, Button, Column};
use iced::time;

use std::fmt::format;
use std::time::{Duration, Instant};

struct Timer {
    current_page: Pages,
    hour_input: u8,
    minute_input: u16,
    second_input: u16,

    hour_str_input: String,
    minute_str_input: String,
    second_str_input: String,

    current_time: u32,
    ticking_down: bool,

    duration: Duration,
    last_tick: Instant

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

    Tick(Instant),
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
            current_time: 0,
            ticking_down: false,
            last_tick: Instant::now(),
            duration: Duration::from_secs(0)
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) -> Command<Messages> {
        match message {
            Messages::ChangePage(page) => {
                self.current_page = page;
                match self.current_page {
                    Pages::TimerLive => self.ticking_down = true,
                    other => self.ticking_down = false
                }
            }

            Messages::HourStrInput(input) => {self.hour_str_input = input}
            Messages::MinuteStrInput(input) => {self.minute_str_input = input}
            Messages::SecondStrInput(input) => {self.second_str_input = input}

            Messages::HourInput => {
                let hour: u8 = self.hour_str_input.parse().unwrap();
                self.hour_input = hour;
                self.current_time += (hour as u32 * 60) * 60;
                self.duration = Duration::from_secs(self.current_time as u64)
            }
            Messages::MinuteInput => {
                let minute: u16 = self.minute_str_input.parse().unwrap();
                self.minute_input = minute;
                self.current_time += minute as u32 * 60;
                self.duration = Duration::from_secs(self.current_time as u64)
            }
            Messages::SecondInput => {
                let second: u16 = self.second_str_input.parse().unwrap();
                self.second_input = second;
                self.current_time += second as u32;
                self.duration = Duration::from_secs(self.current_time as u64)
            }

            Messages::Tick(_) => {
                println!("count {}", self.current_time);
                self.current_time -= 1;
            }
        }
        Command::none()
    }

    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick = match self.ticking_down {
            true => time::every(Duration::from_secs(1)).map(Messages::Tick),
            false => Subscription::none()
        };
        tick
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

            let time_enter = Button::new("Finalize:").on_press(Messages::ChangePage(Pages::TimerLive));
            let current_time = text(format!("{}", self.current_time));
            let input_row = Row::new().push(hour_selector).push(minute_selector).push(second_selector);
            
            Column::new().push(input_row).push(current_time).push(time_enter).into()

            
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
