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
    IBlock = 1,
    JBlock = 2,
    LBlock = 3,
    OBlock = 4,
    SBlock = 5,
    TBlock = 6,
    ZBlock = 7,
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
        !self.cells[self.get_index(row, 0)..self.get_index(row, self.width)].iter().any(|&item| item == Cell::None)
    }
}

#[wasm_bindgen]
impl Board {
    pub fn new(width: u32, height: u32) -> Board {
        let cells = vec![Cell::None; (width * height) as usize];

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
            cells: vec![Cell::None,Cell::None,Cell::None,Cell::TBlock,Cell::None,Cell::None,Cell::JBlock,Cell::None,]
        };
        assert_eq!(board.check_row_is_full(1), false);
    }

    #[test]
    fn should_check_row_is_full_false() {
        let board = Board {
            width: 4,
            height: 2,
            cells: vec![Cell::IBlock,Cell::JBlock,Cell::LBlock,Cell::TBlock,Cell::None,Cell::None,Cell::JBlock,Cell::None,]
        };
        assert_eq!(board.check_row_is_full(0), true);
    }
}
