use iced::widget::{button, row, text};
use iced::{Element, Result};

#[derive(Default)]
struct Plant {
    stage: u8,//holds values 0-255 (needs to account for the 
               //rest of the values after 2 with _)
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Watered,
}

impl Plant {
    fn update(&mut self, message: Message) {
        if let Message::Watered = message {
            if self.stage < 4 {
                self.stage += 1;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let plant = match self.stage {
            0 => " 🫘",
            1 => " 🌱",
            2 => " 🌿",
            3 => " 🪴",
            _ => " 🌳", //accounts for values 2-255 (u8)
        };
        row![
            button("Water").on_press(Message::Watered), 
            text(plant).size(80), 
            ]
        .into()
    }
}

pub fn main() -> Result {
    iced::run(Plant::update, Plant::view)
}
