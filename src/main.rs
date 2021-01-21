use std::collections::HashSet;
use iced::{
    Application, 
    executor, 
    Command, 
    Element, 
    Container,
    Length,
    Column,
    Settings,
    Rectangle,
    Point,
    Color,
    Size,
    Vector,
    };
use iced::canvas::{
    self,
    Cache,
    Canvas,
    Cursor,
    Geometry,
    Path,
};

pub fn main() -> iced::Result {

    GameOfLife::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })

}

#[derive(Default)]
struct GameOfLife {
    grid: Grid,
}

#[derive(Debug, Clone)]
enum Message {

}

impl Application for GameOfLife {
    type Message = Message;
    type Executor = executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Self, Command<Message>) {
        (
            Self {
                ..Self::default()
            },
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("Game of Life - Iced")
    }

    fn update(&mut self, _message: Message) -> Command<Message> {

        Command::none()
    }

    fn view(&mut self) -> Element<Message> {

        let canvas: Element<Message> = Canvas::new(&mut self.grid)
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .into();

        let content = Column::new().push(canvas);

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct Grid {
    life_cache: Cache,
    life: Life,
}

impl<Message> canvas::Program<Message> for Grid {

    fn draw(&self, bounds: Rectangle, _cursor: Cursor) -> Vec<Geometry> {

        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let grid = self.life_cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());

            frame.fill(&background, Color::BLACK);

            frame.with_save(|frame| {

                frame.translate(center);
                frame.scale(Cell::SIZE as f32);

                for cell in &self.life.cells {

                    frame.fill_rectangle(
                        Point::new(cell.j as f32, cell.i as f32),
                        Size::UNIT,
                        Color::WHITE,
                    )

                }

            });

        });

        vec![grid]

    }
}

impl Grid {
    pub fn from_preset(preset: Preset) -> Self {
        Self {
            life: preset.life()
                    .into_iter()
                    .map(|(i,j)| Cell { i, j })
                    .collect(),
            life_cache: Cache::default(),
        }
    }
}

impl Default for Grid {
    fn default() -> Self {
        Self::from_preset(Preset::GliderGun)
    }
}

#[derive(Default)]
struct Life {
    cells: HashSet<Cell>
}

impl std::iter::FromIterator<Cell> for Life {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        Life {
            cells: iter.into_iter().collect(),
        }
    }
}

// #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[derive(PartialEq, Eq, Hash)]
struct Cell {
    i: isize,
    j: isize,
}

impl Cell {
    const SIZE: usize = 10;
}

enum Preset {
    Glider,
    GliderGun,
}

impl Preset {
    pub fn life(self) -> Vec<(isize, isize)> {

        #[rustfmt::skip]
        let cells = match self {
            Preset::Glider => vec![
                " x ",
                "  x",
                "xxx"
            ],
            Preset::GliderGun => vec![
                "                        x           ",
                "                      x x           ",
                "            xx      xx            xx",
                "           x   x    xx            xx",
                "xx        x     x   xx              ",
                "xx        x   x xx    x x           ",
                "          x     x       x           ",
                "           x   x                    ",
                "            xx                      ",
            ],
        };

        let start_row = -(cells.len() as isize / 2);

        cells
            .into_iter()
            .enumerate()
            .flat_map(|(i, cells)| {
                let start_column = -(cells.len() as isize / 2);

                cells
                    .chars()
                    .enumerate()
                    .filter(|(_, c)| !c.is_whitespace())
                    .map(move |(j, _)| {
                        (start_row + i as isize, start_column + j as isize)
                    })
            }).collect()
    }
}

impl Default for Preset {
    fn default() -> Preset {
        Preset::Glider
    }
}

