use rayon::ThreadPoolBuilder;
use std::env;
use tiling_mino_solver::{Board, Mino, check_wall_count};

pub const NUM_THREADS: usize = 8;

fn main() {
    ThreadPoolBuilder::new()
        .num_threads(NUM_THREADS)
        .build_global()
        .unwrap();
    let args: Vec<String> = env::args().collect();
    let (minos_path, board_path) = if args.len() == 3 {
        (args[1].clone(), args[2].clone())
    } else {
        ("data/minos".to_string(), "data/board.txt".to_string())
    };
    let _ = solve(minos_path, board_path);
}

pub fn solve(minos_path: String, board_path: String) -> Result<(), String> {
    let mut minos: Vec<Mino> = Mino::minos_from_path(minos_path)?;
    minos.sort_by_key(|m| m.count_wall());
    minos.reverse();
    let board = Board::from_text_path(board_path)?;
    check_wall_count(&minos, &board);
    let tiled = board.tile_parallel(&minos);
    if let Some(board) = tiled {
        board.pretty_print();
    } else {
        println!("Can NOT resolved");
    }
    Ok(())
}
