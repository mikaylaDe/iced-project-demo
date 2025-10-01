# Plant Growth Demo 

A small Rust project to explore the [Iced](https://github.com/iced-rs/iced) GUI library.
This project simulates a plant that the user can water, refill, and reset, with each action changing the state of the plant and updating the UI. 
🫘 -> 🌱 -> 🌿 -> 🪴 -> 🌳

## How it works

* **`Plant` struct** tracks growth stage and water level
* **Messages** (`Watered`, `Refill`, `Reset`)  state updates
* **update()** changes state
* **view()** displays buttons, plant emojis, water status, and prompts

## Installation

Requirements:

* [Rust](https://www.rust-lang.org/tools/install) 
* Cargo

To clone repo and run:
```bash
git clone https://github.com/mikaylaDe/iced-project-demo

cargo run
```

## Usage

* Click on **Refill** to add max water capacity 
* Click on **Water** to grow the plant, using up one water each time
* Click on **Reset** to reset the plant growth 




