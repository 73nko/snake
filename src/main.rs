mod random;
mod snake;

use gloo::{
    console::{self, Timer},
    timers::callback::{Interval, Timeout},
};
use snake::{Direction, SnakeGame};
use web_sys::HtmlDivElement;
use yew::{html, Callback, Component, Context, Html, KeyboardEvent, NodeRef, Properties};

struct Board {
    node: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    squares: Vec<String>,
    height: usize,
    width: usize,
    handle_key_press: Callback<KeyboardEvent>,
}

impl Component for Board {
    type Message = String;
    type Properties = BoardProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node: NodeRef::default(),
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(div) = self.node.cast::<HtmlDivElement>() {
                div.focus();
            } else {
                console::error!("No div element");
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let positions = ctx.props().squares.clone();
        let height = ctx.props().height;
        let width = ctx.props().width;
        let style = format!(
            "grid-template: repeat({}, auto) / repeat({}, auto)",
            height, width
        );
        html! {
            <div
                tabindex="0"
                class="board"
                style={style}
                ref={self.node.clone()}
                onkeydown={ctx.props().handle_key_press.clone()}
            >
            { positions.iter().map(move |pos| {
                html!{<Square square={pos.clone()} />}
            }).collect::<Html>() }
            </div>
        }
    }
}

struct Square;

#[derive(Properties, PartialEq)]
pub struct SquareProps {
    square: String,
}

impl Component for Square {
    type Message = String;
    type Properties = SquareProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let square = ctx.props().square.clone();

        html! {
            <div class="field">
                { square }
            </div>
        }
    }
}

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

        let mut positions: Vec<String> = vec![];

        let handle_key_press =
            ctx.link()
                .batch_callback(|evt: KeyboardEvent| match &evt.key()[..] {
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
                positions.push(square.to_string());
            }
        }

        html! {
            <div>
                <h2>{ "Snake" }</h2>
                <Board squares={positions} {height} {width} handle_key_press={handle_key_press} />
            </div>
        }
    }
}

fn main() {
    yew::start_app::<SnakeGame>();
}
