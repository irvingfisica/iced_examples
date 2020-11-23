use iced::button;
use iced::{Button, Column, Text, Sandbox, Settings, Element, Align};

#[derive(Default)]
struct Counter {
    value: i32,
    increment_button: button::State,
    decrement_button: button::State,
}

impl Sandbox for Counter {

    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        String::from("Counter - Iced")
    }

    fn view(&mut self) -> Element<Message> {
        Column::new()
            .padding(20)
            .align_items(Align::Center)
            .push(
                Button::new(&mut self.increment_button, Text::new("Increment"))
                .on_press(Message::IncrementPressed)
            )
            .push(
                Text::new(&self.value.to_string()).size(50),
            )
            .push(
                Button::new(&mut self.decrement_button, Text::new("Decrement"))
                .on_press(Message::DecrementPressed),
            ).into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::IncrementPressed => {
                self.value += 1;
            },
            Message::DecrementPressed => {
                self.value -= 1;
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Message {
    IncrementPressed,
    DecrementPressed,
}

fn main() {
    Counter::run(Settings::default())
}
