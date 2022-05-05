use std::{fmt, io};
use rand;

#[derive(Clone, Copy, Debug)]
enum CellTypes {
    Blank,
    Cross,
    Nought
}

#[derive(Clone, Copy)]
struct Cell {
    variant: CellTypes
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.variant {
            CellTypes::Blank => " ",
            CellTypes::Cross => "X",
            CellTypes::Nought => "O"
        })
    }
}

impl Cell {
    fn new() -> Cell {
        Cell { variant: CellTypes::Blank }
    }

    fn change(&mut self, new: CellTypes) {
        self.variant = new
    }
}

struct Board {
    cells: [[Cell; 3]; 3]
}

impl Board {
    fn new() -> Board {
        Board {
            cells: [[Cell::new(); 3]; 3]
        }
    }

    fn draw(&self) {
        let s = &self.cells;
        println!("{}|{}|{}", s[0][0], s[0][1], s[0][2]);
        println!("-+-+-");
        println!("{}|{}|{}", s[1][0], s[1][1], s[1][2]);
        println!("-+-+-");
        println!("{}|{}|{}", s[1][0], s[2][1], s[2][2]);
    }

    fn get(&mut self, point: Point) -> Result<&mut Cell, &str> {
        if point.y > self.cells.len() || point.x > self.cells[0].len() {
            return Err("Co-ordinate out of bounds!")
        }

        println!("({}, {})", point.x, point.y);
        Ok(&mut self.cells[point.y][point.x])
    }
}

struct Player {
    variant: CellTypes,
}

impl Player {
    fn new() -> Player {
        match rand::random() {
            true => Player { variant: CellTypes::Nought },
            false => Player { variant: CellTypes::Cross },
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
        write!(f, "{}", match self.variant {
            CellTypes::Cross => "Cross",
            CellTypes::Nought => "Nought",
            CellTypes::Blank => panic!("For some reason player.variant is blank! No idea why!"),
        })
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(vector: Vec<usize>) -> Point {
        println!("{}, {}", vector[0], vector[1]);
        Point { x: vector[0] - 1, y: vector[1] - 1 }
    }
}

fn main() {
    let mut board = Board::new();
    let mut player = Player::new();

    loop {
        board.draw();
        println!("{}'s turn!", player);
        println!("Enter your input in the form x, y where the top-left is 1,  and the bottom right is 3, 3:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Reading from input failed :(");

        let coords = Point::new(input.split(",") // Parsing input into a point struct
                                    .map(|x| x.trim().parse().expect("Enter a number!"))
                                    .collect());

        match board.get(coords) {
            Ok(cell) => cell.change(player.variant),
            Err(msg) => {
                println!("{}", msg);
                break;
            }
        };

        player.switch();
    }
}
