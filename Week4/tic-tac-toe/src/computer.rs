use game;
use std::collections::HashMap;
use board::Board;

const INITIAL_DEPTH: i32 = 0;
const TIED: i32 = 0;
const MAX_SCORE: i32 = 1000;
const INCREMENT: i32 = 1;
const EARLY_STAGES_OF_GAME: usize = 1;
const FIRST_MOVE: i32 = 4;
const SECOND_MOVE: i32 = 0;

pub fn find_space(board: &Board) -> i32 {
    if is_game_in_early_stages(board) {
        choose_strategic_space(board)
    } else {
        find_best_score(board, INITIAL_DEPTH, HashMap::new())
    }
}

fn find_best_score(board: &Board, depth: i32, mut best_score: HashMap<i32, i32>) -> i32 {
    if game::is_game_over(board) {
        score_scenarios(board, depth)
    } else {
        for space in &board.get_available_spaces() {
            let emulated_board = board.clone().place_marker(*space);
            best_score.insert(
                *space,
                -find_best_score(&emulated_board, depth + INCREMENT, HashMap::new()),
            );
        }
        analyse_board(&best_score, depth)
    }
}

fn score_scenarios(board: &Board, depth: i32) -> i32 {
    if game::is_game_tied(board) {
        TIED
    } else {
        -MAX_SCORE / depth
    }
}

fn analyse_board(best_score: &HashMap<i32, i32>, depth: i32) -> i32 {
    let space_with_highest_score: (i32, i32) = find_highest_score(best_score);
    if current_turn(depth) {
        choose_best_space(space_with_highest_score)
    } else {
        choose_highest_score(space_with_highest_score)
    }
}

fn find_highest_score(best_score: &HashMap<i32, i32>) -> (i32, i32) {
    let mut scores_to_compare: Vec<(&i32, &i32)> = best_score.iter().collect();
    scores_to_compare.sort_by(|key, value| value.1.cmp(key.1));
    (*scores_to_compare[0].0, *scores_to_compare[0].1)
}

fn choose_best_space(best_space_with_score: (i32, i32)) -> i32 {
    best_space_with_score.0
}

fn choose_highest_score(best_space_with_score: (i32, i32)) -> i32 {
    best_space_with_score.1
}

fn current_turn(depth: i32) -> bool {
    depth == 0
}

fn is_game_in_early_stages(board: &Board) -> bool {
    board.get_spaces().len() <= EARLY_STAGES_OF_GAME
}

fn choose_strategic_space(board: &Board) -> i32 {
    if board.is_space_available(&FIRST_MOVE) {
        FIRST_MOVE
    } else {
        SECOND_MOVE
    }
}