use iced::widget::{button, column, row, text};
use iced::{Element, Result};

#[derive(Default)]
struct Plant {
    growth: u8,//holds values 0-255
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
            }
        }
    }

    fn view(&self) -> Element<Message> {
        let plant = match self.growth {
            0 => " 🫘",
            1 => " 🌱",
            2 => " 🌿",
            3 => " 🪴",
            _ => " 🌳", // "_" accounts for values 4-255
        };

        let water = match self.water {
            0 => "  🚰",     
            1 => " 💧",
            2 => " 💧💧",
            _ => " 💧💧💧", 
        };
        let prompts = match self.growth {
            0 => "      Refill and water the seed to grow!",
            1 => "      Keep watering!",
            2 => "      Still thirsty :(",
            3 => "      Almost there!",
            _ => "      Fully grown! Reset to start over. :))",
        };

        row![
            column![
                button("Refill water").on_press(Message::Refill),
                text(" ").size(20),
                button("Water plant").on_press(Message::Watered),
                text(" ").size(20),
                button("Reset growth").on_press(Message::Reset),
            ],
            text(plant).size(80),
            column![
                text(water).size(40),
                text(" ").size(10),
                text(prompts).size(15),
            ] 
        ]
        .into()
    }
}

pub fn main() -> Result {
    iced::run(Plant::update, Plant::view)
}
