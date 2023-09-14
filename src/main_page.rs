use iced::Element;
use iced::widget::{Text}

use crate::Messages;
pub struct selector {}


impl selector {
    fn new() -> Self {
        selector {  }
    }

    fn view() -> iced::Element<Messages> {
        Text::from("Chicken nuggets are good").into()
    }
}

