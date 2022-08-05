use gloo::console;
use web_sys::{HtmlDivElement, KeyboardEvent};
use yew::{html, Callback, Component, Context, Html, NodeRef, Properties};

use crate::square::Square;

pub(crate) struct Board {
    node: NodeRef,
}

#[derive(Properties, PartialEq)]
pub struct BoardProps {
    pub squares: Vec<String>,
    pub height: usize,
    pub width: usize,
    pub handle_key_press: Callback<KeyboardEvent>,
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
