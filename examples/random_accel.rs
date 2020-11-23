use rand::prelude::*;

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
            window,
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

        let (width, height) = window::Settings::default().size;

        let pos_ini = Point::new((width as f32)/2.0,(height as f32)/2.0);
        let vec_ini = Vector::new(0.0,0.0);
        let acc_ini = Vector::new(0.0,0.0);

        (
            Lienzo {
                circulo: Circulo::new(pos_ini,vec_ini,acc_ini),
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

        let (width, height) = window::Settings::default().size;
        
        let canvas = Canvas::new()
            .width(Length::Units(width as u16))
            .height(Length::Units(height as u16))
            .push(self.circle.with(&self.circulo));

        Container::new(canvas)
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
    accel: Vector,
}

impl Circulo {

    pub fn new(posicion: Point, velocidad: Vector, acelera: Vector) -> Circulo {

        Circulo {
            center: posicion,
            velocity: velocidad,
            accel: acelera,
            radius: 10.0,
        }
    }

    pub fn update(&mut self) {

        let mut rng = thread_rng();

        self.check_edges();

        let acc_x: f32 = rng.gen_range(-1.0,1.0);
        let temp_y: f32 = rng.gen_range(-1.0,1.0);
        let acc_y: f32 = (1.0 - acc_x.powi(2)).sqrt() * temp_y.signum();

        self.accel = Vector::new(acc_x,acc_y);
        self.velocity = self.velocity + self.accel;
        self.center = self.center + self.velocity;
    }

    pub fn check_edges(&mut self) {

        let (width, height) = window::Settings::default().size;

        if self.center.x > width as f32 || self.center.x < 0.0 {
            self.velocity.x = self.velocity.x * -1.0;
        }

        if self.center.y > height as f32 || self.center.y < 0.0 {
            self.velocity.y = self.velocity.y * -1.0;
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