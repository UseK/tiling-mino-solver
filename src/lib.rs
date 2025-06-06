use ansi_term::Colour as Color;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};
pub mod gui;

#[derive(Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Board {
    pub shape: Shape,
    mino_transforms: Vec<(Mino, TransForm)>,
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub struct TransForm {
    x: usize,
    y: usize,
    rotation: Rotation,
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug)]
pub enum Rotation {
    Neutral,
    Left,
    Right,
    OneEighty,
}

impl Board {
    pub fn new(shape: Shape) -> Self {
        Self {
            shape,
            mino_transforms: vec![],
        }
    }
    pub fn from_text_path<P>(path: P) -> Result<Self, String>
    where
        P: AsRef<Path>,
    {
        let mut buf = "".to_string();
        File::open(path).unwrap().read_to_string(&mut buf).unwrap();
        Ok(Self::new(Shape::from_str(&buf)?))
    }
    pub fn height(&self) -> usize {
        self.shape.height()
    }
    pub fn width(&self) -> usize {
        self.shape.width()
    }
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.shape.is_wall(x, y)
    }
    pub fn tile_parallel(&self, minos: &[Mino]) -> Option<Self> {
        if minos.len() > 8 {
            self.pretty_print();
            println!("{}", "-".repeat(self.width()));
        }
        if minos.is_empty() {
            return Some(self.clone());
        }
        let head_mino = minos[0].clone();
        let ts = self.search_can_put(&head_mino);
        if ts.is_empty() {
            None
        } else {
            ts.into_par_iter().find_map_any(|t| {
                let mut new_board: Board = self.clone();
                new_board.put_mino(head_mino.clone(), t);
                new_board.tile_parallel(&minos[1..])
            })
        }
    }
    #[allow(dead_code)]
    fn tile_serial(&self, minos: &[Mino]) -> Option<Self> {
        if minos.len() > 8 {
            self.pretty_print();
            println!("{}", "-".repeat(self.width()));
        }
        if minos.is_empty() {
            return Some(self.clone());
        }
        let head_mino = minos[0].clone();
        let ts = self.search_can_put(&head_mino);
        if ts.is_empty() {
            None
        } else {
            ts.into_iter().find_map(|t| {
                let mut new_board: Board = self.clone();
                new_board.put_mino(head_mino.clone(), t);
                new_board.tile_serial(&minos[1..])
            })
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
    pub fn can_put(&self, mino: &Mino, transform: &TransForm) -> bool {
        mino.shape
            .coordinates()
            .into_iter()
            .all(|(mino_x, mino_y, b)| {
                let x = transform.x + mino_x;
                let y = transform.y + mino_y;
                !(self.is_wall(x, y) && b)
            })
    }

    pub fn put_mino(&mut self, mino: Mino, transform: TransForm) {
        mino.rotated(&transform.rotation)
            .shape
            .coordinates()
            .into_iter()
            .for_each(|(mino_x, mino_y, mino_b)| {
                let x = transform.x + mino_x;
                let y = transform.y + mino_y;
                self.shape.put_on(x, y, mino_b);
            });
        self.mino_transforms.push((mino, transform));
    }
    const MINO_COLORS: [Color; 6] = [
        Color::Blue,
        Color::Cyan,
        Color::Green,
        Color::Purple,
        Color::Red,
        Color::Yellow,
    ];
    pub fn pretty_print(&self) {
        let mino_chars: HashMap<char, Color> = self
            .mino_transforms
            .iter()
            .enumerate()
            .map(|(ind, t)| {
                (
                    t.0.name,
                    if ind < 6 {
                        Self::MINO_COLORS[ind]
                    } else {
                        Color::White
                    },
                )
            })
            .collect();
        let mut n_vacant = 0;
        for c in self.pretty_shape().chars() {
            if c == '#' {
                print!("{}", Color::Black.on(Color::White).paint(c.to_string()));
            } else if c == '.' {
                n_vacant += 1;
                print!(".");
            } else if c == '\n' {
                println!("{}", n_vacant);
            } else {
                let color = mino_chars.get(&c).unwrap_or(&Color::White);
                print!("{}", color.paint(c.to_string()));
            }
        }
        println!("{}", n_vacant);
    }
    pub fn pretty_shape(&self) -> String {
        let mut char_matrix = vec![vec!['.'; self.width()]; self.height()];
        self.shape.coordinates().into_iter().for_each(|(x, y, b)| {
            if b {
                char_matrix[y][x] = '#'
            }
        });
        for (mino, transform) in &self.mino_transforms {
            mino.rotated(&transform.rotation)
                .shape
                .coordinates()
                .into_iter()
                .for_each(|(mino_x, mino_y, mino_b)| {
                    let x = transform.x + mino_x;
                    let y = transform.y + mino_y;
                    if mino_b {
                        char_matrix[y][x] = mino.name;
                    }
                });
        }
        char_matrix
            .iter()
            .map(|cs| cs.iter().collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    }
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Mino {
    pub name: char,
    pub shape: Shape,
}

impl Mino {
    pub fn new(name: char, shape: Shape) -> Self {
        Self { name, shape }
    }
    pub fn minos_from_path<P>(path: P) -> Result<Vec<Self>, String>
    where
        P: AsRef<Path>,
    {
        if path.as_ref().is_file() {
            Ok(Self::minos_from_text_path(path)?)
        } else if path.as_ref().is_dir() {
            Ok(Self::minos_from_directory_path(path))
        } else {
            Err(format!("Invalid path {:?}", path.as_ref()))
        }
    }
    pub fn pretty_print(&self) {
        println!("------------");
        self.shape.0.iter().for_each(|bools| {
            let line = bools
                .iter()
                .map(|&b| if b { self.name } else { '.' })
                .collect::<String>();
            println!("{}", line)
        });
        println!("------------");
    }
    pub fn count_wall(&self) -> usize {
        self.shape.count_wall()
    }
    fn height(&self) -> usize {
        self.shape.height()
    }
    fn width(&self) -> usize {
        self.shape.width()
    }
    fn rotated(&self, rotation: &Rotation) -> Self {
        let new_raw_shape = match rotation {
            Rotation::Neutral => self.shape.clone(),
            Rotation::Left => {
                let mut right_shape = vec![vec![false; self.height()]; self.width()];
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        right_shape[self.width() - x - 1][y] = self.shape.is_wall(x, y);
                    }
                }
                Shape(right_shape)
            }
            Rotation::Right => {
                let mut left_shape = vec![vec![false; self.height()]; self.width()];
                for y in 0..self.height() {
                    (0..self.width()).for_each(|x| {
                        left_shape[x][self.height() - y - 1] = self.shape.is_wall(x, y);
                    });
                }
                Shape(left_shape)
            }
            Rotation::OneEighty => {
                let mut one_eighty_shape = vec![vec![false; self.width()]; self.height()];
                for y in 0..self.height() {
                    for x in 0..self.width() {
                        one_eighty_shape[self.height() - y - 1][self.width() - x - 1] =
                            self.shape.is_wall(x, y);
                    }
                }
                Shape(one_eighty_shape)
            }
        };
        Self {
            shape: new_raw_shape,
            name: self.name,
        }
    }
    fn minos_from_directory_path<P>(directory_path: P) -> Vec<Self>
    where
        P: AsRef<Path>,
    {
        directory_path
            .as_ref()
            .read_dir()
            .unwrap()
            .flat_map(|entry| Self::minos_from_text_path(entry.unwrap().path()).unwrap())
            .collect()
    }
    fn minos_from_text_path<P>(p: P) -> Result<Vec<Self>, String>
    where
        P: AsRef<Path>,
    {
        let mut buf = "".to_string();
        File::open(p).unwrap().read_to_string(&mut buf).unwrap();
        let lines: Vec<String> = buf.lines().map(|s| s.to_string()).collect();
        lines
            .split(|line| line.contains('-'))
            .flat_map(|block| {
                let count = block[0].parse::<usize>().unwrap();
                let s: String = block[1..].join("\n");
                vec![Mino::from_str(&s); count]
            })
            .collect()
    }
}

impl FromStr for Mino {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut cs: HashSet<char> = s.trim().chars().collect();
        cs.remove(&'.');
        cs.remove(&'\n');
        if cs.len() != 1 {
            println!("{}", s);
        }
        assert_eq!(cs.len(), 1);
        let name = cs.into_iter().collect::<Vec<char>>()[0];
        Ok(Self {
            name,
            shape: Shape::from_str(s)?,
        })
    }
}

