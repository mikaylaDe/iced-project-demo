use iced::widget::{button, column, row, text};
use iced::{Element, Result};

#[derive(Default)]
struct Plant {
    stage: u8,//holds values 0-255 (needs to account for the 
               //rest of the values after 2 with _
    water: u8, //water capacity left
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Watered,
    Refill,
    
}

impl Plant {
    fn update(&mut self, message: Message) {
        match message {
            Message::Watered => {
                if self.water > 0 {
                    if self.stage < 4 {
                        self.stage += 1;
                    }
                    self.water -= 1;
                }
            }
            Message::Refill => {
                self.water = 3; //refills water to max
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let plant = match self.stage {
            0 => " 🫘",
            1 => " 🌱",
            2 => " 🌿",
            3 => " 🪴",
            _ => " 🌳",
        };

        let water = match self.water {
            0 => " 🚰",     
            1 => " 💧",
            2 => " 💧💧",
            _ => " 💧💧💧", 
        };

        row![
            column![
                button("Water").on_press(Message::Watered),
                button("Refill").on_press(Message::Refill),
                //button("Reset plant"),on_press(Message:Reset),
            ],
            text(plant).size(80),
            text(water).size(40),
        ]
        .into()
    }
}

pub fn main() -> Result {
    iced::run(Plant::update, Plant::view)
}
