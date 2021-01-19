use iced::{Application,
            executor,
            time,
            Command,
            Element,
            Canvas,
            Length,
            canvas,
            Color,
            Settings,
            Point,
            Vector,
            Subscription,
            Rectangle,
            window,
        };

use std::time::Instant;

pub fn main() {
    Lienzo::run(Settings {
        antialiasing: true,
        ..Settings::default()
    }).unwrap();
}

pub struct Lienzo {
    circulo: Circulo,
    circle: canvas::Cache
}

#[derive(Debug,Clone,Copy)]
pub enum Message {
    Tick(Instant)
}

impl Application for Lienzo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {

        let pos_ini = Point::new(50.0,50.0);
        let vec_ini = Vector::new(0.4,0.2);

        (
            Lienzo {
                circulo: Circulo::new(pos_ini,vec_ini),
                circle: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Círculo simple moviendose")
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::Tick(_instant) => {
                self.circulo.update();
                self.circle.clear();
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message> {
        time::every(std::time::Duration::from_millis(1))
            .map(|instant| Message::Tick(instant))
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
    center: Point,
    velocity: Vector,
}

impl Circulo {

    pub fn new(posicion: Point, velocidad: Vector) -> Circulo {

        Circulo {
            center: posicion,
            velocity: velocidad,
            radius: 10.0,
        }
    }

    pub fn update(&mut self) {

        let (width, height) = window::Settings::default().size;

        if self.center.x > width as f32 || self.center.x < 0.0 {
            self.velocity.x = self.velocity.x * -1.0
        }

        if self.center.y > height as f32 || self.center.y < 0.0 {
            self.velocity.y = self.velocity.y * -1.0
        }

        self.center = self.center + self.velocity;
    }

}

impl<Message> canvas::Program<Message> for Lienzo {
    fn draw(&self, bounds: Rectangle, _cursor: canvas::Cursor) -> Vec<canvas::Geometry> {

        let circle = self.circle.draw(bounds.size(), |frame| {
            let cir = canvas::Path::circle(Point::new(self.circulo.center.x, self.circulo.center.y), self.circulo.radius);

            frame.fill(&cir, Color::from_rgb8(0xF9, 0xD7, 0x1C));
        });

        vec![circle]

    }
} 