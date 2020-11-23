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
            Point,
            Vector,
            Subscription,
        };

use std::time::Instant;

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
    Tick(Instant)
}

impl Application for Lienzo {
    type Executor = executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Lienzo {
                circulo: Circulo::new(50.0,50.0),
                circle: Default::default(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Círculo simple")
    }

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::Tick(_instant) => {
                self.circulo.center = self.circulo.center + Vector::new(2.0,2.0);
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
    center: Point,
}

impl Circulo {

    pub fn new(x: f32, y: f32) -> Circulo {

        Circulo {
            center: Point::new(x,y),
            radius: 10.0,
        }
    }

}

impl canvas::Drawable for Circulo {
    fn draw(&self, frame: &mut canvas::Frame) {
        use canvas::Path;

        let center = self.center;
        let radius = self.radius;

        let circ = Path::circle(center, radius);
        frame.fill(&circ, Color::from_rgb8(0x12, 0x93, 0xD8));

    }
}

mod time {
    use iced::futures;
    use std::time::Instant;

    pub fn every(duration: std::time::Duration) -> iced::Subscription<Instant> {
        iced::Subscription::from_recipe(Every(duration))
    }

    struct Every(std::time::Duration);

    impl<H, I> iced_native::subscription::Recipe<H,I> for Every
    where
        H: std::hash::Hasher,
    {
        type Output = Instant;

        fn hash(&self, state: &mut H) {
            use std::hash::Hash;

            std::any::TypeId::of::<Self>().hash(state);
            self.0.hash(state);
        }

        fn stream(
            self: Box<Self>,
            _input: futures::stream::BoxStream<'static, I>,
        ) -> futures::stream::BoxStream<'static, Self::Output> {
            use futures::stream::StreamExt;

            async_std::stream::interval(self.0)
                .map(|_| Instant::now())
                .boxed()
        }
    }
}