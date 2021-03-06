use piston_window::*;
use piston_window::types::Color;

use rand::{thread_rng, Rng};

use snake::{Direction, Snake};
use draw::{draw_block, draw_rectangle, draw_number};

const FOOTER_HEIGHT: i32 = 6;

const FOOD_COLOR: Color = [0.80, 0.00, 0.00, 1.0];
const FLASHING_COLOR: Color = [0.90, 0.90, 0.90, 1.0];
const BORDER_COLOR: Color = [0.00, 0.00, 0.00, 1.0];
const SCORE_COLOR: Color = [0.80, 0.80, 0.80, 1.0];
const GAMEOVER_COLOR: Color = [0.90, 0.00, 0.00, 0.5];

const MOVING_PERIOD: f64 = 0.1;
const FLASHING_PERIOD: f64 = 0.2;
const RESTART_TIME: f64 = 1.0;

pub struct Game {
    snake: Snake,

    food_exists: bool,
    food_flashing: bool,
    food_x: i32,
    food_y: i32,

    score: u32,

    width: i32,
    height: i32,

    game_over: bool,
    waiting_time: f64,
    flashing_time: f64,
}

impl Game {
    pub fn new(width: i32, height: i32) -> Game {
        let field_height = height - FOOTER_HEIGHT;
        Game {
            snake: Snake::new(2, 2),
            waiting_time: 0.0,
            flashing_time: 0.0,
            food_exists: true,
            food_flashing: false,
            food_x: 6,
            food_y: 4,
            score: 0,
            width,
            height: field_height,
            game_over: false,
        }
    }

    pub fn key_pressed(&mut self, key: Key) {
        if self.game_over {
            return;
        }

        let dir = match key {
            Key::Up => Some(Direction::Up),
            Key::Down => Some(Direction::Down),
            Key::Left => Some(Direction::Left),
            Key::Right => Some(Direction::Right),
            _ => Some(self.snake.head_direction())
        };

        if dir.unwrap() == self.snake.head_direction().opposite() {
            return;
        }

        self.update_snake(dir);
    }

    pub fn draw(&self, con: &Context, g: &mut G2d) {
        self.snake.draw(con, g);

        if self.food_exists {
            match self.food_flashing {
                true => draw_block(FOOD_COLOR, self.food_x, self.food_y, con, g),
                false => draw_block(FLASHING_COLOR, self.food_x, self.food_y, con, g),
            };
        }

        draw_rectangle(BORDER_COLOR, 0, 0, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, self.height - 1, self.width, 1, con, g);
        draw_rectangle(BORDER_COLOR, 0, 0, 1, self.height, con, g);
        draw_rectangle(BORDER_COLOR, self.width - 1, 0, 1, self.height, con, g);

        draw_rectangle(BORDER_COLOR, 0, self.height, self.width, FOOTER_HEIGHT, con, g);

        let (score_x, score_y) = (self.width - 2, self.height + 1);
        draw_number(SCORE_COLOR, self.score, score_x, score_y, con, g);

        if self.game_over {
            draw_rectangle(GAMEOVER_COLOR, 0, 0, self.width, self.height, con, g);
        }
    }

    pub fn update(&mut self, delta_time: f64) {
        self.waiting_time += delta_time;
        self.flashing_time += delta_time;

        if self.game_over {
            if self.waiting_time > RESTART_TIME {
                self.restart();
            }
            return;
        }

        if !self.food_exists {
            self.add_food();
        }

        if self.waiting_time > MOVING_PERIOD {
            self.update_snake(None);
        }

        if self.flashing_time > FLASHING_PERIOD {
            self.update_food();
        }
    }

    fn check_eating(&mut self) {
        let (head_x, head_y): (i32, i32) = self.snake.head_position();
        if self.food_exists && self.food_x == head_x && self.food_y == head_y {
            self.score += 1;
            self.food_exists = false;
            self.snake.restore_tail();
        }
    }

    fn check_if_snake_alive(&self, dir: Option<Direction>) -> bool {
        let (next_x, next_y) = self.snake.next_head(dir);

        if self.snake.overlap_tail(next_x, next_y) {
            return false;
        }

        next_x > 0 && next_y > 0 && next_x < self.width - 1 && next_y < self.height - 1
    }

    fn add_food(&mut self) {
        let mut rng = thread_rng();

        let mut new_x = rng.gen_range(1, self.width - 1);
        let mut new_y = rng.gen_range(1, self.height - 1);
        while self.snake.overlap_tail(new_x, new_y) {
            new_x = rng.gen_range(1, self.width - 1);
            new_y = rng.gen_range(1, self.height - 1);
        }

        self.food_x = new_x;
        self.food_y = new_y;
        self.food_exists = true;
    }

    fn update_food(&mut self) {
        self.food_flashing = !self.food_flashing;
        self.flashing_time = 0.0;
    }

    fn update_snake(&mut self, dir: Option<Direction>) {
        if self.check_if_snake_alive(dir) {
            self.snake.move_forward(dir);
            self.check_eating();
        } else {
            self.game_over = true;
        }
        self.waiting_time = 0.0;
    }

    fn restart(&mut self) {
        self.snake = Snake::new(2, 2);
        self.waiting_time = 0.0;
        self.flashing_time = 0.0;
        self.food_exists = true;
        self.food_flashing = false;
        self.food_x = 6;
        self.food_y = 4;
        self.score = 0;
        self.game_over = false;
    }
}
