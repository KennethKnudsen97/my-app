use host::canvas_module;
use minesweeper::{FieldStatus, FieldType, Game, MAX_COLUMNS, MAX_ROWS};
use sdl2::mouse::MouseButton;
use sdl2::{event::Event, keyboard::Keycode};
use std::{thread, time};

fn main() {
    let canvas_width = 720_u32;
    let canvas_height = 720_u32;
    let columns = MAX_COLUMNS as u32;
    let rows = MAX_ROWS as u32;
    let cell_width = canvas_width / columns;
    let bomb_number = 10;
    let (mut canvas_renderer, mut events) = canvas_module::init(canvas_width, canvas_height);
    let mut grid = canvas_module::Grid::new(columns, rows);
    let mut game = minesweeper::Game::new(bomb_number);

    'event_loop: loop {
        for event in events.poll_iter() {
            match event {
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Left,
                    x,
                    y,
                    ..
                } => {
                    if !game.game_lost {
                        let (row, column) = canvas_module::where_is_mouse(x, y, cell_width);
                        game.game_fields[row][column].left_click();

                        if game.game_fields[row][column].field_type == FieldType::Empty {
                            game.uncover_neighbours(row, column);
                        } else if game.game_fields[row][column].field_type == FieldType::Bomb {
                            game.game_lost = true;
                            game.uncover_bombs();
                        }

                        dbg!(game.game_fields[row][column]);
                    }
                }
                Event::MouseButtonDown {
                    mouse_btn: MouseButton::Right,
                    x,
                    y,
                    ..
                } => {
                    if !game.game_lost {
                        let (row, column) = canvas_module::where_is_mouse(x, y, cell_width);
                        game.game_fields[row][column].right_click();
                    }
                }
                Event::KeyDown {
                    keycode: Some(Keycode::R),
                    ..
                } => game = Game::new(bomb_number),
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'event_loop,
                _ => continue,
            }
        }
        grid.update_color(&game);
        grid.display(&mut canvas_renderer, columns, rows, cell_width);

        //thread::sleep(time::Duration::from_millis(1000));
    }
}
