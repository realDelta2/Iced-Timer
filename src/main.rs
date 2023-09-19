use iced::{Application, Settings, Theme, executor, Command, Subscription, Length, Element};
use iced::widget::{text, Row, TextInput, Button, Column, button, Container};
use iced::time;


use std::time::{Duration, Instant};

struct InputData {
    hour_input: u16,
    minute_input: u16,
    second_input: u16,
    hour_str_input: String,
    minute_str_input: String,
    second_str_input: String,
    total_input: u32
}

struct Timer {
    current_page: Pages,
    input_data: InputData,
    duration: Duration,
    state: State,
}

#[derive(Debug, Clone, Copy)]
enum Pages {
    TimerSelecting,
    TimerFinished,
    TimerLive,
    TimerError
}

#[derive(Debug, Clone)]
enum Entries {
    Hours,
    Minutes,
    Seconds
}

#[derive(Debug, Clone)]
enum Messages {
    ChangePage(Pages),
    TimeDataInput(Entries, String),
    TimeInput,
    Tick(Instant),
    Void
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
            input_data: InputData { 
                hour_input: 0,
                minute_input: 0,
                second_input: 0,
                total_input: 0,
                hour_str_input: String::from(""),
                minute_str_input: String::from(""),
                second_str_input: String::from(""),
         },
            state: State::Idle,
            duration: Duration::default(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) -> Command<Messages> {
        match message {
            Messages::Void => {}
            Messages::ChangePage(page) => {
                self.current_page = page;
            }

            Messages::TimeDataInput(entry, value) => {
                match entry {
                    Entries::Hours => {
                        let value_as_number = value.parse::<u16>();
                        if value.is_empty() {
                            self.input_data.hour_str_input = value;
                            self.input_data.hour_input = 0;
                        } else if value_as_number.is_ok() {
                            self.input_data.hour_str_input = value;
                            self.input_data.hour_input = value_as_number.unwrap()
                        }
                    }
                    Entries::Minutes => {
                        let value_as_number = value.parse::<u16>();
                        if value.is_empty() {
                            self.input_data.minute_str_input = value;
                            self.input_data.minute_input = 0;
                        } else if value_as_number.is_ok() {
                            self.input_data.minute_str_input = value;
                            self.input_data.minute_input = value_as_number.unwrap()
                        }
                    }
                    Entries::Seconds => {
                        let value_as_number = value.parse::<u16>();
                        if value.is_empty() {
                            self.input_data.second_str_input = value;
                            self.input_data.second_input = 0;
                        } else if value_as_number.is_ok() {
                            self.input_data.second_str_input = value;
                            self.input_data.second_input = value_as_number.unwrap()
                        }
                    }
                }
            }

            Messages::TimeInput => {
                let mut error_occured = false;
                let time_string = format!("{}:{}:{}", self.input_data.hour_input, self.input_data.minute_input, self.input_data.second_input);
                let time_vec: Vec<u32> = time_string.split(':').map(|section| {
                    match section.parse::<u32>() {
                        Ok(data) => {data}
                        Err(_) => {
                            error_occured = true;
                            self.current_page = Pages::TimerError;
                            self.state = State::Idle;
                            0
                        }
                    }
                }).collect();

                self.input_data.total_input = (time_vec[0] * 60 * 60) + (time_vec[1] * 60) + time_vec[2];
                self.duration = Duration::from_secs(self.input_data.total_input as u64);
                if !error_occured {
                    self.current_page = Pages::TimerLive;
                    self.state = State::Ticking { last_tick: Instant::now() }
                }
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
                
                let hour_select = TextInput::new("0", &self.input_data.hour_str_input)
                .on_input(|input| {
                    Messages::TimeDataInput(Entries::Hours, input)
                }).size(200);


                let minute_select = TextInput::new("0", &self.input_data.minute_str_input)
                .on_input(|input| {
                    Messages::TimeDataInput(Entries::Minutes, input)
                }).size(200);

                let second_select = TextInput::new("0", &self.input_data.second_str_input)
                .on_input(|input| {
                    Messages::TimeDataInput(Entries::Seconds, input)
                }).size(200);

                let select_row = Row::new().push(hour_select).push(minute_select).push(second_select);
                let start_timer = Button::new("Start Timer!")
                .on_press(Messages::TimeInput);
                let column = Column::new().push(select_row).push(start_timer);


                column.into()
            
            }
            Pages::TimerLive => {
                let total_seconds = self.duration.as_secs_f64().round();
                let hours = (total_seconds / 3600.0).floor();
                let minutes = ((total_seconds.rem_euclid(3600.0)) / 60.0).ceil();
                let seconds =  (total_seconds.rem_euclid(3600.0)).rem_euclid(60.0);
                let time_display = text(format!("h:{} m:{} s: {}", hours, minutes, seconds));
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
