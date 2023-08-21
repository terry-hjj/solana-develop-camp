use board_formatter;
use board::Board;

const OFFSET: usize = 1;

pub fn split_board_into_rows(expanded_board: &[String], size: i32) -> Vec<Vec<String>> {
    let chunks = expanded_board.chunks(size as usize);
    let mut rows: Vec<Vec<String>> = Vec::new();
    for chunk in chunks {
        let row: Vec<String> = chunk.to_vec();
        rows.push(row);
    }
    rows
}

pub fn find_columns(rows: &[Vec<String>]) -> Vec<Vec<String>> {
    let mut columns = rows.to_vec();
    for (row_index, row) in rows.iter().enumerate() {
        for (space_index, space) in row.iter().enumerate() {
            columns[space_index][row_index] = space.to_string();
        }
    }
    columns
}

pub fn find_left_diagonal(rows: &[Vec<String>]) -> Vec<String> {
    let mut diagonal: Vec<String> = vec![" ".to_string(); rows.len()];
    for (index, row) in rows.iter().enumerate() {
        diagonal[index] = row[index].to_string();
    }
    diagonal
}

pub fn find_right_diagonal(rows: &[Vec<String>]) -> Vec<String> {
    let mut diagonal: Vec<String> = vec![" ".to_string(); rows.len()];
    for (index, row) in rows.iter().enumerate() {
        diagonal[index] = row[rows.len() - (index + OFFSET)].to_string();
    }
    diagonal
}

pub fn find_all_lines(board: &Board) -> Vec<Vec<String>> {
    let mut winning_scenarios: Vec<Vec<String>> = Vec::new();
    let mut rows = split_board_into_rows(
        &board_formatter::expand_board(board),
        board.get_size().abs(),
    );
    let mut columns = find_columns(&rows);
    let left = find_left_diagonal(&rows);
    let right = find_right_diagonal(&rows);
    winning_scenarios.append(&mut rows);
    winning_scenarios.append(&mut columns);
    winning_scenarios.push(left);
    winning_scenarios.push(right);
    winning_scenarios
}