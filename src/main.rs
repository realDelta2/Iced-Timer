use iced::{Sandbox, Settings};

use pages::{Selector, TimerLive};
mod pages;



struct Timer {
    current_page: Pages,
    entered_time: i32, // in seconds
    timer_pages: (Selector, TimerLive)
}

#[derive(Debug, Clone, Copy)]
pub enum Pages {
    TimerSelecting,
    TimerFinished,
    TimerLive
}

#[derive(Debug, Clone, Copy)]
pub enum Messages {
    ChangePage(Pages)
}

impl Sandbox for Timer {
    type Message = Messages;

    fn new() -> Self {
        Timer {
            current_page: Pages::TimerSelecting,
            entered_time: 0,
            timer_pages: (Selector::new(), TimerLive::new())
        }
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) {
        match message {
            Messages::ChangePage(chosen_page) => self.current_page = chosen_page
        }
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        self.timer_pages.1.view()
    }
}

fn main() {
    Timer::run(Settings::default()).expect("this is boilerplate")
}
