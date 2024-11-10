use serde::Deserialize;
use std::fs::File;

fn main() {
    println!("Hello, world!");
    let minos: Vec<Mino> = serde_json::from_reader(File::open("data/minos.json").unwrap()).unwrap();
    minos
        .iter()
        .for_each(|mino| println!("mino: {:?}", mino.name));
    let mut board: Board = serde_json::from_reader(File::open("data/board.json").unwrap()).unwrap();
    board.print_shape();
    minos[0].print_shape();
    let t = TransForm {
        x: 1,
        y: 1,
        rotation: Rotation::Neutral,
    };
    println!("{:?}", board.can_put(&minos[0], &t));
    board.put_mino(minos[0].clone(), t);
    board.print_shape();
}

#[derive(Deserialize)]
struct Board {
    shape: Vec<Vec<bool>>,
    mino_transforms: Vec<(Mino, TransForm)>,
}

impl Board {
    fn can_put(&self, mino: &Mino, transform: &TransForm) -> bool {
        mino.shape.iter().enumerate().all(|(mino_y, line)| {
            let y = transform.y + mino_y;
            line.iter().enumerate().all(|(mino_x, &b)| {
                let x = transform.x + mino_x;
                println!("b: {}", b);
                println!(
                    "x: {}, y: {}, board[{}][{}]: {}",
                    x, y, y, x, self.shape[y][x]
                );
                !(self.shape[y][x] && b)
            })
        })
    }

    fn put_mino(&mut self, mino: Mino, transform: TransForm) {
        mino.shape.iter().enumerate().for_each(|(mino_y, line)| {
            let y = transform.y + mino_y;
            line.iter().enumerate().for_each(|(mino_x, &b)| {
                let x = transform.x + mino_x;
                self.shape[y][x] = b;
            });
        });
        self.mino_transforms.push((mino, transform));
    }
    fn print_shape(&self) {
        let mut char_matrix = vec![vec!['.'; self.shape[0].len()]; self.shape.len()];
        println!("------------");
        self.shape.iter().enumerate().for_each(|(y, bs)| {
            bs.iter().enumerate().for_each(|(x, &b)| {
                if b {
                    char_matrix[y][x] = '#'
                }
            });
        });
        for (mino, transform) in &self.mino_transforms {
            mino.shape.iter().enumerate().for_each(|(mino_y, line)| {
                let y = transform.y + mino_y;
                line.iter().enumerate().for_each(|(mino_x, &b)| {
                    let x = transform.x + mino_x;
                    if b {
                        char_matrix[y][x] = mino.name;
                    }
                });
            });
        }
        char_matrix
            .iter()
            .for_each(|cs| println!("{}", cs.iter().collect::<String>()));
        println!("------------");
    }
}

#[derive(Deserialize, Clone)]
struct TransForm {
    x: usize,
    y: usize,
    rotation: Rotation,
}

#[derive(Deserialize, Clone)]
enum Rotation {
    Neutral,
    Left,
    OneEighty,
    Right,
}

#[derive(Deserialize, Clone)]
struct Mino {
    name: char,
    shape: Vec<Vec<bool>>,
}

impl Mino {
    fn print_shape(&self) {
        println!("------------");
        self.shape.iter().for_each(|bools| {
            let line = bools
                .iter()
                .map(|&b| if b { self.name } else { '.' })
                .collect::<String>();
            println!("{}", line)
        });
        println!("------------");
    }
}

impl Mino {
    fn from_str(s: &str) -> Self {
        Self {
            name: '-',
            shape: s
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().map(|c| c == '#').collect())
                .collect(),
        }
    }
}

const A: &str = "
###
.##
";

const B: &str = "11";

#[test]
fn test_mino_from_str() {
    let mino = Mino::from_str(A);
    assert_eq!(
        mino.shape,
        vec![vec![true, true, true], vec![false, true, true]]
    )
}