pub fn check_wall_count(minos: &Vec<Mino>, board: &Board) {
    let count_mino_walls = minos
        .iter()
        .map(|mino| mino.shape.count_wall())
        .reduce(|a, b| a + b)
        .unwrap_or_default();
    let mut count = 0;
    if count_mino_walls != board.shape.count_vacant() {
        for m in minos {
            m.pretty_print();
            count += m.shape.count_wall();
            println!("count wall: {}", count);
        }
        board.pretty_print();
        println!("count vacant: {}", board.shape.count_vacant());
    }
    assert_eq!(
        count_mino_walls,
        board.shape.count_vacant(),
        "the number of walls is different"
    );
}

#[test]
fn test_mino_from_str() {
    let mino = Mino::from_str("###\n.##").unwrap();
    assert_eq!(
        mino.shape.0,
        vec![vec![true, true, true], vec![false, true, true]]
    );
    assert_eq!(mino.height(), 2);
    assert_eq!(mino.width(), 3);
}

#[test]
fn test_mino_rotated_left() {
    // ###
    // ..#
    let mino = Mino::from_str("###\n..#").unwrap();
    // ##
    // #.
    // #.
    assert_eq!(
        mino.rotated(&Rotation::Left),
        Mino::from_str("##\n#.\n#.").unwrap()
    );
}

