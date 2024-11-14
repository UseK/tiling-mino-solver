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
    Right,
    OneEighty,
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
struct Mino {
    name: char,
    shape: Vec<Vec<bool>>,
}

impl Mino {
    fn height(&self) -> usize {
        self.shape.len()
    }
    fn width(&self) -> usize {
        self.shape[0].len()
    }
    fn rotated(&self, rotation: &Rotation) -> Self {
        let new_shape = match rotation {
            Rotation::Neutral => self.shape.clone(),
            Rotation::Left => {
                let mut right_shape = vec![vec![false; self.height()]; self.width()];
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        right_shape[self.width() - x - 1][y] = self.shape[y][x];
                    }
                }
                right_shape
            }
            Rotation::Right => {
                let mut left_shape = vec![vec![false; self.height()]; self.width()];
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        left_shape[x][self.height() - y - 1] = self.shape[y][x];
                    }
                }
                left_shape
            }
            Rotation::OneEighty => {
                let mut one_eighty_shape = vec![vec![false; self.width()]; self.height()];
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        one_eighty_shape[self.height() - y - 1][self.width() - x - 1] =
                            self.shape[y][x];
                    }
                }
                one_eighty_shape
            }
        };
        Self {
            shape: new_shape,
            name: self.name,
        }
    }

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

#[test]
fn test_mino_from_str() {
    let mino = Mino::from_str("###\n.##");
    assert_eq!(
        mino.shape,
        vec![vec![true, true, true], vec![false, true, true]]
    );
    assert_eq!(mino.height(), 2);
    assert_eq!(mino.width(), 3);
}

#[test]
fn test_mino_rotated_left() {
    // ###
    // ..#
    let mino = Mino::from_str("###\n..#");
    // ##
    // #.
    // #.
    assert_eq!(mino.rotated(&Rotation::Left), Mino::from_str("##\n#.\n#."));
}

#[test]
fn test_mino_rotated_right() {
    // ###
    // ..#
    let mino = Mino::from_str("###\n..#");
    // .#
    // .#
    // ##
    assert_eq!(mino.rotated(&Rotation::Right), Mino::from_str(".#\n.#\n##"));
}

#[test]
fn test_mino_rotated_one_eighty() {
    // ###
    // ..#
    let mino = Mino::from_str("###\n..#");
    // #..
    // ###
    assert_eq!(
        mino.rotated(&Rotation::OneEighty),
        Mino::from_str("#..\n###")
    );
}
