use iced::{Sandbox, Settings};
v
struct Timer {
    current_page: Pages,
    entered_time: i32 // in seconds
}

#[derive(Debug)]
enum Pages {
    TimerSelecting,
    TimerFinished,
    TimerLive
}

#[derive(Debug)]
enum Messages {
    ChangePage(Pages)
}

impl Sandbox for Timer {
    type Message = Messages;

    fn new() -> Self {
        Timer {
            current_page: Pages::TimerSelecting,
            entered_time: 0
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
        todo!()
    }
}

fn main() {
    Timer::run(Settings::default()).expect("this is boilerplate")
}
