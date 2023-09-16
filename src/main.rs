use iced::{Application, Settings, Theme, executor, Command, Subscription, Length};
use iced::widget::{text, Row, TextInput, Button, Column};
use iced::time;

use std::fmt::format;
use std::time::{Duration, Instant};

struct Timer {
    current_page: Pages,


    time_input: u32,
    time_str_input: String,

    current_time: u32,
    ticking_down: bool,

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

    TimeStrInput(String),
    TimeInput,

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
            time_str_input: String::from("0:0:0"),
            time_input: 0,
            current_time: 0,
            ticking_down: false,
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

            Messages::TimeStrInput(Str) => {
                self.time_str_input = Str;
            }

            Messages::TimeInput => {
                let time_string = &self.time_str_input;
                let time_vec: Vec<&str> = time_string.split(':').collect();

                println!("{:?}", time_vec);


            }




            Messages::Tick(_) => {
                println!("count {}", self.current_time);
                self.current_time -= 1;

                if self.current_time == 0 {
                    self.ticking_down = false;
                    self.current_page = Pages::TimerFinished;
                }
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
            let time_selector = TextInput::new("0:0:0", &self.time_str_input)
            .on_input(|input| {Messages::TimeStrInput(input)})
            .on_submit(Messages::TimeInput).width(Length::Fill);

            let time_enter = Button::new("Finalize:").on_press(Messages::ChangePage(Pages::TimerLive));
            let current_time = text(format!("{}", self.current_time));
            
            Column::new().push(time_selector).push(current_time).push(time_enter).into()

            
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
