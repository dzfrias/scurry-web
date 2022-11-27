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
                    {"Scurry"}<span class={classes!("text-xs", "text-yellow-500", "pl-1", "hidden", "sm:inline-flex")}>{"A refreshing take on object-oriented languages"}</span>
                    </h1>
                    <ul class={classes!("pr-9", "self-center", "flex", "gap-10", "text-amber-500", "text-lg")}>
                        // TODO: Link here
                        <a class={classes!("hover:text-yellow-100")}>{"Docs"}</a>
                        <a target="_blank" href="https://github.com/dzfrias/scurry" class={classes!("hover:text-yellow-100")}>{"GitHub"}</a>
                    </ul>
                </div>
            </div>
            <div class={classes!("ml-4", "h-screen", "w-full")}>
                <h1 class={classes!("text-4xl", "border-b-2", "mb-3", "md:w-1/5", "border-yellow-300", "text-orange-900", "w-1/3", "ml-2")}>{"About"}</h1>
                <div class={classes!("flex-col", "md:flex-row", "flex", "mr-8")}>
                    <p class={classes!("mb-5", "flex-1", "bg-gray-200", "rounded", "p-3", "active:bg-gray-100", "text-base")}>
                        {"Scurry is a strictly component-based object-oriented language. The goal of Scurry is to limit shared state while still allowing users to use the intuitive nature of object-oriented programming."}
                        <br/><br/>
                        {"While Scurry is dynamically typed, it encourages type annotations and has features that allow nasty type errors to be caught early in a program's lifetime."}
                        <br/><br/>
                        {"On top of this, Scurry aims to be as intuitive as possible, offering both little syntax and syntax that users will feel familiar with if they've programmed before."}
                    </p>
                    // TODO: Image
                    <p class={classes!("flex-1")}></p>
                    // TODO: Image
                    <p class={classes!("flex-1")}></p>
                </div>
                <h1 class={classes!("text-4xl", "border-b-2", "mb-3", "md:w-1/5", "border-yellow-300", "text-orange-900", "w-1/3", "ml-2")}>{"Playground"}</h1>
                <Editor/>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
