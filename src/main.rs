mod editor;

use editor::Editor;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1 class={classes!("bg-sky-300", "text-4xl", "py-2", "pl-2", "italic", "mb-5")}>{"Scurry"}</h1>
            <Editor/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
