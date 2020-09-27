use rand::{thread_rng, Rng};
use std::fmt;
use std::{thread, time};
use lazy_static::lazy_static;

const LIFE: f64 = 0.7;
const SPEED: u64 = 200;

lazy_static!{
    static ref HEIGHT: usize = 500; //term_size::dimensions().unwrap().1;
    static ref WIDTH: usize = 500;//term_size::dimensions().unwrap().0;
}

struct Board {
    rows: Vec<Vec<bool>>,
}

impl Board {
    fn new() -> Board {
        let mut rng = rand::thread_rng();

        let mut b = Board::default();
        for row in b.rows.iter_mut() {
            for cell in row.iter_mut() {
                *cell = rng.gen_bool(LIFE);
            }
        }
        b
    }

    fn default() -> Board {
        Board {
            rows: vec![vec![false; *WIDTH]; *HEIGHT],
        }
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for row in self.rows.iter() {
            for cell in row.iter() {
                if *cell == true {
                    write!(f, "*")?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "")?;
        }
        Ok(())
    }
}

fn main() {
    let mut b = Board::new();
    let ten_millis = time::Duration::from_millis(SPEED);
    let now = time::Instant::now();

    loop {
        print!("{}[2J", 27 as char);
        print!("{}", &b);
        b = tick(&b);
        thread::sleep(ten_millis);
    }
}

fn tick(old: &Board) -> Board {
    let mut new = Board::default();
    for h in 0..*HEIGHT - 1 {
        for w in 0..*WIDTH - 1 {
            new.rows[h][w] = is_alive(old, h, w);
        }
    }
    new
}

fn is_alive(b: &Board, h: usize, w: usize) -> bool {
    let mut score = 0;

    if w != 0 {
        if b.rows[h][w - 1] {
            score += 1;
        }

        if h != 0 {
            if b.rows[h - 1][w - 1] {
                score += 1;
            }
        }

        if h != *HEIGHT {
            if b.rows[h + 1][w - 1] {
                score += 1;
            }
        }
    }

    if w != *WIDTH {
        if b.rows[h][w + 1] {
            score += 1;
        }
        if h != 0 {
            if b.rows[h - 1][w + 1] {
                score += 1;
            }
        }

        if h != *HEIGHT {
            if b.rows[h + 1][w + 1] {
                score += 1;
            }
        }
    }

    if h != 0 {
        if b.rows[h - 1][w] {
            score += 1;
        }
    }

    if h != *HEIGHT {
        if b.rows[h + 1][w] {
            score += 1;
        }
    }

    match (b.rows[h][w], score) {
        (true, 2..=3) => true,
        (false, 3) => true,
        _ => false,
    }
}
