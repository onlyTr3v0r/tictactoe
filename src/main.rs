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

impl Cell {
    fn new() -> Cell {
        Cell { variant: CellTypes::Blank }
    }

    fn change(&mut self, new: CellTypes) {
        println!("changing {:?}", new);
        self.variant = new
    }
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

struct Row {
    cells: Vec<Cell>
}

impl Row {
    fn new() -> Row {
        let mut row = vec![];

        for _ in 0..3 {
            row.push(Cell::new())
        }

        Row {
            cells: row
        }
    }

    fn draw(&self) {
        let s: &Vec<Cell> = &self.cells;

        println!("{}|{}|{}", s[0], s[1], s[2])
    }
}

struct Board {
    rows: Vec<Row>
}

impl Board {
    fn new() -> Board {
        let mut rows = vec![];

        for _ in 0..3 {
            rows.push(Row::new())
        }

        Board {
            rows
        }
    }

    fn draw(&self) {
        println!();
        self.rows[0].draw();
        println!("-+-+-");
        self.rows[1].draw();
        println!("-+-+-");
        self.rows[2].draw();
        println!();
    }

    fn get(&self, point: Point) -> Cell {
        self.rows[point.y].cells[point.x]
    }
}

enum PlayerTypes {
    Nought,
    Cross
}

impl PlayerTypes {
    fn as_celltype(&self) -> CellTypes {
        match self {
            PlayerTypes::Nought => CellTypes::Nought,
            PlayerTypes::Cross => CellTypes::Cross
        }
    }
}

struct Player {
    variant: PlayerTypes,
}

impl Player {
    fn new() -> Player {
        match rand::random() {
            true => Player { variant: PlayerTypes::Nought },
            false => Player { variant: PlayerTypes::Cross },
        }
    }

    fn switch(&mut self) {
        self.variant = match self.variant {
            PlayerTypes::Nought => PlayerTypes::Cross,
            PlayerTypes::Cross => PlayerTypes::Nought,
        }
    }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self.variant {
            PlayerTypes::Cross => "Cross",
            PlayerTypes::Nought => "Nought"
        })
    }
}

struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(vector: Vec<usize>) -> Point {
        Point { x: vector[0], y: vector[1] }
    }
}

fn main() {
    let board = Board::new();
    let mut player = Player::new();

    loop {
        board.draw();
        println!("{}'s turn!", player);
        println!("Enter your input in the form x, y where the top-left is 0, 0 and the bottom right is 3, 3:");

        let mut input = String::new();
        io::stdin().read_line(&mut input)
                   .expect("Reading from input failed :(");

        let coords = Point::new(input.split(",") // Parsing input into a vector of u32s
                                    .map(|x| x.trim().parse().unwrap())
                                    .collect());

        println!("Turning cell ({}, {}) to {}", coords.x, coords.y, player);
        board.get(coords).change(player.variant.as_celltype());
        player.switch();
    }
}
