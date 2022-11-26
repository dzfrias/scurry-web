mod editor;

use editor::Editor;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("h-screen")}>
            <h1 class={classes!("bg-orange-900", "text-4xl", "py-3", "pl-2", "italic", "mb-5", "text-yellow-400")}>{"Scurry"}</h1>
            <Editor/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
