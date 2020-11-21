mod circle {

use iced_native::{
    Widget, Length, layout, Layout, Size, Hasher, Point, MouseCursor,
    Color, Background, Element
};
use iced_wgpu::{
    Renderer, Defaults, Primitive
};

pub struct Circle {
    radius: u16,
}

impl Circle {
    pub fn new(radius: u16) -> Self {
        Self { radius}
    }
}

impl<Message> Widget<Message, Renderer> for Circle {

    fn width(&self) -> Length {
        Length::Shrink
    }

    fn height(&self) -> Length {
        Length::Shrink
    }

    fn layout(
        &self,
        _renderer: &Renderer,
        _limits: &layout::Limits,
    ) -> layout::Node {
        layout::Node::new(Size::new(
            f32::from(self.radius) * 2.0,
            f32::from(self.radius) * 2.0,
        ))
    }

    fn hash_layout(&self, state: &mut Hasher) {
        use std::hash::Hash;

        self.radius.hash(state);
    }

    fn draw(
        &self,
        _renderer: &mut Renderer,
        _defaults: &Defaults,
        layout: Layout<'_>,
        _cursor_position: Point,
    ) -> (Primitive, MouseCursor) {
        (
            Primitive::Quad {
                bounds: layout.bounds(),
                background: Background::Color(Color::BLACK),
                border_radius: self.radius,
                border_width: 0,
                border_color: Color::TRANSPARENT,
            },
            MouseCursor::OutOfBounds,
        )
    }

}

impl<'a, Message> Into<Element<'a, Message, Renderer>> for Circle {
    fn into(self) -> Element<'a, Message, Renderer> {
        Element::new(self)
    }
}

}

use circle::Circle;
use iced::{
    slider, Sandbox, Column, Align, Slider, Container, Element,
    Length, Settings, Text
};


fn main() {
    Example::run(Settings::default())
}

struct Example {
    radius: u16,
    slider: slider::State
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example {
            radius: 50,
            slider: slider::State::new()
        }
    }

    fn title(&self) -> String {
        String::from("Círculo")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
                self.radius = radius.round() as u16;
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let content = Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Align::Center)
            .push(Circle::new(self.radius))
            .push(Text::new(format!("Radius: {}", self.radius.to_string())))
            .push(Slider::new(
                &mut self.slider,
                1.0..=100.0,
                f32::from(self.radius),
                Message::RadiusChanged,
            ));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()

    }
}
