use io;

const OFFSET: usize = 1;

pub fn find_space() -> i32 {
    io::select_number() - OFFSET as i32
}
