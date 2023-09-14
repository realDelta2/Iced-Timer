use iced::{Sandbox, Settings, widget::text};

struct Timer {
    current_page: Pages,
    entered_time: i32, // in seconds
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
            entered_time: 0
        }
    }

    fn title(&self) -> String {
        String::from("boiler plate")
    }

    fn update(&mut self, message: Self::Message) {
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
       text("here is just a test").into()
    }
}

fn main() {
    Timer::run(Settings::default()).expect("this is boilerplate")
}
