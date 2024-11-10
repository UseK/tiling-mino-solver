fn main() {
    println!("Hello, world!");
}

struct Board {
    inner: Vec<Vec<bool>>,
}

impl Board {
    fn new(width: usize, height: usize) -> Self {
        Board {
            inner: vec![vec![false; width]; height],
        }
    }
}

struct Mino {
    inner: Vec<Vec<bool>>,
}

impl Mino {
    fn from_str(s: &str) -> Self {
        Self {
            inner: s
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
        mino.inner,
        vec![vec![true, true, true], vec![false, true, true]]
    )
}
