use iced::Element;
use iced::widget::{Text, Button, Column};

use crate::Messages;

pub struct Selector {}

impl Selector {
   pub fn new() -> Self {
        Selector {  }
    }

   pub fn view(&self) -> iced::Element<Messages> {
        Text::from("Chicken nuggets are good").into()
    }
}

pub struct TimerLive {
    timer: i32
}

#[derive(Debug, Clone, Copy)]
enum TLMessagse {
    OnPress
}

impl TimerLive {
    pub fn new() -> Self {
        TimerLive { timer: 10 }
    }

    fn update(&mut self, message: TLMessagse) {
        match message {
            TLMessagse::OnPress => self.timer += 1
        }
    }

    pub fn view(&self) -> iced::Element<Messages> {
        let my_button = Button::new("").on_press(TLMessagse::OnPress);
        let my_text = Text::new(self.timer.to_string());

        Column::new().push(my_button).push(my_text).into()

    } 
}


