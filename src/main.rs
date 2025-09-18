use iced::widget::{button, column, row, text};
use iced::{Element, Result};

#[derive(Default)]
struct Plant {
    growth: u8,//holds values 0-255 (needs to account for the 
               //rest of the values after 2 with _
    water: u8, //water capacity left
    reset: u8,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Watered,
    Refill,
    Reset,
    
}

impl Plant {
    fn update(&mut self, message: Message) {
        match message {
            Message::Watered => {
                if self.water > 0 {
                    if self.growth < 4 {
                        self.growth += 1;
                    }
                    self.water -= 1;
                }
            }
            Message::Refill => {
                self.water = 3; //refills water to max
            }
            Message::Reset => {
                self.growth = 0;
                self.water = 0;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let plant = match self.growth {
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
                button("Reset").on_press(Message::Reset),
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
