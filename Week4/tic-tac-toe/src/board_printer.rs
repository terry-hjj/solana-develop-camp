use board_formatter;
use lines;
use board::Board;

const OFFSET: usize = 1;

pub fn format_board(board: &Board) -> String {
    let split_board = lines::split_board_into_rows(
        &number_spaces(&board_formatter::expand_board(board)),
        board.get_size().abs(),
    );
    let mut formatted_board: String = "".to_string();
    for (index, row) in split_board.iter().enumerate() {
        let formatted_row = format_row(&row.to_vec());
        let length = formatted_row.len();
        formatted_board += &formatted_row;
        if index < row.len() - OFFSET {
            formatted_board += &"-".repeat(length - OFFSET);
            formatted_board += "\n";
        }
    }
    formatted_board
}

fn format_row(row: &[String]) -> String {
    let mut formatted_row: String = "".to_string();
    for (index, mark) in row.iter().enumerate() {
        formatted_row.push_str(" ");
        formatted_row.push_str(mark);
        if mark.len() == OFFSET {
            formatted_row.push_str(" ");
        }
        formatted_row.push_str(" ");
        if index < row.len() - OFFSET {
            formatted_row.push_str("|");
        } else {
            formatted_row.push_str("\n");
        }
    }
    formatted_row
}

fn number_spaces(spaces: &[String]) -> Vec<String> {
    let mut updated_spaces: Vec<String> = vec![" ".to_string(); spaces.len() as usize];
    for (index, space) in spaces.iter().enumerate() {
        if space == " " {
            let number = index + OFFSET;
            updated_spaces[index] = number.to_string();
        } else {
            updated_spaces[index] = space.to_string();
        }
    }
    updated_spaces
}