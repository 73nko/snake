mod board;
mod random;
mod snake;
mod square;

use gloo::timers::callback::Interval;
use snake::{Direction, SnakeGame};
use yew::{html, Component, Context, Html, KeyboardEvent};

use board::Board;

pub enum Msg {
    Move(Direction),
    UpdateTime,
}

impl Component for SnakeGame {
    type Message = Msg;

    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let timer = Interval::new(200, move || link.send_message(Msg::UpdateTime));
        SnakeGame::new(15, 15, timer)
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Move(dir) => self.change_direction(dir),
            Msg::UpdateTime => self.tick(),
        };

        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let height = self.height;
        let width = self.width;
        let food = self.food;
        let snake = self.snake.clone();

        let mut squares: Vec<String> = vec![];

        let handle_key_press = ctx
            .link()
            .batch_callback(|evt: KeyboardEvent| match &*evt.key() {
                "ArrowUp" => Some(Msg::Move(Direction::Up)),
                "ArrowRight" => Some(Msg::Move(Direction::Right)),
                "ArrowDown" => Some(Msg::Move(Direction::Down)),
                "ArrowLeft" => Some(Msg::Move(Direction::Left)),
                _ => None,
            });

        for y in 0..height {
            for x in 0..width {
                let pos = (x, y);
                let square = if pos == food {
                    "🍎"
                } else if snake.get(0) == Some(&pos) {
                    "❇️"
                } else if snake.contains(&pos) {
                    "🟩"
                } else {
                    " "
                };
                squares.push(square.to_string());
            }
        }

        if self.finished {
            html! {
                <div>
                    <h2>{ "Snake" }</h2>
                    <h3>{ "End Game! " }</h3>
                </div>
            }
        } else {
            html! {
                <div>
                    <h2>{ "Snake" }</h2>
                    <Board {squares} {height} {width} {handle_key_press} />
                </div>
            }
        }
    }
}

fn main() {
    yew::start_app::<SnakeGame>();
}
