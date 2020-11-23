use iced::{Application,
            executor,
            Command,
            Element,
            Canvas,
            Length,
            canvas,
            Color,
            Container,
            Settings,
        };

pub fn main() {
    Lienzo::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

pub struct Lienzo {
    circulo: Circulo,
    circle: canvas::layer::Cache<Circulo>
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
                circle:Default::default()},
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
        let canvas = Canvas::new()
            .width(Length::Units(400))
            .height(Length::Units(400))
            .push(self.circle.with(&self.circulo));

        Container::new(canvas)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .center_x()
            .center_y()
            .into()

    }
}

#[derive(Debug)]
struct Circulo {
    radius: f32,
}

impl canvas::Drawable for Circulo {
    fn draw(&self, frame: &mut canvas::Frame) {
        use canvas::Path;

        let center = frame.center();
        let radius = self.radius;

        let circ = Path::circle(center, radius);
        frame.fill(&circ, Color::from_rgb8(0x12, 0x93, 0xD8));

    }
}