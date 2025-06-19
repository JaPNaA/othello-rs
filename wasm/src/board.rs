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

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
