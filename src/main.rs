use std::collections::{HashSet,HashMap};
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
    Subscription,
    time,
    mouse,
    };
use iced::canvas::{
    self,
    Cache,
    Canvas,
    Cursor,
    Geometry,
    Path,
    Frame,
};
use iced::canvas::event::{self, Event};


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

#[derive(Debug)]
enum Message {
    Tick,
    Populate(Cell),
    Unpopulate(Cell),
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

    fn update(&mut self, message: Message) -> Command<Message> {

        match message {
            Message::Tick => {
                self.grid.update();
            },
            Message::Populate(cell) => {
                self.grid.populate(cell);
            },
            Message::Unpopulate(cell) => {
                self.grid.unpopulate(&cell);
            }
        }

        Command::none()
    }

    fn subscription(&self) -> Subscription<Message>{
        time::every(std::time::Duration::from_millis(50))
            .map(|_instant| {
                Message::Tick
             } )
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

enum Interaction {
    None,
    Panning {translation: Vector, start: Point},
    Drawing,
    Erasing,
}

pub struct Region {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

struct Grid {
    life_cache: Cache,
    life: Life,
    interaction: Interaction,
    translation: Vector,
    scaling: f32,
}

impl canvas::Program<Message> for Grid {

    fn update(&mut self, event: Event, bounds: Rectangle, cursor: Cursor) -> (event::Status, Option<Message>) {

        if let Event::Mouse(mouse::Event::ButtonReleased(_)) = event {
            self.interaction = Interaction::None;
        }

        let cursor_position = if let Some(position) = cursor.position_in(&bounds) {
            position 
        } else {
            return (event::Status::Ignored, None)
        };

        let cell = Cell::at(self.project(cursor_position, bounds.size()));
        let is_populated = self.life.contains(&cell);

        let (populate, unpopulate) = if is_populated {
            (None, Some(Message::Unpopulate(cell)))
        } else {
            (Some(Message::Populate(cell)), None)
        };

        match event {
            Event::Mouse(mouse_event) => match mouse_event {
                mouse::Event::ButtonPressed(button) => {
                    let message = match button {
                        mouse::Button::Left => {
                            self.interaction = if is_populated {
                                Interaction::Erasing
                            } else {
                                Interaction::Drawing
                            };

                            populate.or(unpopulate)
                        },
                        mouse::Button::Right => {
                            self.interaction = Interaction::Panning {
                                translation: self.translation,
                                start: cursor_position,
                            };

                            None
                        },
                        _ => None
                    };

                    (event::Status::Captured, message)
                },
                mouse::Event::CursorMoved {..} => {
                    let message = match self.interaction {
                        Interaction::Drawing => populate,
                        Interaction::Erasing => unpopulate,
                        Interaction::Panning {translation, start} => {
                            self.translation = translation
                                + (cursor_position - start)
                                * (1.0 / self.scaling);
    
                            self.life_cache.clear();
    
                            None
                        },
                        _ => None,
                    };
    
                    let event_status = match self.interaction {
                        Interaction::None => event::Status::Ignored,
                        _ => event::Status::Captured,
                    };
    
                    (event_status, message)
                },
                mouse::Event::WheelScrolled { delta } => match delta {
                    mouse::ScrollDelta::Lines { y, .. } |
                    mouse::ScrollDelta::Pixels { y, .. } => {
                        if y < 0.0 && self.scaling > Self::MIN_SCALING
                            || y > 0.0 && self.scaling < Self::MAX_SCALING
                        {
                            let old_scaling = self.scaling;

                            self.scaling = (self.scaling
                                *(1.0 + y / 30.0))
                                .max(Self::MIN_SCALING)
                                .min(Self::MAX_SCALING);

                            if let Some(cursor_to_center) = 
                                cursor.position_from(bounds.center())
                            {
                                let factor = self.scaling - old_scaling;

                                self.translation = self.translation
                                    - Vector::new(
                                        cursor_to_center.x * factor / (old_scaling * old_scaling),
                                        cursor_to_center.y * factor / (old_scaling * old_scaling),
                                    );
                            }

                            self.life_cache.clear();
                        }

                        (event::Status::Captured, None)
                    }
                }
                _ => (event::Status::Ignored, None)
            },
            _ => (event::Status::Ignored, None)
        }
    }

    fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Vec<Geometry> {

        let center = Vector::new(bounds.width / 2.0, bounds.height / 2.0);

        let grid = self.life_cache.draw(bounds.size(), |frame| {
            let background = Path::rectangle(Point::ORIGIN, frame.size());

            frame.fill(&background, Color::BLACK);

            frame.with_save(|frame| {

                frame.translate(center);
                frame.scale(self.scaling);
                frame.translate(self.translation);
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

        let overlay = {
            let mut frame = Frame::new(bounds.size());

            let hovered_cell = cursor.position_in(&bounds).map(|position| {
                Cell::at(self.project(position, frame.size()))
            });

            if let Some(cell) = hovered_cell {
                frame.with_save(|frame|{
                    frame.translate(center);
                    frame.scale(self.scaling);
                    frame.translate(self.translation);
                    frame.scale(Cell::SIZE as f32);

                    frame.fill_rectangle(
                        Point::new(cell.j as f32, cell.i as f32),
                        Size::UNIT,
                        Color {
                            a: 0.5,
                            ..Color::WHITE
                        },
                    );
                });
            }

            frame.into_geometry()
        };

        vec![grid, overlay]

    }
}

impl Grid {
    const MIN_SCALING: f32 = 0.1;
    const MAX_SCALING: f32 = 2.0;

    pub fn from_preset(preset: Preset) -> Self {
        Self {
            life: preset.life()
                    .into_iter()
                    .map(|(i,j)| Cell { i, j })
                    .collect(),
            life_cache: Cache::default(),
            interaction: Interaction::None,
            translation: Vector::default(),
            scaling: 1.0,
        }
    }

    pub fn update(&mut self) {
        self.life.tick();
        self.life_cache.clear();
    }

    fn visible_region(&self, size: Size) -> Region {
        let width = size.width / self.scaling;
        let height = size.height / self.scaling;

        Region {
            x: -self.translation.x - width / 2.0,
            y: -self.translation.y - height / 2.0,
            width,
            height,
        }
    }

    fn project(&self, position: Point, size: Size) -> Point {
        let region = self.visible_region(size);

        Point::new(
            position.x / self.scaling + region.x,
            position.y / self.scaling + region.y,
        )
    }

    fn populate(&mut self, cell: Cell) {
        self.life.populate(cell);
        self.life_cache.clear();
    }

    fn unpopulate(&mut self, cell: &Cell) {
        self.life.unpopulate(cell);
        self.life_cache.clear();
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

impl Life {
    fn tick(&mut self) {
        let mut adjacent_life: HashMap<Cell,usize> = HashMap::default();

        for cell in &self.cells {
            adjacent_life.entry(*cell).or_insert(0);

            for neighbour in Cell::neighbors(*cell) {
                let amount = adjacent_life.entry(neighbour).or_insert(0);

                *amount += 1;
            }
        }

        for (cell, amount) in adjacent_life.iter() {
            match amount {
                2 => {},
                3 => {
                    self.cells.insert(*cell);
                },
                _ => {
                    self.cells.remove(cell);
                }
            }
        }
    }

    fn contains(&self, cell: &Cell) -> bool {
        self.cells.contains(cell)
    }

    fn populate(&mut self, cell: Cell) {
        self.cells.insert(cell);
    }

    fn unpopulate(&mut self, cell: &Cell) {
        self.cells.remove(cell);
    }
}

impl std::iter::FromIterator<Cell> for Life {
    fn from_iter<I: IntoIterator<Item = Cell>>(iter: I) -> Self {
        Life {
            cells: iter.into_iter().collect(),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
struct Cell {
    i: isize,
    j: isize,
}

impl Cell {
    const SIZE: usize = 10;

    fn cluster(cell: Cell) -> impl Iterator<Item = Cell> {
        use itertools::Itertools;

        let rows = cell.i.saturating_sub(1) ..= cell.i.saturating_add(1);
        let columns = cell.j.saturating_sub(1) ..= cell.j.saturating_add(1);

        rows.cartesian_product(columns).map(|(i,j)| Cell {i, j})
    }

    fn neighbors(cell: Cell) -> impl Iterator<Item = Cell> {
        Cell::cluster(cell).filter(move |candidate| *candidate != cell)
    }

    fn at(position: Point) -> Cell {
        let i = (position.y / Cell::SIZE as f32).ceil() as isize;
        let j = (position.x / Cell::SIZE as f32).ceil() as isize;

        Cell {
            i: i.saturating_sub(1),
            j: j.saturating_sub(1),
        }
    }
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

