use serde::Deserialize;
use std::{collections::HashSet, fs::File};

fn main() {
    let minos: Vec<Mino> = serde_json::from_reader(File::open("data/minos.json").unwrap()).unwrap();
    let board: Board = serde_json::from_reader(File::open("data/board.json").unwrap()).unwrap();
    let tiled = board.tile(&minos);
    println!("{}", tiled.unwrap().pretty_shape());
}

#[derive(Deserialize, Clone, Debug)]
struct Board {
    shape: Vec<Vec<bool>>,
    mino_transforms: Vec<(Mino, TransForm)>,
}

impl Board {
    fn height(&self) -> usize {
        self.shape.len()
    }
    fn width(&self) -> usize {
        self.shape[0].len()
    }
    fn tile(&self, minos: &[Mino]) -> Option<Self> {
        if minos.is_empty() {
            return Some(self.clone());
        }
        let head_mino = minos[0].clone();
        let ts = self.search_can_put(&head_mino);
        if ts.is_empty() {
            None
        } else {
            for t in ts {
                let mut new_board = self.clone();
                new_board.put_mino(head_mino.clone(), t);
                let tiled = new_board.tile(&minos[1..]);
                if tiled.is_some() {
                    return tiled;
                }
            }
            None
        }
    }
    fn search_can_put(&self, mino: &Mino) -> Vec<TransForm> {
        let mut transforms = vec![];
        for r in [
            Rotation::Neutral,
            Rotation::Left,
            Rotation::Right,
            Rotation::OneEighty,
        ] {
            transforms.extend(self.search_can_put_rotated(&mino.rotated(&r), &r));
        }
        transforms
    }
    fn search_can_put_rotated(&self, rotated_mino: &Mino, rotation: &Rotation) -> Vec<TransForm> {
        let mut transforms = vec![];
        for y in 0..=self.height() - rotated_mino.height() {
            for x in 0..=self.width() - rotated_mino.width() {
                let t = TransForm {
                    x,
                    y,
                    rotation: rotation.clone(),
                };
                if self.can_put(rotated_mino, &t) {
                    transforms.push(t);
                }
            }
        }
        transforms
    }
    fn can_put(&self, mino: &Mino, transform: &TransForm) -> bool {
        mino.shape.iter().enumerate().all(|(mino_y, line)| {
            let y = transform.y + mino_y;
            line.iter().enumerate().all(|(mino_x, &b)| {
                let x = transform.x + mino_x;
                // println!("b: {}", b);
                // println!(
                //     "x: {}, y: {}, board[{}][{}]: {}",
                //     x, y, y, x, self.shape[y][x]
                // );
                !(self.shape[y][x] && b)
            })
        })
    }

    fn put_mino(&mut self, mino: Mino, transform: TransForm) {
        mino.rotated(&transform.rotation)
            .shape
            .iter()
            .enumerate()
            .for_each(|(mino_y, line)| {
                let y = transform.y + mino_y;
                line.iter().enumerate().for_each(|(mino_x, &b)| {
                    let x = transform.x + mino_x;
                    self.shape[y][x] |= b;
                });
            });
        self.mino_transforms.push((mino, transform));
    }
    fn pretty_shape(&self) -> String {
        let mut char_matrix = vec![vec!['.'; self.shape[0].len()]; self.shape.len()];
        self.shape.iter().enumerate().for_each(|(y, bs)| {
            bs.iter().enumerate().for_each(|(x, &b)| {
                if b {
                    char_matrix[y][x] = '#'
                }
            });
        });
        for (mino, transform) in &self.mino_transforms {
            mino.rotated(&transform.rotation)
                .shape
                .iter()
                .enumerate()
                .for_each(|(mino_y, line)| {
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
            .map(|cs| cs.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Deserialize, Clone, Debug)]
struct TransForm {
    x: usize,
    y: usize,
    rotation: Rotation,
}

#[derive(Deserialize, Clone, Debug)]
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
        let mut cs: HashSet<char> = s.trim().chars().collect();
        cs.remove(&'.');
        cs.remove(&'\n');
        println!("{:?}", cs);
        assert_eq!(cs.len(), 1);
        let name = cs.into_iter().collect::<Vec<char>>()[0];
        Self {
            name,
            shape: s
                .lines()
                .filter(|l| !l.is_empty())
                .map(|l| l.chars().map(|c| c != '.').collect())
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

#[test]
fn test_put_mino() {
    let mut board: Board = serde_json::from_reader(File::open("data/board.json").unwrap()).unwrap();
    let mino = Mino::from_str(
        "a.
aa
aa
aa",
    );
    let t = TransForm {
        x: 1,
        y: 1,
        rotation: Rotation::Neutral,
    };
    assert!(board.can_put(&mino, &t));
    let expected = "####...
#a.....
.aa....
.aa...#
.aa...#
......#
#...###";
    board.put_mino(mino, t);
    assert_eq!(board.pretty_shape(), expected);
}

#[test]
fn test_put_rotated_mino() {
    let mut board: Board = serde_json::from_reader(File::open("data/board.json").unwrap()).unwrap();
    let mino = Mino::from_str(
        "a.
aa
aa
aa",
    );
    let t = TransForm {
        x: 1,
        y: 1,
        rotation: Rotation::Right,
    };
    assert!(board.can_put(&mino, &t));
    let expected = "####...
#aaaa..
.aaa...
......#
......#
......#
#...###";
    board.put_mino(mino, t);
    assert_eq!(board.pretty_shape(), expected);
}
