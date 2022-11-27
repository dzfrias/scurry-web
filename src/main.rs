mod editor;

use editor::Editor;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("h-screen")}>
            <div class={classes!("bg-orange-900", "mb-5")}>
                <div class={classes!("flex")}>
                    <h1 class={classes!("text-4xl", "py-3", "pl-2", "italic", "text-yellow-400", "flex-1")}>
                    {"Scurry"}<span class={classes!("text-xs", "text-yellow-500", "pl-1")}>{"A new take on object-oriented languages"}</span>
                    </h1>
                    <ul class={classes!("pr-9", "self-center", "flex", "gap-10", "text-amber-500", "text-lg")}>
                        // TODO: Link here
                        <a class={classes!("hover:text-yellow-100")}>{"Docs"}</a>
                        <a target="_blank" href="https://github.com/dzfrias/scurry" class={classes!("hover:text-yellow-100")}>{"GitHub"}</a>
                    </ul>
                </div>
            </div>
            <Editor/>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
