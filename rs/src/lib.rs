use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}


#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    None = 0,
    // ----
    IBlock = 1,
    // -
    // ---
    JBlock = 2,
    //   -
    // ---
    LBlock = 3,
    // --
    // --
    OBlock = 4,
    //  --
    // --
    SBlock = 5,
    //  -
    // ---
    TBlock = 6,
    // --
    //  --
    ZBlock = 7,
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Action {
    Left = 0,
    Right = 1,
    Rotate = 2,
    Down = 3,
}

#[wasm_bindgen]
pub struct Board {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

impl Board {
    fn get_index(&self, row: u32, column: u32) -> usize {
        (row * self.width + column) as usize
    }

    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    pub fn check_row_is_full(&self, row: u32) -> bool {
        !self.cells[self.get_index(row, 0)..self.get_index(row, self.width)]
            .iter()
            .any(|&item| item == Cell::None)
    }

    pub fn remove_and_shift_row(&mut self, row: u32) {
        for current_row in (1..=row).rev() {
            for current_column in 0..self.width {
                let index = self.get_index(current_row, current_column);
                let index_above = self.get_index(current_row.wrapping_sub(1), current_column);
                self.cells[index] = self.cells[index_above];
            }
        }
        for current_column in 0..self.width {
            let index = self.get_index(0, current_column);
            self.cells[index] = Cell::None;
        }
    }
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let cells = vec![Cell::LBlock; (width * height) as usize];

        Board {
            width,
            height,
            cells,
        }
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn cells(&self) -> *const Cell {
        self.get_cells().as_ptr()
    }

    pub fn tick(&mut self) {
        for current_row in 0..self.height {
            if self.check_row_is_full(current_row) {
                self.remove_and_shift_row(current_row);
            }
        }
    }

    pub fn action(&self, action: Action) {
        
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_init_board() {
        let board = Board::new(2, 4);
        assert_eq!(board.get_cells(), [Cell::None; 8]);
    }

    #[test]
    fn should_get_width() {
        let board = Board::new(5, 10);
        assert_eq!(board.get_width(), 5);
    }

    #[test]
    fn should_get_height() {
        let board = Board::new(5, 10);
        assert_eq!(board.get_height(), 10);
    }

    #[test]
    fn should_get_index_0_3() {
        let board = Board::new(5, 10);
        assert_eq!(board.get_index(0, 3), 3);
    }

    #[test]
    fn should_get_index_3_4() {
        let board = Board::new(5, 10);
        assert_eq!(board.get_index(3, 4), 19);
    }

    #[test]
    fn should_check_row_is_full_true() {
        let board = Board {
            width: 4,
            height: 2,
            cells: vec![
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::JBlock,
                Cell::None,
            ],
        };
        assert_eq!(board.check_row_is_full(1), false);
    }

    #[test]
    fn should_check_row_is_full_false() {
        let board = Board {
            width: 4,
            height: 2,
            cells: vec![
                Cell::IBlock,
                Cell::JBlock,
                Cell::LBlock,
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::JBlock,
                Cell::None,
            ],
        };
        assert_eq!(board.check_row_is_full(0), true);
    }

    #[test]
    fn should_remove_and_shift_row_1() {
        let mut board = Board {
            width: 3,
            height: 3,
            cells: vec![
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::JBlock,
                Cell::None,
                Cell::None,
                Cell::IBlock,
                Cell::JBlock,
                Cell::LBlock,
            ],
        };
        board.remove_and_shift_row(2);
        assert_eq!(
            board.get_cells(),
            [
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::JBlock,
                Cell::None,
                Cell::None,
            ],
        );
    }

    #[test]
    fn should_remove_and_shift_row_2() {
        let mut board = Board {
            width: 3,
            height: 3,
            cells: vec![
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::IBlock,
                Cell::JBlock,
                Cell::LBlock,
                Cell::JBlock,
                Cell::None,
                Cell::None,
            ],
        };
        board.remove_and_shift_row(1);
        assert_eq!(
            board.get_cells(),
            [
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::JBlock,
                Cell::None,
                Cell::None,
            ],
        );
    }

    #[test]
    fn should_remove_and_shift_row_3() {
        let mut board = Board {
            width: 3,
            height: 3,
            cells: vec![
                Cell::IBlock,
                Cell::JBlock,
                Cell::LBlock,
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::JBlock,
                Cell::None,
                Cell::None,
            ],
        };
        board.remove_and_shift_row(0);
        assert_eq!(
            board.get_cells(),
            [
                Cell::None,
                Cell::None,
                Cell::None,
                Cell::TBlock,
                Cell::None,
                Cell::None,
                Cell::JBlock,
                Cell::None,
                Cell::None,
            ],
        );
    }
}
