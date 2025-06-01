use rayon::ThreadPoolBuilder;
use std::env;
use tiling_mino_slover::NUM_THREADS;

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
    tiling_mino_slover::solve(minos_path, board_path);
}
