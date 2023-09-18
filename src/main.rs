use iced::{Application, Settings, Theme, executor, Command, Subscription, Length};
use iced::widget::{text, Row, TextInput, Button, Column, button};
use iced::time;


use std::time::{Duration, Instant};

struct Timer {
    current_page: Pages,

    time_input: u32,
    time_str_input: String,

    duration: Duration,
    state: State

}

#[derive(Debug, Clone, Copy)]
enum Pages {
    TimerSelecting,
    TimerFinished,
    TimerLive,
    TimerError
}

#[derive(Debug, Clone)]
enum Messages {
    ChangePage(Pages),

    TimeStrInput(String),
    TimeInput,

    Tick(Instant),
    
}

enum State {
    Idle,
    Ticking { last_tick: Instant }
}

impl Application for Timer {
    type Message = Messages;
    type Theme = Theme;
    type Executor = executor::Default;
    type Flags = ();


    fn new(_flags: ()) -> (Timer, Command<Messages>) {
        (Timer {
            current_page: Pages::TimerSelecting,
            time_str_input: String::from("0:0:0"),
            time_input: 0,
            state: State::Idle,
            duration: Duration::default()
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) -> Command<Messages> {
        match message {
            Messages::ChangePage(page) => {
                self.current_page = page;
            }

            Messages::TimeStrInput(Str) => {
                self.time_str_input = Str;
            }

            Messages::TimeInput => {
                let time_string = &self.time_str_input;
                let time_vec: Vec<u32> = time_string.split(':').map(|section| {
                    match section.parse::<u32>() {
                        Ok(data) => {data}
                        Err(_) => {
                            self.current_page = Pages::TimerError;
                            self.state = State::Idle;
                            0
                        }
                    }
                }).collect();



                self.time_input = (time_vec[0] * 60 * 60) + (time_vec[1] * 60) + time_vec[2];
                self.duration = Duration::from_secs(self.time_input as u64);

                self.current_page = Pages::TimerLive;
                self.state = State::Ticking { last_tick: Instant::now() }
            }



            Messages::Tick(now) => {
                if let State::Ticking { last_tick } = &mut self.state {
                    match self.duration.checked_sub(now.duration_since(*last_tick)) {
                        Some(new_duration) => {
                            self.duration = new_duration;
                            *last_tick = now;
                        }
                        None => {
                            self.state = State::Idle;
                            self.current_page = Pages::TimerFinished
                        }
                    }
                }
            }

        }
        Command::none()
    }


    fn subscription(&self) -> iced::Subscription<Self::Message> {
        let tick = match self.state {
            State::Ticking { .. } => time::every(Duration::from_millis(100)).map(Messages::Tick),
            State::Idle => Subscription::none()
        };
        tick
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        match &self.current_page {
            Pages::TimerSelecting => {
            let time_selector = TextInput::new("0:0:0", &self.time_str_input)
            .on_input(|input| {Messages::TimeStrInput(input)}).width(Length::Fill);

            let time_enter = Button::new("Finalize:").on_press(Messages::TimeInput);
            let current_time = text(format!("{:?}", self.duration));
            
            Column::new().push(time_selector).push(current_time).push(time_enter).into()

            
            }
            Pages::TimerLive => {

                let time_display = text(format!("time in seconds: {:?}", &self.duration.as_secs()));
                time_display.into()



            }
            Pages::TimerFinished => {
                let restart_button = Button::new("restart").on_press(Messages::ChangePage(Pages::TimerSelecting));
                restart_button.into()
            }
            Pages::TimerError => {
                let go_back_button = Button::new("retry").on_press(Messages::ChangePage(Pages::TimerSelecting));
                go_back_button.into()
            }
        }
    }
}

fn main() {
    Timer::run(Settings::default()).expect("this is boilerplate")
}
