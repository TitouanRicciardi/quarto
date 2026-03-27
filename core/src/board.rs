use crate::Piece;

const EMPTY_SLOT: Piece = Piece(0b11110000);

/// The standard 4x4 Quarto board.
#[derive(Clone)]
pub struct Board(pub [[Piece; 4]; 4]);

impl Board {
    pub fn new() -> Self {
        Board([[EMPTY_SLOT; 4]; 4])
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Option<Piece> {
        if x >= 4 || y >= 4 {
            return None;
        }

        match self.0[x][y] {
            EMPTY_SLOT => None,
            p => Some(p),
        }
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Option<Piece>) {
        if x >= 4 || y >= 4 {
            return;
        }

        self.0[x][y] = match piece {
            Some(p) => p,
            None => EMPTY_SLOT,
        };
    }

    /// Checks for a win condition given starting coordinates (sx, sy) and direction (mx, my).
    fn is_win_axis(&self, sx: usize, sy: usize, mx: isize, my: isize) -> bool {
        let mut x: isize = sx as isize;
        let mut y: isize = sy as isize;

        let mut current = match self.get_piece(x as usize, y as usize) {
            Some(p) => p,
            None => return false,
        };

        let mut flags: u8 = 0x0F;

        for _ in 0..3 {
            x += mx;
            y += my;
            let next = match self.get_piece(x as usize, y as usize) {
                Some(p) => p,
                None => return false,
            };

            flags &= !(current.0 ^ next.0);
            if (flags & 0x0F) == 0 {
                return false;
            }

            current = next;
        }

        true
    }

    pub fn is_win(&self, x: usize, y: usize) -> bool {
        if self.is_win_axis(0, y, 1, 0) {
            return true;
        }

        if self.is_win_axis(x, 0, 0, 1) {
            return true;
        }

        if x == y && self.is_win_axis(0, 0, 1, 1) {
            return true;
        }

        if x + y == 3 && self.is_win_axis(3, 0, -1, 1) {
            return true;
        }

        false
    }

    pub fn is_win_global(&self) -> bool {
        for i in 0..4 {
            if self.is_win_axis(0, i, 1, 0) {
                return true;
            }

            if self.is_win_axis(i, 0, 0, 1) {
                return true;
            }
        }

        if self.is_win_axis(0, 0, 1, 1) {
            return true;
        }

        if self.is_win_axis(3, 0, -1, 1) {
            return true;
        }

        false
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new()
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for x in 0..4 {
            for y in 0..4 {
                match self.get_piece(x, y) {
                    Some(p) => f.write_fmt(format_args!("{p} "))?,
                    None => f.write_str("    ")?,
                }
            }
            f.write_str("\n")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_piece() {
        let board = Board::new();

        assert_eq!(None, board.get_piece(1, 1));
        assert_eq!(None, board.get_piece(5, 1));
    }

    #[test]
    fn test_set_piece() {
        let mut board = Board::new();
        let p0 = Piece::new(true, true, false, false);
        let p1 = Piece::new(false, true, true, true);

        assert_eq!(None, board.get_piece(1, 3));
        assert_eq!(None, board.get_piece(2, 0));
        assert_eq!(None, board.get_piece(0, 1));

        board.set_piece(1, 3, Some(p0));
        board.set_piece(2, 0, Some(p1));
        assert_eq!(Some(p0), board.get_piece(1, 3));
        assert_eq!(Some(p1), board.get_piece(2, 0));
        assert_eq!(None, board.get_piece(0, 1));

        board.set_piece(1, 3, None);
        assert_eq!(None, board.get_piece(1, 3));
        assert_eq!(Some(p1), board.get_piece(2, 0));
        assert_eq!(None, board.get_piece(0, 1));
    }

    #[test]
    fn test_win_axis() {
        let mut board = Board::new();

        assert_eq!(false, board.is_win_axis(0, 0, 1, 1));
        assert_eq!(false, board.is_win_axis(3, 0, -1, 1));
        assert_eq!(false, board.is_win_axis(0, 0, 1, 0));

        board.set_piece(0, 0, Some(Piece(0b0101)));
        board.set_piece(1, 0, Some(Piece(0b1101)));
        board.set_piece(2, 0, Some(Piece(0b1001)));
        board.set_piece(3, 0, Some(Piece(0b1111)));
        assert_eq!(false, board.is_win_axis(0, 0, 1, 1));
        assert_eq!(false, board.is_win_axis(3, 0, -1, 1));
        assert_eq!(true, board.is_win_axis(0, 0, 1, 0));

        board.set_piece(2, 0, Some(Piece(0b1000)));
        assert_eq!(false, board.is_win_axis(0, 0, 1, 0));

        board.set_piece(3, 0, Some(Piece(0b0110)));
        board.set_piece(2, 1, Some(Piece(0b0011)));
        board.set_piece(1, 2, Some(Piece(0b0101)));
        board.set_piece(0, 3, Some(Piece(0b0011)));
        assert_eq!(false, board.is_win_axis(0, 0, 1, 1));
        assert_eq!(true, board.is_win_axis(3, 0, -1, 1));
        assert_eq!(false, board.is_win_axis(0, 0, 1, 0));

        board.set_piece(2, 1, Some(Piece(0b0111)));
        board.set_piece(1, 2, Some(Piece(0b1101)));
        board.set_piece(0, 3, Some(Piece(0b0111)));
        assert_eq!(true, board.is_win_axis(3, 0, -1, 1));

        board.set_piece(0, 3, Some(Piece(0b0000)));
        assert_eq!(false, board.is_win_axis(3, 0, -1, 1));

        board.set_piece(0, 0, Some(Piece(0b0000)));
        board.set_piece(1, 1, Some(Piece(0b0001)));
        board.set_piece(2, 2, Some(Piece(0b0010)));
        board.set_piece(3, 3, Some(Piece(0b0011)));
        assert_eq!(true, board.is_win_axis(0, 0, 1, 1));
    }

    #[test]
    fn test_win() {
        let mut board = Board::new();

        board.set_piece(3, 0, Some(Piece(0b0110)));
        board.set_piece(3, 1, Some(Piece(0b0100)));
        board.set_piece(3, 2, Some(Piece(0b1100)));
        board.set_piece(3, 3, Some(Piece(0b1000)));
        assert_eq!(true, board.is_win(3, 0));
        assert_eq!(true, board.is_win(3, 1));
        assert_eq!(true, board.is_win(3, 2));
        assert_eq!(true, board.is_win(3, 3));
    }
}
