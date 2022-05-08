use rand;
use std::{
    fmt, io,
    ops::{Add, AddAssign},
};

const EDGE: u32 = 3;

#[derive(Clone, Copy, Debug, PartialEq)]
enum CellTypes {
    Blank,
    Cross,
    Nought,
}

#[derive(Clone, Copy)]
struct Cell {
    variant: CellTypes,
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.variant {
                CellTypes::Blank => " ",
                CellTypes::Cross => "X",
                CellTypes::Nought => "O",
            }
        )
    }
}

impl Cell {
    fn new() -> Cell {
        Cell {
            variant: CellTypes::Blank,
        }
    }

    fn change(&mut self, new: CellTypes) {
        self.variant = new
    }
}

struct Pointer {
    pos: Point,
}

impl Pointer {
    fn new() -> Pointer {
        Pointer {
            pos: Point::new(1, 1),
        }
    }

    fn change_pos(&mut self, by: Point) -> Result<(), &str> {
        if !coords_in_bounds(self.pos + by) {
            return Err("Ended up out of bounds whilst moving the pointer!");
        }

        self.pos += by;
        Ok(())
    }

    fn change_pos_coords(&mut self, x: i32, y: i32) -> Result<(), &str> {
        self.change_pos(Point::new(x as isize, y as isize))
    }

    fn set_pos(&mut self, to: Point) -> Result<(), &str> {
        if !coords_in_bounds(to) {
            return Err("Ended up out of bounds whilst moving the pointer!");
        }

        self.pos = to;
        Ok(())
    }
}

struct Board {
    cells: [[Cell; (EDGE as usize)]; (EDGE as usize)],
    pointer: Pointer,
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[Cell::new(); (EDGE as usize)]; (EDGE as usize)],
            pointer: Pointer::new(),
        }
    }

    fn draw(&self) {
        println!();
        let s = &self.cells;
        println!("{}|{}|{}", s[0][0], s[0][1], s[0][2]);
        println!("-+-+-");
        println!("{}|{}|{}", s[1][0], s[1][1], s[1][2]);
        println!("-+-+-");
        println!("{}|{}|{}", s[1][0], s[2][1], s[2][2]);
        println!();
    }

    fn get(&mut self, point: Point) -> Result<&mut Cell, &str> {
        if !coords_in_bounds(point) {
            return Err("Co-ordinate out of bounds!");
        }

        match self.cells[point.y as usize][point.x as usize].variant {
            CellTypes::Blank => Ok(&mut self.cells[point.y as usize][point.x as usize]),
            _ => Err("Cell is already filled!"),
        }
    }

    fn type_at(&self, point: Point) -> Result<Cell, &str> {
        if !coords_in_bounds(point) {
            return Err("Co-ordinate out of bounds!");
        }

        // println!("Co-ords: {}, {}", point.x, point.y);
        // println!("In bounds: {}", coords_in_bounds(point));
        Ok(self.cells[point.y as usize][point.x as usize])
    }

    fn verify(&mut self) -> Option<CellTypes> {
        for (x, row) in self.cells.iter().enumerate() {
            for (y, _) in row.iter().enumerate() {
                self.pointer
                    .set_pos(Point::new(x as isize, y as isize))
                    .expect("Position out of bounds! HOW!!!");
                let checking = self
                    .type_at(self.pointer.pos)
                    .expect("Position out of bounds! HOW!!!")
                    .variant;

                if checking == CellTypes::Blank {
                    continue;
                }

                'directions: for direction in [
                    (-1, -1),
                    (-1, 0),
                    (-1, 1),
                    (0, -1),
                    (0, 1),
                    (1, -1),
                    (1, 0),
                    (1, 1),
                ] {
                    let mut count = 1;
                    loop {
                        match self.pointer.change_pos_coords(direction.0, direction.1) {
                            Ok(()) => {
                                if let Ok(cell) = self.type_at(self.pointer.pos) {
                                    if cell.variant == checking {
                                        count += 1;
                                        if count >= EDGE {
                                            return Some(checking);
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                self.pointer
                                    .set_pos(Point::new(x as isize, y as isize))
                                    .expect("Position out of bounds! HOW!!!");
                                continue 'directions;
                            }
                        };
                    }
                }
            }
        }

        None
    }
}

struct Player {
    variant: CellTypes,
}

impl Player {
    fn new() -> Player {
        match rand::random() {
            true => Player {
                variant: CellTypes::Nought,
            },
            false => Player {
                variant: CellTypes::Cross,
            },
        }
    }

    fn switch(&mut self) {
        self.variant = match self.variant {
            CellTypes::Nought => CellTypes::Cross,
            CellTypes::Cross => CellTypes::Nought,
            CellTypes::Blank => panic!("For some reason player.variant is blank! No idea why!"),
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.variant {
                CellTypes::Cross => "Cross",
                CellTypes::Nought => "Nought",
                CellTypes::Blank => panic!("For some reason player.variant is blank! No idea why!"),
            }
        )
    }
}

#[derive(Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

impl Point {
    fn from_vec(vector: Vec<isize>) -> Point {
        Point {
            x: vector[0] - 1,
            y: vector[1] - 1,
        }
    }

    fn new(x: isize, y: isize) -> Point {
        Point { x, y }
    }
}

impl AddAssign<Point> for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

fn coords_in_bounds(point: Point) -> bool {
    point.y < EDGE as isize && point.x < EDGE as isize && point.y >= 0 && point.x >= 0
}

fn main() {
    let mut board = Board::new();
    let mut player = Player::new();

    'gameloop: loop {
        board.draw();
        println!("{}'s turn!", player);
        println!("Enter your input in the form x, y where the top-left is 1, 1 and the bottom right is 3, 3:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Reading from input failed :(");

        let coords = Point::from_vec(
            input
                .split(",") // Parsing input into a point struct
                .map(|x| x.trim().parse().expect("Enter a number!"))
                .collect(),
        );

        match board.get(coords) {
            Ok(cell) => cell.change(player.variant),
            Err(msg) => {
                println!("{}", msg);
                continue;
            }
        };

        match board.verify() {
            Some(victor) => {
                board.draw();
                println!("{:?} has won!", victor);
                break 'gameloop;
            }
            None => (),
        };
        player.switch();
    }
}
