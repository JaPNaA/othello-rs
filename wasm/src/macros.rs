#[macro_export]
macro_rules! _create_board_piece {
    (X) => {
        1
    };
    (O) => {
        2
    };
    (_) => {
        3
    };
}

#[macro_export]
macro_rules! _create_board_row {
    ([$a:tt $b:tt $c:tt $d:tt $e:tt $f:tt $g:tt $h:tt]) => {
        [
            crate::_create_board_piece!($a),
            crate::_create_board_piece!($b),
            crate::_create_board_piece!($c),
            crate::_create_board_piece!($d),
            crate::_create_board_piece!($e),
            crate::_create_board_piece!($f),
            crate::_create_board_piece!($g),
            crate::_create_board_piece!($h),
        ]
    };
}

#[macro_export]
macro_rules! create_board {
    ($a:tt, $b:tt, $c:tt, $d:tt, $e:tt, $f:tt, $g:tt, $h:tt,) => {{
        let mut board = Board::new();
        let data = [
            crate::_create_board_row!($a),
            crate::_create_board_row!($b),
            crate::_create_board_row!($c),
            crate::_create_board_row!($d),
            crate::_create_board_row!($e),
            crate::_create_board_row!($f),
            crate::_create_board_row!($g),
            crate::_create_board_row!($h),
        ];

        for y in 0..8usize {
            for x in 0..8usize {
                match data[y][x] {
                    1 => board.set(x as i8, y as i8, false),
                    2 => board.set(x as i8, y as i8, true),
                    3 => board.clear(x as i8, y as i8),
                    _ => { }
                }
            }
        }

        board
    }};
}
