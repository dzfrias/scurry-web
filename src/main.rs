mod bar_item;
mod editor;
mod example;

use bar_item::BarItem;
use editor::Editor;
use example::Example;
use yew::prelude::*;
use yew_icons::IconId;

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("h-screen", "2xl:w-3/4", "m-auto")}>
            <div class={classes!("mb-5")}>
                <div class={classes!("flex")}>
                    <h1 class={classes!("text-6xl", "py-3", "pl-2", "italic", "text-amber-900", "flex-1", "font-['Raleway']", "drop-shadow-lg")}>
                    {"Scurry"}<span class={classes!("text-xs", "text-gray-800", "pl-1", "hidden", "sm:inline-flex")}>{"A refreshing take on object-oriented languages"}</span>
                    </h1>
                    <ul class={classes!("pr-9", "self-center", "flex", "md:gap-10", "gap-4")}>
                        // TODO: Link here
                        <BarItem name="Docs" target="https://github.com/dzfrias/scurry" icon_id={IconId::HeroiconsOutlineDocumentText}/>
                        <BarItem name="GitHub" target="https://github.com/dzfrias/scurry" icon_id={IconId::BootstrapGithub}/>
                    </ul>
                </div>
            </div>
            <div class={classes!("h-screen", "mx-4")}>
                <h1 class={classes!("text-4xl", "border-b-2", "mb-3", "md:w-1/5", "border-orange-900", "text-gray-800", "w-1/3", "ml-2", "font-main", "font-bold")}>{"About"}</h1>
                <div class={classes!("flex-col", "md:flex-row", "flex")}>
                    <p class={classes!("mb-5", "flex-1", "rounded", "p-3", "text-lg", "font-main", "text-gray-800", "mr-14")}>
                        {"Scurry is a strictly component-based object-oriented language. The goal of Scurry is to limit shared state while still allowing users to use the intuitive nature of object-oriented programming."}
                        <br/><br/>
                        {"While Scurry is dynamically typed, it encourages type annotations and has features that allow nasty type errors to be caught early in a program's lifetime."}
                        <br/><br/>
                        {"On top of this, Scurry aims to be as intuitive as possible, offering both little syntax and syntax that users will feel familiar with if they've programmed before."}
                    </p>
                    <Example code={include_str!("../static/examples/hello_world.scy")}/>
                </div>
                <h1 class={classes!("text-4xl", "border-b-2", "mb-3", "md:w-1/5", "border-orange-900", "text-gray-800", "w-1/3", "ml-2", "font-main", "font-bold")}>{"Playground"}</h1>
                <Editor/>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
