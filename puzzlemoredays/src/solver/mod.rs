use crate::puzzle::config::Target;
use crate::puzzle_state::{Cell, PuzzleState};
use crate::state::SolverStatus;
use puzzle_solver::board::Board;
use puzzle_solver::tile::Tile;
use std::sync::atomic::{AtomicU64, Ordering};
use std::thread;

#[derive(Debug, Clone)]
pub struct SolverCallId(u64);

pub type OnCompleteCallback = Box<dyn FnOnce(SolverStatus) + Send>;

const SOLVER_CALL_ID_ATOMIC_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn create_solver_call_id() -> SolverCallId {
    SolverCallId(SOLVER_CALL_ID_ATOMIC_COUNTER.fetch_add(1, Ordering::SeqCst))
}

pub fn solve_for_target(
    solver_call_id: &SolverCallId,
    puzzle_state: &PuzzleState,
    target: &Target,
    on_complete: OnCompleteCallback,
) {
    let board = create_board(puzzle_state, target);
    let tiles: Vec<Tile> = puzzle_state
        .unused_tiles
        .iter()
        .map(|tile_state| Tile::new(tile_state.base.clone()))
        .collect();
    thread::spawn(move || {
        let result = puzzle_solver::solve_all_filling(board, &tiles);
        match result {
            Ok(_) => {
                on_complete(SolverStatus::Done { solvable: true });
            }
            Err(_) => {
                on_complete(SolverStatus::Done { solvable: false });
            }
        }
    });
}

pub fn interrupt_solver_call(call_id: &SolverCallId) {
    dbg!("Interrupting solver call: {:?}", call_id);
}

fn create_board(puzzle_state: &PuzzleState, target: &Target) -> Board {
    let dims = puzzle_state.grid.dim();
    let mut board = Board::new(dims);

    puzzle_state.grid.indexed_iter().for_each(|((x, y), cell)| {
        let is_filled = match cell {
            Cell::Empty(cell_data) => !cell_data.is_on_board,
            Cell::One(_, _) => true,
            Cell::Many(_, _) => true,
        };

        board[[x, y]] = is_filled;
    });

    for index in target.indices.iter() {
        let x = index.0 + 1;
        let y = index.1 + 1;
        board[[x, y]] = true;
    }

    board
}
