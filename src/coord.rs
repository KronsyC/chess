#[derive(Debug)]
pub struct Coord{
    pub row : u8,
    pub col : u8
}


impl Coord{
    pub const fn from_idx(idx : u8) -> Coord{
        Coord{
            col : idx % 8,
            row : idx / 8
        }
    }
}
