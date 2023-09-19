use iced::{Application, Settings, Theme, executor, Command, Subscription, Length, theme, alignment, Alignment};
use iced::widget::{text, row, TextInput, Button, Column, Space, container, column};
use iced::time;


use std::time::{Duration, Instant};

struct InputData {
    hour_input: u8,
    minute_input: u8,
    second_input: u8,
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
    total_duration: Duration
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
    Void,
    ClearTime,
    ResetTimer,
    Cancel
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
            total_duration: Duration::default(),
            duration: Duration::default(),
        }, Command::none())
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) -> Command<Messages> {
        match message {
            Messages::Cancel => {
                self.state = State::Idle;
                self.duration = self.total_duration;
                self.current_page = Pages::TimerSelecting;
            }
            Messages::ResetTimer => {
                self.duration = self.total_duration;
            }
            Messages::ClearTime => {
                self.input_data.hour_input = 0;
                self.input_data.hour_str_input = String::from("");
                self.input_data.minute_input = 0;
                self.input_data.minute_str_input = String::from("");
                self.input_data.second_input = 0;
                self.input_data.second_str_input = String::from("");
            }
            Messages::Void => {}
            Messages::ChangePage(page) => {
                self.current_page = page;
            }

            Messages::TimeDataInput(entry, value) => {
                match entry {
                    Entries::Hours => {
                        let value_as_number = value.parse::<u8>();
                        if value.is_empty() {
                            self.input_data.hour_str_input = value;
                            self.input_data.hour_input = 0;
                        } else if value_as_number.is_ok() {
                            self.input_data.hour_str_input = value;
                            self.input_data.hour_input = value_as_number.unwrap()
                        }
                    }
                    Entries::Minutes => {
                        let value_as_number = value.parse::<u8>();
                        if value.is_empty() {
                            self.input_data.minute_str_input = value;
                            self.input_data.minute_input = 0;
                        } else if value_as_number.is_ok() {
                            self.input_data.minute_str_input = value;
                            self.input_data.minute_input = value_as_number.unwrap()
                        }
                    }
                    Entries::Seconds => {
                        let value_as_number = value.parse::<u8>();
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
                self.total_duration = self.duration;
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
                }).size(170);
                let minute_select = TextInput::new("0", &self.input_data.minute_str_input)
                .on_input(|input| {
                    Messages::TimeDataInput(Entries::Minutes, input)
                }).size(170);
                let second_select = TextInput::new("0", &self.input_data.second_str_input)
                .on_input(|input| {
                    Messages::TimeDataInput(Entries::Seconds, input)
                }).size(170);

                let select_row = row![
                    Space::with_width(10),
                    hour_select,
                    Space::with_width(10),
                    minute_select,
                    Space::with_width(10), 
                    second_select, 
                    Space::with_width(10)
                ];
                
                
                let clear_time = Button::new("Clear Time")
                .on_press(Messages::ClearTime).style(theme::Button::Destructive);
                let start_timer = Button::new("Start Timer!")
                .on_press(Messages::TimeInput);

                let button_row = row![start_timer, clear_time].spacing(20).align_items(iced::Alignment::Center);
                let button_row_container = container(button_row).center_x().width(Length::Fill);
                let column = column![Space::with_height(40), select_row, Space::with_height(40), button_row_container];

                column.into()
            
            }
            Pages::TimerLive => {
                let total_seconds = self.duration.as_secs_f64().round();
                let hours = (total_seconds / 3600.0).floor();
                let minutes = ((total_seconds.rem_euclid(3600.0)) / 60.0).floor();
                let seconds =  (total_seconds.rem_euclid(3600.0)).rem_euclid(60.0);
            

                let display = text(format!(
                    "{:0>2}:{:0>2}:{:0>2}",
                    hours, minutes, seconds
                    
                ))
                .size(170);

                let display_container = container(display).center_x().width(Length::Fill).center_y().align_y(alignment::Vertical::Center);

                let reset_timer = Button::new("Reset Timer!").on_press(Messages::ResetTimer);
                let pause = Button::new("Pause");
                let cancel_timer = Button::new("Cancel").on_press(Messages::Cancel);

                let button_row = row![reset_timer, cancel_timer].spacing(20).align_items(Alignment::Center);
                let button_row_container = container(button_row).center_x().width(Length::Fill);

                let column = column![Space::with_height(40), display_container, Space::with_height(40), button_row_container];

                column.into()
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