#[test]
fn test_mino_rotated_right() {
    // ###
    // ..#
    let mino = Mino::from_str("###\n..#").unwrap();
    // .#
    // .#
    // ##
    assert_eq!(
        mino.rotated(&Rotation::Right),
        Mino::from_str(".#\n.#\n##").unwrap()
    );
}

#[test]
fn test_mino_rotated_one_eighty() {
    // ###
    // ..#
    let mino = Mino::from_str("###\n..#").unwrap();
    // #..
    // ###
    assert_eq!(
        mino.rotated(&Rotation::OneEighty),
        Mino::from_str("#..\n###").unwrap()
    );
}

#[test]
fn test_put_mino() {
    let mut board: Board =
        serde_json::from_reader(File::open("testdata/board.json").unwrap()).unwrap();
    let mino = Mino::from_str(
        "a.
aa
aa
aa",
    )
    .unwrap();
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
    let mut board: Board =
        serde_json::from_reader(File::open("testdata/board.json").unwrap()).unwrap();
    let mino = Mino::from_str(
        "a.
aa
aa
aa",
    )
    .unwrap();
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

#[test]
fn test_board_from_text_path() {
    let board = Board::from_text_path("testdata/board.txt").unwrap();
    let expected: Board =
        serde_json::from_reader(File::open("testdata/board.json").unwrap()).unwrap();
    assert_eq!(board, expected);
}

#[test]
fn test_minos_from_text_path() {
    let minos = Mino::minos_from_text_path("testdata/minos.txt").unwrap();
    for m in &minos {
        {
            let this = &m;
            println!("------------");
            this.shape.0.iter().for_each(|bools| {
                let line = bools
                    .iter()
                    .map(|&b| if b { this.name } else { '.' })
                    .collect::<String>();
                println!("{}", line)
            });
            println!("------------");
        };
    }
    let expected: Vec<Mino> =
        serde_json::from_reader(File::open("data/minos.json").unwrap()).unwrap();
    assert_eq!(minos, expected);
}

#[derive(Deserialize, Clone, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct Shape(Vec<Vec<bool>>);

impl Shape {
    pub fn new(vec: Vec<Vec<bool>>) -> Self {
        Self(vec)
    }
    pub fn width(&self) -> usize {
        self.0[0].len()
    }
    pub fn height(&self) -> usize {
        self.0.len()
    }
    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        self.0[y][x]
    }
    pub fn count_wall(&self) -> usize {
        let mut count = 0;
        for y in 0..self.height() {
            for x in 0..self.width() {
                if self.is_wall(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
    pub fn count_vacant(&self) -> usize {
        self.width() * self.height() - self.count_wall()
    }
    pub fn put_on(&mut self, x: usize, y: usize, b: bool) {
        self.0[y][x] |= b;
    }
    pub fn toggle(&mut self, x: usize, y: usize) {
        self.0[y][x] = !self.0[y][x];
    }
    pub fn coordinates(&self) -> Vec<(usize, usize, bool)> {
        let mut vs = vec![];
        for y in 0..self.height() {
            for x in 0..self.width() {
                vs.push((x, y, self.is_wall(x, y)))
            }
        }
        vs
    }
}

impl FromStr for Shape {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.lines()
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().map(|c| c != '.').collect::<Vec<bool>>())
                .collect(),
        ))
    }
}

