use std::fmt::Debug;

#[derive(Clone)]
pub struct Board {
    pub filled: u64,
    pub color: u64,
}

static DIRECTIONS: &[(i8, i8)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

impl Board {
    pub fn new() -> Board {
        Board {
            filled: 0b00000000_00000000_00000000_00011000_00011000_00000000_00000000_00000000,
            color: 0b00000000_00000000_00000000_00010000_00001000_00000000_00000000_00000000,
        }
    }

    pub fn set(&mut self, x: i8, y: i8, color: bool) {
        let bit = 1u64 << (y * 8 + x);
        self.filled |= bit;

        if color {
            self.color |= bit;
        } else {
            self.color &= !bit;
        }
    }

    /// This method is used for testing only
    #[allow(dead_code)]
    pub fn clear(&mut self, x: i8, y: i8) {
        let bit = 1u64 << (y * 8 + x);
        self.filled &= !bit;
    }

    /// Tries to place a chip and calculate results of action.
    ///
    /// Colors: true = white; false = black
    ///
    /// Precondition: 0 <= x <= 7 and 0 <= y <= 7
    ///
    /// Returns true if placement was successful.
    pub fn try_place_chip(&mut self, x: i8, y: i8, color: bool) -> bool {
        if self.is_occupied(x, y) {
            return false;
        }

        let mut can_set = false;

        for direction in DIRECTIONS {
            let start_x = x + direction.0;
            let start_y = y + direction.1;

            if Board::is_coord_valid(start_x, start_y)
                && self.is_occupied(start_x, start_y)
                && self.get_color(start_x, start_y) != color
            {
                let mut curr_x = start_x + direction.0;
                let mut curr_y = start_y + direction.1;

                let mut is_valid = false;
                loop {
                    if !Board::is_coord_valid(curr_x, curr_y) {
                        break;
                    }
                    if !self.is_occupied(curr_x, curr_y) {
                        break;
                    }
                    if self.get_color(curr_x, curr_y) == color {
                        is_valid = true;
                        break;
                    }
                    curr_x += direction.0;
                    curr_y += direction.1;
                }

                if is_valid {
                    let mut set_curr_x = start_x;
                    let mut set_curr_y = start_y;

                    while set_curr_x != curr_x || set_curr_y != curr_y {
                        self.set(set_curr_x, set_curr_y, color);
                        set_curr_x += direction.0;
                        set_curr_y += direction.1;
                    }

                    can_set = true;
                }
            }
        }

        if can_set {
            self.set(x, y, color);
        }
        can_set
    }

    /// Determines if a move is valid. This is a dry-run version of
    /// `try_place_chip``.
    ///
    /// Colors: true = white; false = black
    ///
    /// Precondition: 0 <= x <= 7 and 0 <= y <= 7
    pub fn is_valid_move(&self, x: i8, y: i8, color: bool) -> bool {
        if self.is_occupied(x, y) {
            return false;
        }

        for direction in DIRECTIONS {
            let start_x = x + direction.0;
            let start_y = y + direction.1;

            if Board::is_coord_valid(start_x, start_y)
                && self.is_occupied(start_x, start_y)
                && self.get_color(start_x, start_y) != color
            {
                let mut curr_x = start_x + direction.0;
                let mut curr_y = start_y + direction.1;

                loop {
                    if !Board::is_coord_valid(curr_x, curr_y) {
                        break;
                    }
                    if !self.is_occupied(curr_x, curr_y) {
                        break;
                    }
                    if self.get_color(curr_x, curr_y) == color {
                        return true;
                    }
                    curr_x += direction.0;
                    curr_y += direction.1;
                }
            }
        }

        false
    }

    /// Returns a vector of all valid moves for the specified color
    pub fn get_all_valid_moves(&self, color: bool) -> Vec<(i8, i8)> {
        let mut valid_moves = Vec::new();

        for y in 0..8 {
            for x in 0..8 {
                if self.is_valid_move(x, y, color) {
                    valid_moves.push((x, y));
                }
            }
        }

        valid_moves
    }

    /// Checks if a player has any valid move
    pub fn has_valid_move(&self, color: bool) -> bool {
        for y in 0..8 {
            for x in 0..8 {
                if self.is_valid_move(x, y, color) {
                    return true;
                }
            }
        }
        false
    }

    /// Counts the number of placed pieces on the board for a color
    pub fn count_pieces(&self, color: bool) -> u16 {
        let mut count = 0;

        let mut curr = self.filled & (if color { self.color } else { !self.color });

        // Brian Kernighan Algorithm to count 1 digits in binary
        while curr > 0 {
            count += 1;
            curr &= curr - 1;
        }

        count
    }

    fn is_occupied(&self, x: i8, y: i8) -> bool {
        return ((self.filled >> (y * 8 + x)) & 1) != 0;
    }

    fn get_color(&self, x: i8, y: i8) -> bool {
        return ((self.color >> (y * 8 + x)) & 1) != 0;
    }

    fn is_coord_valid(x: i8, y: i8) -> bool {
        x >= 0 && x < 8 && y >= 0 && y < 8
    }
}

impl Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut res = String::from("Board:\n");
        for y in 0..8 {
            res.push('[');
            res.push(' ');

            for x in 0..8 {
                res.push(if self.is_occupied(x, y) {
                    if self.get_color(x, y) { 'O' } else { 'X' }
                } else {
                    '_'
                });
                res.push(' ');
            }

            res.push(']');
            res.push('\n');
        }

        f.write_str(&res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_and_clear() {
        let mut board = Board::new();

        for x in 0..8 {
            for y in 0..8 {
                board.clear(x, y);
                assert!(!board.is_occupied(x, y));

                board.set(x, y, true);
                assert!(board.is_occupied(x, y));
                assert!(board.get_color(x, y));

                board.set(x, y, false);
                assert!(board.is_occupied(x, y));
                assert!(!board.get_color(x, y));

                board.clear(x, y);
                assert!(!board.is_occupied(x, y));
            }
        }
    }
}
