use rand;
use std::{
    fmt, io,
    ops::{Add, AddAssign},
};

const EXHUASTIVE_CHECK_MSG: &str = "Exhaustive checking! If you see this something has gone seriously, seriously wrong.";
const EDGE: u32 = 3;

#[derive(Clone, Copy, PartialEq)]
enum CellTypes {
    Blank,
    Cross,
    Nought,
}

impl fmt::Display for CellTypes {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                CellTypes::Cross => "Cross",
                CellTypes::Nought => "Nought",
                _ => panic!("{}", EXHUASTIVE_CHECK_MSG),
            }
        )
    }
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
            return Err("Co-ordinates given to Pointer::set_pos are out of bounds!");
        }

        self.pos = to;
        Ok(())
    }
}

struct Board {
    cells: [[Cell; (EDGE as usize)]; (EDGE as usize)], // 2 Dimesnional array of size EDGE x EDGE
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
        let s = &self.cells; // So you don't have to write %self.cell[y][x]
        println!("{}|{}|{}", s[0][0], s[0][1], s[0][2]);
        println!("-+-+-");
        println!("{}|{}|{}", s[1][0], s[1][1], s[1][2]);
        println!("-+-+-");
        println!("{}|{}|{}", s[2][0], s[2][1], s[2][2]);
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

    fn type_at(&self, point: Point) -> Result<CellTypes, &str> {
        if !coords_in_bounds(point) {
            return Err("Co-ordinate out of bounds!");
        }

        Ok(self.cells[point.y as usize][point.x as usize].variant)
    }

    fn verify(&mut self) -> Option<CellTypes> {
        for (x, row) in self.cells.iter().enumerate() {
            for y in 0..row.len() {
                // Loop through every y co-ordinate
                self.pointer
                    .set_pos(Point::new(x as isize, y as isize))
                    .unwrap(); // Unwrap because this should never be out of bounds

                let checking = self.type_at(self.pointer.pos).unwrap(); // Ditto
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
                            // Go forever in each cardinal direction
                            Ok(()) => {
                                // If the pointer did not end up out of bounds
                                if let Ok(variant) = self.type_at(self.pointer.pos) {
                                    if variant == checking {
                                        // If this is the variant we started from
                                        count += 1; // Increase the count. if count is now > EDGE we know it must be a full row
                                        if count >= EDGE {
                                            return Some(checking);
                                        }
                                    }
                                }
                            }
                            Err(_) => {
                                self.pointer
                                    .set_pos(Point::new(x as isize, y as isize))
                                    .unwrap(); // Reset the pointer to the starting co-ordinate and move on to a new direction
                                continue 'directions;
                            }
                        };
                    }
                }
            }
        }

        None // If there were no full rows
    }
}

struct Player {
    variant: CellTypes,
}

impl Player {
    fn new() -> Player {
        Player {
            variant: match rand::random() {
                true => CellTypes::Nought,
                false => CellTypes::Cross,
            },
        }
    }

    fn switch(&mut self) {
        self.variant = match self.variant {
            CellTypes::Nought => CellTypes::Cross,
            CellTypes::Cross => CellTypes::Nought,
            _ => panic!("{}", EXHUASTIVE_CHECK_MSG),
        }
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

    loop {
        board.draw();
        println!("{}'s turn!", player.variant);
        println!("Enter your input in the form 'x, y' where the top-left is 1, 1 and the bottom right is 3, 3:");

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Reading from input failed :(");

        // Parsing input into a point struct
        let coords = Point::from_vec(
            input
                .split(",")
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

        if let Some(victor) = board.verify() {
            board.draw();
            println!("{} has won!", victor);
            break;
        }

        player.switch();
    }
}
