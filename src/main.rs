use std::io::stdout;

use crossterm::{
    cursor::MoveTo,
    event::{self, Event, KeyCode},
    execute,
    style::Print,
    terminal::{enable_raw_mode, Clear},
};

const WIDTH: u16 = 80;
const HEIGHT: u16 = 20;

fn draw_paddles(x: u16, y: u16, height: u16) {
    for i in 0..height {
        match execute!(stdout(), MoveTo(x, y + i), Print("█")) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}

fn main() {
    enable_raw_mode().unwrap();
    let mut score_left = 0;
    let mut score_right = 0;
    let left_paddle_x = 1;
    let right_paddle_x = WIDTH - 1;
    let mut left_paddle_y = 10;
    let mut right_paddle_y = 10;

    let mut ball_x: i16 = 5;
    let mut ball_y: i16 = 5;
    let mut dx: i16 = 1;
    let mut dy: i16 = 1;

    match execute!(stdout(), MoveTo(0, 0), Print("Hello to Pong")) {
        Ok(_) => {}
        Err(e) => eprintln!("Error: {}", e),
    }

    loop {
        match execute!(stdout(), Clear(crossterm::terminal::ClearType::All)) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }

        match execute!(
            stdout(),
            MoveTo(35, 25),
            Print(format!("Score: {} - {}", score_left, score_right))
        ) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
        draw_paddles(left_paddle_x, left_paddle_y, 4);
        draw_paddles(right_paddle_x, right_paddle_y, 4);

        match execute!(stdout(), MoveTo(ball_x as u16, ball_y as u16), Print("🏀")) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }

        if score_left == 10 {
            println!("Left player wins");
            break;
        } else if score_right == 10 {
            println!("Right player wins");
            break;
        }

        if ball_x == 0 {
            score_right += 1;
            ball_x = 40;
            ball_y = 10;
            dx = 1;
            dy = 1;
        } else if ball_x == 79 {
            score_left += 1;
            ball_x = 40;
            ball_y = 10;
            dx = -1;
            dy = 1;
        }

        if ball_y == 0 {
            ball_y = 1;
            dy = 1;
        } else if ball_y == 19 {
            ball_y = 18;
            dy = -1;
        } else {
            ball_y += dy;
        }

        if ball_x == 1 && ball_y as u16 >= left_paddle_y && ball_y as u16 <= left_paddle_y + 4 {
            dx = 1;
            ball_x += dx;
        } else if ball_x == 78
            && ball_y as u16 >= right_paddle_y
            && ball_y as u16 <= right_paddle_y + 4
        {
            dx = -1;
            ball_x += dx;
        } else {
            ball_x += dx;
        }

        if let Ok(true) = event::poll(std::time::Duration::from_millis(100)) {
            if let Event::Key(key) = match event::read() {
                Ok(event) => event,
                Err(_) => continue,
            } {
                match key.code {
                    KeyCode::Up => {
                        if right_paddle_y > 0 {
                            right_paddle_y -= 1;
                        }
                    }
                    KeyCode::Down => {
                        if right_paddle_y < 16 {
                            right_paddle_y += 1;
                        }
                    }
                    KeyCode::Char('W') | KeyCode::Char('w') => {
                        if left_paddle_y > 0 {
                            left_paddle_y -= 1;
                        }
                    }
                    KeyCode::Char('S') | KeyCode::Char('s') => {
                        if left_paddle_y < 16 {
                            left_paddle_y += 1;
                        }
                    }
                    KeyCode::Esc => break,
                    _ => continue,
                }
            }
        }
    }
}
