use iced::Element;
use iced::widget::{Text};

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

pub struct TimerLive {}

impl TimerLive {
    pub fn new() -> Self {
        TimerLive {  }
    }

    pub fn view(&self) -> iced::Element<Messages> {
        Text::from("Chicken nuggets are not good").into()
    } 
}


