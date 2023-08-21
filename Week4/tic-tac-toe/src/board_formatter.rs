use board::Board;
use marker::Marker;
use marker;

pub fn expand_board(board: &Board) -> Vec<String> {
    let spaces = board.get_spaces();
    let number_of_spaces = board.get_size() * board.get_size();
    let mut expanded_board: Vec<String> = vec![" ".to_string(); number_of_spaces as usize];
    for (index, space) in spaces.iter().enumerate() {
        expanded_board[*space as usize] = marker::inspect(&set_marker(index));
    }
    expanded_board
}

fn set_marker(index: usize) -> Marker {
    if is_even(index) {
        Marker::X
    } else {
        Marker::O
    }
}

fn is_even(index: usize) -> bool {
    index % 2 == 0
}