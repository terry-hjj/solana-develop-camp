use lines;
use marker;
use marker::Marker;
use board::Board;

pub fn find_current_player(board: &Board) -> Marker {
    // TODO: 根据棋盘当前的棋子，判断下个棋子，是X还是O

    // 已经下过子的格子数量 = 格子总数 - 空格子的数量
    let invalid_space_num = board.get_size() * board.get_size() - board.get_available_spaces().len() as i32;
    // 如果已下子的数量为偶数,即下一步应由玩家来下,则是X; 否则,就是由对手方下,即O
    if invalid_space_num % 2 == 0 {
        Marker::X
    } else {
        Marker::O
    }
}

pub fn is_game_over(board: &Board) -> bool {
    is_game_tied(board) || is_game_won(board)
}

pub fn is_game_tied(board: &Board) -> bool {
    !is_game_won(board) && board.get_available_spaces().is_empty()
}

fn is_game_won(board: &Board) -> bool {
    is_game_won_by(board, &Marker::X) || is_game_won_by(board, &Marker::O)
}

pub fn is_game_won_by(board: &Board, player: &Marker) -> bool {
    let winning_scenarios = lines::find_all_lines(board);
    winning_scenarios
        .iter()
        .any(|line| is_line_won_by(line, player))
}

fn is_line_won_by(line: &[String], player: &Marker) -> bool {
    line.iter().all(|space| space == &marker::inspect(player))
}

pub fn find_winner(board: &Board) -> Marker {
    // TODO: 调用is_game_won_by 判断是 X赢了还是O赢了
    if is_game_won_by(board, &Marker::X) {
        Marker::X
    } else if is_game_won_by(board, &Marker::O) {
        Marker::O
    } else {
        Marker::NA // 双方都没赢, 则平局
    }
}

pub mod tests {
    #[cfg(test)]
    use super::*;
    #[cfg(test)]
    use board::tests::set_up_board;
    #[test]
    fn x_is_current_player_at_start_of_game() {
        let board: Board = set_up_board(3, vec![]);
        assert_eq!(Marker::X, find_current_player(&board));
    }

    #[test]
    fn o_is_current_player_after_one_move() {
        let board: Board = set_up_board(3, vec![0]);
        assert_eq!(Marker::O, find_current_player(&board));
    }

    #[test]
    fn game_not_over_when_board_is_empty() {
        let board: Board = set_up_board(3, vec![]);
        assert!(!is_game_over(&board));
    }

    #[test]
    fn game_is_over_when_board_is_full() {
        let board: Board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert!(is_game_over(&board));
    }

    #[test]
    fn check_if_tied_game_is_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert!(!is_game_won(&board));
    }

    #[test]
    fn check_if_game_won_by_x_is_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 3, 7]);
        assert!(is_game_won(&board));
    }

    #[test]
    fn check_if_game_won_by_o_is_won() {
        let board = set_up_board(3, vec![0, 8, 4, 7, 2, 6]);
        assert!(is_game_won(&board));
    }

    #[test]
    fn an_empty_game_is_not_won() {
        let board = set_up_board(3, vec![]);
        assert!(!is_game_won(&board));
    }

    #[test]
    fn an_empty_game_is_not_tied() {
        let board = set_up_board(3, vec![]);
        assert!(!is_game_tied(&board));
    }

    #[test]
    fn a_won_game_is_not_tied() {
        let board = set_up_board(3, vec![0, 8, 4, 7, 2, 6]);
        assert!(!is_game_tied(&board));
    }

    #[test]
    fn a_won_game_with_a_full_board_is_not_tied() {
        let board = set_up_board(3, vec![0, 3, 1, 4, 6, 7, 5, 8, 2]);
        assert!(!is_game_tied(&board));
    }

    #[test]
    fn a_tied_game_is_tied() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert!(is_game_tied(&board));
    }

    #[test]
    fn check_line_won_by_x() {
        let line: Vec<String> = vec!["X".to_string(), "X".to_string(), "X".to_string()];
        assert!(is_line_won_by(&line, &Marker::X));
    }

    #[test]
    fn check_row_not_won_by_o() {
        let line: Vec<String> = vec!["O".to_string(), " ".to_string(), "X".to_string()];
        assert!(!is_line_won_by(&line, &Marker::O));
    }

    #[test]
    fn find_winner_when_nobody_has_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 7, 1, 3, 5]);
        assert_eq!(Marker::NA, find_winner(&board));
    }

    #[test]
    fn find_winner_when_x_has_won() {
        let board = set_up_board(3, vec![0, 4, 8, 2, 6, 3, 7]);
        assert_eq!(Marker::X, find_winner(&board));
    }

    #[test]
    fn find_winner_when_o_has_won() {
        let board = set_up_board(3, vec![0, 8, 4, 7, 2, 6]);
        assert_eq!(Marker::O, find_winner(&board));
    }
}
