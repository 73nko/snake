use yew::{function_component, html, Properties};

#[derive(Properties, PartialEq)]
pub struct SquareProps {
    pub square: String,
}

#[function_component(Square)]
pub fn square(props: &SquareProps) -> Html {
    html! {
        <div class="field">
            { &props.square }
        </div>
    }
}
