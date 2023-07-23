extern crate termion;

// Set up termion
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;
use termion::cursor;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};

struct GridPosition {
    x_position: u16,
    y_position: u16,
}

fn print_field(
    stdout: &mut RawTerminal<std::io::Stdout>,
    grid: &Vec<Vec<bool>>,
    cursor_pos: &GridPosition,
    show_cursor: bool,
) {
    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            let c = if cell { '#' } else { ' ' };
            write!(stdout, "{}", cursor::Goto(x as u16 + 1, y as u16 + 1)).unwrap();
            if show_cursor && x as u16 == cursor_pos.x_position && y as u16 == cursor_pos.y_position
            {
                write!(
                    stdout,
                    "{}{}{}",
                    termion::style::Invert,
                    c,
                    termion::style::Reset
                )
                .unwrap();
            } else {
                write!(
                    stdout,
                    "{}{}{}",
                    termion::color::Fg(termion::color::White),
                    c,
                    termion::color::Fg(termion::color::Reset)
                )
                .unwrap();
            }
        }
    }
}

fn get_num_neighbors(grid: &Vec<Vec<bool>>, y: usize, x: usize) -> u8 {
    let mut num_neighbors = 0;
    let rows = grid.len();
    let cols = grid[0].len();

    for i in 0..3 {
        for j in 0..3 {
            if i == 1 && j == 1 {
                continue;
            }

            let y = y as i32 + i as i32 - 1;
            let x = x as i32 + j as i32 - 1;
            if y >= 0 && x >= 0 && y < rows as i32 && x < cols as i32 {
                if grid[y as usize][x as usize] {
                    num_neighbors += 1;
                }
            }
        }
    }
    num_neighbors
}

fn step_simulation(grid: &Vec<Vec<bool>>) -> Vec<Vec<bool>> {
    let side_length = grid.len();
    let mut grid_buffer: Vec<Vec<bool>> = vec![vec![false; side_length]; side_length];

    for (y, row) in grid.iter().enumerate() {
        for (x, &cell) in row.iter().enumerate() {
            // get number of neighbors for the current cell.
            let number_of_neighbors = get_num_neighbors(&grid, y, x);

            // apply rules
            if cell {
                if number_of_neighbors == 2 || number_of_neighbors == 3 {
                    grid_buffer[y][x] = true;
                } // else cell dies from under- or over-population, default false
            } else {
                if number_of_neighbors == 3 {
                    grid_buffer[y][x] = true;
                } // else cell stays dead, default false
            }
        }
    }
    grid_buffer
}

fn main() -> Result<(), std::io::Error> {
    let mut stdout = stdout().into_raw_mode()?;
    let side_length: usize = 40;
    let stdin = stdin();
    let mut cursor_pos = GridPosition {
        x_position: 1,
        y_position: 1,
    };

    let mut grid: Vec<Vec<bool>> = vec![vec![false; side_length]; side_length];

    write!(stdout, "{}", termion::cursor::Hide).unwrap();
    for choice in stdin.keys() {
        // Clear the terminal:
        write!(stdout, "{}", termion::clear::All).unwrap();
        stdout.flush().unwrap();
        // Print initial grid

        // Handle input
        match choice {
            Ok(key) => {
                match key {
                    Key::Up => {
                        if cursor_pos.y_position > 0 {
                            cursor_pos.y_position -= 1;
                        } else {
                            cursor_pos.y_position = side_length as u16 - 1;
                        }
                    }
                    Key::Down => {
                        cursor_pos.y_position += 1;
                        cursor_pos.y_position =
                            (cursor_pos.y_position + side_length as u16) % side_length as u16;
                    }
                    Key::Left => {
                        if cursor_pos.x_position > 0 {
                            cursor_pos.x_position -= 1;
                        } else {
                            cursor_pos.x_position = side_length as u16 - 1;
                        }
                    }
                    Key::Right => {
                        cursor_pos.x_position += 1;
                        cursor_pos.x_position =
                            (cursor_pos.x_position + side_length as u16) % side_length as u16;
                    }
                    Key::Char(' ') => {
                        grid[cursor_pos.y_position as usize][cursor_pos.x_position as usize] = true;
                    }
                    Key::Char('\n') => {
                        // Run simulation
                        break;
                    }
                    _ => {
                        break;
                    }
                }
            }
            Err(err) => {
                // print an error message and continue
                eprintln!("Error reading key: {}", err);
                continue;
            }
        }
        print_field(&mut stdout, &grid, &cursor_pos, true);
    }

    // Run simulation
    for i in 0..500 {
        // Display the generation
        write!(
            stdout,
            "{}Generation: {}",
            cursor::Goto(0, side_length as u16 + 5),
            i
        )
        .unwrap();

        // Print the board
        print_field(&mut stdout, &grid, &cursor_pos, false);

        // copy the board to a buffer.
        let grid_buffer = step_simulation(&grid);

        // set the board = to the buffered state.
        grid = grid_buffer.clone();

        sleep(Duration::from_millis(100));
        stdout.flush().unwrap();
    }
    // Show the cursor and flush the output. Return the error at the end.
    write!(stdout, "{}\n", termion::cursor::Show).unwrap();
    stdout.flush().unwrap();
        write!(stdout, "{}", termion::clear::All).unwrap();
    Ok(())
}
