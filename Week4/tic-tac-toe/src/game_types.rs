use players::Players;
use marker::Marker;

pub fn create_players() -> Vec<Players> {
    human_vs_computer()
}


fn human_vs_computer() -> Vec<Players> {
    let x = Players::Human { marker: Marker::X };
    let o = Players::Computer { marker: Marker::O };
    vec![x, o]
}