#[test]
fn test_minos_from_path() {
    use std::collections::BTreeSet;
    fn assert_eq_as_set<T>(a: &[T], b: &[T])
    where
        T: std::cmp::Ord,
        T: std::fmt::Debug,
    {
        let a_set = BTreeSet::from_iter(a.iter());
        let b_set = BTreeSet::from_iter(b.iter());
        assert_eq!(a_set, b_set);
    }
    let minos = Mino::minos_from_path("data/minos").unwrap();
    for m in &minos {
        {
            let this = &m;
            println!("------------");
            this.shape.0.iter().for_each(|bools| {
                let line = bools
                    .iter()
                    .map(|&b| if b { this.name } else { '.' })
                    .collect::<String>();
                println!("{}", line)
            });
            println!("------------");
        };
    }
    let expected: Vec<Mino> =
        serde_json::from_reader(File::open("data/minos.json").unwrap()).unwrap();
    assert_eq_as_set(&minos, &expected);
}

#[test]
fn bench_tile_parallel() {
    let board = Board::from_text_path("data/bench/board.txt").unwrap();
    let minos: Vec<Mino> = Mino::minos_from_path("data/bench/minos.txt").unwrap();
    assert!(board.tile_parallel(&minos).is_some());
}

#[test]
fn bench_tile_serial() {
    let board = Board::from_text_path("data/bench/board.txt").unwrap();
    let minos: Vec<Mino> = Mino::minos_from_path("data/bench/minos.txt").unwrap();
    assert!(board.tile_serial(&minos).is_some());
}

#[test]
fn test_shape_toggle() {
    let mut shape = Shape::from_str("##\n.#").unwrap();
    assert_eq!(shape.is_wall(0, 0), true);
    assert_eq!(shape.is_wall(1, 0), true);
    assert_eq!(shape.is_wall(0, 1), false);
    assert_eq!(shape.is_wall(1, 1), true);

    shape.toggle(0, 1);
    assert_eq!(shape.is_wall(0, 0), true);
    assert_eq!(shape.is_wall(1, 0), true);
    assert_eq!(shape.is_wall(0, 1), true);
    assert_eq!(shape.is_wall(1, 1), true);
    shape.toggle(0, 0);
    assert_eq!(shape.is_wall(0, 0), false);
    assert_eq!(shape.is_wall(1, 0), true);
    assert_eq!(shape.is_wall(0, 1), true);
    assert_eq!(shape.is_wall(1, 1), true);
}

#[test]
fn test_shape_new() {
    let shape = Shape::new(vec![vec![true, true], vec![false, true]]);
    let expected = Shape::from_str("##\n.#").unwrap();
    assert_eq!(shape, expected);
}
