#[derive(Clone, Copy, PartialEq)]
pub struct Position(u8);



impl Position{
    pub const fn new(row : u8, col : u8) -> Position{
        assert!(row < 8, "Row in bounds");
        assert!(col < 8, "Col in bounds");
        Position(
            col | row << 3
        )
    }


    pub const fn row(&self) -> u8{
        self.0 >> 3
    }

    pub const fn col(&self) -> u8{
        self.0 & 0x07
    }


    ///
    /// Return the integral representation of this position 
    /// (0..63) where 0 is A1 and 63 is H8
    ///
    pub const fn integral(&self) -> u8{
        self.0
    }


    pub const fn from_integral(integ : u8) -> Position{
        assert!(integ < 64, "Integral less than 64");

        Position(integ)
    }

    pub fn as_alphanum(&self) -> String{
        let col = (b'A' + self.col()) as char;
        let row = (b'1' + self.row()) as char;

        format!("{col}{row}")
    }
}




impl std::fmt::Debug for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row = {}, col = {})", self.row(), self.col())
    }
}
impl std::fmt::Display for Position{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(row = {}, col = {})", self.row(), self.col())
    }
}
