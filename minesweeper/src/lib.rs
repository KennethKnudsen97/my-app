//#![no_std]

use std::ops::RangeBounds;

use rand::Rng;
pub const MAX_ROWS: usize = 8;
pub const MAX_COLUMNS: usize = 8;

#[derive(Debug)]
pub struct Game {
    pub game_fields: [[Field; MAX_ROWS]; MAX_COLUMNS],
    pub game_lost: bool,
}

impl Game {
    pub fn new(bomb_number: u8) -> Self {
        let mut game_fields = [[Field::default(); MAX_ROWS]; MAX_COLUMNS];
        let mut rng = rand::thread_rng();
        let mut row: usize = rng.gen_range(0..MAX_ROWS);
        let mut col: usize = rng.gen_range(0..MAX_COLUMNS);

        //Generating random bomb fields
        let mut bomb_count: u8 = 0;
        while bomb_count != bomb_number {
            //Only make empty field = bomb
            if game_fields[row][col].field_type == FieldType::Empty {
                game_fields[row][col].field_type = FieldType::Bomb;
                bomb_count += 1;
            }

            //get random field
            row = rng.gen_range(0..MAX_ROWS);
            col = rng.gen_range(0..MAX_COLUMNS);
        }

        //Calculation how many bomb are next to each field.
        let mut bombs = 0;
        for (x, fields) in game_fields.into_iter().enumerate() {
            for (y, field) in fields.into_iter().enumerate() {
                if field.field_type == FieldType::Empty {
                    bombs = calculate_hint_field(&game_fields, x, y);
                    if bombs > 0 {
                        game_fields[x][y].field_type = FieldType::Hint;
                        game_fields[x][y].hint = bombs;
                    }
                }
            }
        }

        Game {
            game_fields,
            game_lost: false,
        }
    }

    pub fn uncover_neighbours(&mut self, row: usize, col: usize) {
        let mut row_check_idx: i32;
        let mut col_check_idx: i32;

        for x in 0..3 {
            row_check_idx = (row + x) as i32 - 1;

            if row_check_idx > -1 && row_check_idx < MAX_ROWS as i32 {
                for y in 0..3 {
                    col_check_idx = (col + y) as i32 - 1;

                    if col_check_idx >= 0 && col_check_idx < MAX_COLUMNS as i32 {
                        let field =
                            self.game_fields[row_check_idx as usize][col_check_idx as usize];

                        if field.field_type == FieldType::Hint {
                            self.game_fields[row_check_idx as usize][col_check_idx as usize]
                                .field_status = FieldStatus::Uncovered;
                        } else if field.field_type == FieldType::Empty
                            && field.field_status == FieldStatus::Covered
                        {
                            self.game_fields[row_check_idx as usize][col_check_idx as usize]
                                .field_status = FieldStatus::Uncovered;
                            self.uncover_neighbours(row_check_idx as usize, col_check_idx as usize);
                        }
                    }
                }
            }
        }
    }

    pub fn uncover_bombs(&mut self) {
        for row in &mut self.game_fields {
            for field in row {
                if field.field_type == FieldType::Bomb {
                    field.field_status = FieldStatus::Uncovered;
                }
            }
        }
    }
}

fn calculate_hint_field(
    game_fields: &[[Field; MAX_ROWS]; MAX_COLUMNS],
    row: usize,
    col: usize,
) -> u8 {
    let mut hint_number = 0;
    let mut row_check_idx: i32 = 0;
    let mut col_check_idx: i32 = 0;

    for x in 0..3 {
        row_check_idx = (row + x) as i32 - 1;

        if row_check_idx > -1 && row_check_idx < MAX_ROWS as i32 {
            for y in 0..3 {
                col_check_idx = (col + y) as i32 - 1;

                if col_check_idx >= 0
                    && col_check_idx < MAX_COLUMNS as i32
                    && game_fields[row_check_idx as usize][col_check_idx as usize].field_type
                        == FieldType::Bomb
                {
                    hint_number += 1;
                }
            }
        }
    }
    hint_number
}

#[derive(Debug, Clone, Copy)]
pub struct Field {
    pub field_status: FieldStatus,
    pub field_type: FieldType,
    pub hint: u8, //Number of neighbouring bombs
}

impl Default for Field {
    fn default() -> Self {
        Field {
            field_status: FieldStatus::Covered,
            field_type: FieldType::Empty,
            hint: 0,
        }
    }
}

impl Field {
    pub fn right_click(&mut self) {
        match self.field_status {
            FieldStatus::Covered => self.field_status = FieldStatus::Marked,
            FieldStatus::Marked => self.field_status = FieldStatus::Covered,
            FieldStatus::Uncovered => println!("This field is uncovered"),
        }
    }
    pub fn left_click(&mut self) {
        self.field_status = FieldStatus::Uncovered;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldType {
    Bomb,
    Empty,
    Hint,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FieldStatus {
    Covered,
    Uncovered,
    Marked,
}
