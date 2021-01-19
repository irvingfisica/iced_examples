use iced::{Application,
            executor,
            Command,
            Element,
            Canvas,
            Length,
            canvas,
            Color,
            Rectangle,
            Settings,
        };

pub fn main() {
    Lienzo::run(Settings {
        antialiasing: true,
        ..Settings::default()
    }).unwrap();
}

pub struct Lienzo {
    circulo: Circulo,
    circle: canvas::Cache,
}

#[derive(Debug,Clone,Copy)]
pub enum Message {
}

impl Application for Lienzo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Lienzo {
                circulo: Circulo {radius: 50.0},
                circle: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Círculo simple")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {
        Command::none()
    }

    fn view(&mut self) -> Element<Message> {
        Canvas::new(self)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()

    }
}

#[derive(Debug)]
struct Circulo {
    radius: f32,
}

impl<Message> canvas::Program<Message> for Lienzo {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {

        let circle = self.circle.draw(bounds.size(), |frame| {
            let cir = canvas::Path::circle(frame.center(), self.circulo.radius);

            frame.fill(&cir, Color::from_rgb8(0xF9, 0xD7, 0x1C));
        });

        vec![circle]

    }
} 
