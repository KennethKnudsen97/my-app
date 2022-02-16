use minesweeper::{Field, FieldStatus, Game, MAX_COLUMNS, MAX_ROWS};
use rand;
use sdl2::rect::Rect;
use sdl2::{pixels::Color, render::Canvas, video::Window, EventPump};
// define characteristics of a Button
pub struct Button {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
    //  ˆˆˆ ˆˆˆˆ  ˆˆ
    //   |   |   data type
    //   |  field
    //   |
    //   visibility: everyone can read this field
}

pub struct Grid(pub Vec<Vec<Button>>);
//               ˆˆˆˆˆˆˆˆˆˆˆˆˆˆˆˆˆˆ
//               the more complex data
//               type getting a new name

impl Grid {
    pub fn new(columns: u32, rows: u32) -> Self {
        // create a grid *instance*
        let mut grid_vec = Vec::new();
        for row in 0..rows {
            grid_vec.push(Vec::new());
            for _column in 0..columns {
                grid_vec[row as usize].push(Button {
                    red: 150,
                    green: 150,
                    blue: 150,
                });
            }
        }
        Grid(grid_vec)
        //    ˆˆ
        //     no semicolon 👉 grid_vec is returned
        //     from this function
    }
    pub fn display(&self, renderer: &mut Canvas<Window>, columns: u32, rows: u32, cell_width: u32) {
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        for row in 0..rows {
            for column in 0..columns {
                display_cell(renderer, row, column, self, cell_width)
            }
        }
        renderer.present();
    }

    pub fn color_cell(&mut self, row: i32, column: i32, color: Color) {
        self.0[row as usize][column as usize].red = color.r;
        self.0[row as usize][column as usize].green = color.g;
        self.0[row as usize][column as usize].blue = color.b;
    }

    pub fn update_color(&mut self, game: &Game) {
        for (x, fields) in game.game_fields.iter().enumerate() {
            for (y, field) in fields.iter().enumerate() {
                match field.field_status {
                    minesweeper::FieldStatus::Covered => {
                        self.color_cell(x as i32, y as i32, Color::RGB(100, 100, 100))
                    }

                    minesweeper::FieldStatus::Uncovered => match field.field_type {
                        minesweeper::FieldType::Bomb => {
                            self.color_cell(x as i32, y as i32, Color::RGB(0, 0, 0))
                        }
                        minesweeper::FieldType::Empty => {
                            self.color_cell(x as i32, y as i32, Color::RGB(220, 220, 220))
                        }
                        minesweeper::FieldType::Hint => match field.hint {
                            1 => self.color_cell(x as i32, y as i32, Color::RGB(0, 0, 255)),
                            2 => self.color_cell(x as i32, y as i32, Color::RGB(0, 255, 0)),
                            3 => self.color_cell(x as i32, y as i32, Color::RGB(255, 150, 0)),
                            4 => self.color_cell(x as i32, y as i32, Color::RGB(0, 0, 60)),
                            5 => self.color_cell(x as i32, y as i32, Color::RGB(60, 0, 0)),
                            6 => self.color_cell(x as i32, y as i32, Color::RGB(0, 60, 0)),
                            7 => self.color_cell(x as i32, y as i32, Color::RGB(0, 0, 20)),
                            8 => self.color_cell(x as i32, y as i32, Color::RGB(90, 0, 50)),
                            _ => panic!("Wrong number of neightbours"),
                        },
                    },

                    minesweeper::FieldStatus::Marked => {
                        self.color_cell(x as i32, y as i32, Color::RGB(255, 0, 0))
                    }
                }
            }
        }
    }
}

pub fn init(width: u32, height: u32) -> (Canvas<Window>, EventPump) {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("Simulator", width as u32 + 1, height as u32 + 1)
        .position_centered()
        .build()
        .unwrap();

    let mut canvas_renderer = window.into_canvas().present_vsync().build().unwrap();

    canvas_renderer.set_draw_color(Color::RGB(0, 0, 0));
    canvas_renderer.clear();
    canvas_renderer.present();

    let event_pump = sdl_context.event_pump().unwrap();
    (canvas_renderer, event_pump)
}

pub fn display_rectangle(
    canvas_renderer: &mut Canvas<Window>,
    canvas_width: u32,
    canvas_height: u32,
) {
    let red: u8 = rand::random();
    let green: u8 = rand::random();
    let blue: u8 = rand::random();

    canvas_renderer.clear();

    let drawing_color = Color::RGB(red, green, blue);
    canvas_renderer.set_draw_color(drawing_color);

    let square = Rect::new(0, 0, canvas_width, canvas_height);
    canvas_renderer.fill_rect(square);

    canvas_renderer.present();
}

fn display_cell(
    renderer: &mut Canvas<Window>,
    row: u32,
    col: u32,
    grid_data: &Grid,
    cell_width: u32,
) {
    let grid = &grid_data.0;

    // calculate coordinates of displayed cell out of the number of the row or column multiplied with the width of the cell.
    let cell_height = cell_width;

    let (x_button, y_button, button_width, button_height) =
        calculate_button_size(cell_width, row, col);

    // read color values from Button and make them into an RGB color
    let button = &grid[row as usize][col as usize];
    let drawing_color = Color::RGB(button.red, button.green, button.blue);

    // set drawing color and add the square to the renderer
    renderer.set_draw_color(drawing_color);
    let square = Rect::new(x_button, y_button, button_width, button_height);

    renderer.fill_rect(square).unwrap();
}

fn calculate_button_size(cell_width: u32, row: u32, col: u32) -> (i32, i32, u32, u32) {
    // calculate raw coordinates of the cell out of the number of the row or column multiplied with the width of the cell.
    let cell_height = cell_width;
    // they are both the same, because we deal with squares

    // calculate the coordinates of the cell, the part we already did in the other function
    let x_cell = (cell_width) * col;
    let y_cell = (cell_width) * row;

    // we set the half_margin to 3. How big you make it in the end is a matter of preference
    let half_margin = 3_u32;

    // add half_margin as offset to x and y to get x and y of the button
    let x_button = (x_cell + half_margin) as i32;
    let y_button = (y_cell + half_margin) as i32;

    // subtract half_margin * 2 from width and height of the cell to get width and height of the button
    let button_width = cell_width - (half_margin * 2);
    let button_height = cell_height - (half_margin * 2);

    // return all values in a tuple
    (x_button, y_button, button_width, button_height)
}

pub fn where_is_mouse(mouse_x: i32, mouse_y: i32, cell_width: u32) -> (usize, usize) {
    let column = mouse_x / cell_width as i32;
    let row = mouse_y / cell_width as i32;
    (row as usize, column as usize)
}
