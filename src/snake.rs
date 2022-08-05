use gloo::timers::callback::Interval;
use random::random_range;
use std::collections::VecDeque;

use crate::random;

pub type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
pub struct SnakeGame {
    pub width: usize,
    pub height: usize,
    pub snake: VecDeque<Pos>,
    pub direction: Direction,
    next_direction: Direction,
    pub food: Pos,
    pub finished: bool,
    _timer: Interval,
}

impl SnakeGame {
    pub fn new(width: usize, height: usize, timer: Interval) -> SnakeGame {
        SnakeGame {
            width,
            height,
            snake: [((width - 2).max(0), height / 2)].into_iter().collect(),
            direction: Direction::Left,
            next_direction: Direction::Left,
            food: (2.min(width - 1), height / 2),
            finished: false,
            _timer: timer,
        }
    }

    pub fn change_direction(&mut self, direction: Direction) {
        match (&self.direction, direction) {
            (Direction::Up, Direction::Up)
            | (Direction::Up, Direction::Down)
            | (Direction::Right, Direction::Right)
            | (Direction::Right, Direction::Left)
            | (Direction::Down, Direction::Up)
            | (Direction::Down, Direction::Down)
            | (Direction::Left, Direction::Right)
            | (Direction::Left, Direction::Left) => (),

            (_, direction) => {
                self.next_direction = direction;
            }
        }
    }

    pub fn tick(&mut self) {
        if self.finished || self.snake.is_empty() {
            return;
        }

        self.direction = self.next_direction;

        let head = self.snake.get(0);
        let new_head = head.map(|&(x, y)| match &self.direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Right => (x + 1, y),
            Direction::Left => (x - 1, y),
        });

        if let Some(new_head) = new_head {
            if !self.is_valid(new_head) || self.is_touching_snake(&new_head) {
                self.finished = true;
            } else {
                if new_head != self.food {
                    self.snake.pop_back();
                } else {
                    self.eat()
                }

                self.snake.push_front(new_head);
            }
        }
    }

    fn is_valid(&self, (x, y): Pos) -> bool {
        x < self.width || y < self.height
    }

    fn is_touching_snake(&self, pos: &Pos) -> bool {
        self.snake.contains(pos)
    }

    fn eat(&mut self) {
        let free_position = (0..self.height)
            .flat_map(|y| (0..self.width).map(move |x| (x, y)))
            .filter(|pos| !self.snake.contains(pos))
            .collect::<Vec<_>>();

        if free_position.is_empty() {
            self.finished = true;
            return;
        };

        self.food = free_position[random_range(0, free_position.len())];
    }
}
