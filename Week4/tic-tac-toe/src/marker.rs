#[derive(PartialEq, Debug)]
pub enum Marker {
    X,
    O,
    NA,
}

pub fn inspect(marker: &Marker) -> String {
    match *marker {
        Marker::X => "X".to_string(),
        Marker::O => "O".to_string(),
        Marker::NA => "Nobody".to_string(),
    }
}