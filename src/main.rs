mod editor;

use editor::Editor;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[function_component]
fn App() -> Html {
    html! {
        <div class={classes!("h-screen")}>
            <div class={classes!("mb-5")}>
                <div class={classes!("flex")}>
                    <h1 class={classes!("text-6xl", "py-3", "pl-2", "italic", "text-amber-900", "flex-1", "font-['Raleway']", "drop-shadow-lg")}>
                    {"Scurry"}<span class={classes!("text-xs", "text-gray-800", "pl-1", "hidden", "sm:inline-flex")}>{"A refreshing take on object-oriented languages"}</span>
                    </h1>
                    <ul class={classes!("pr-9", "self-center", "flex", "md:gap-10", "text-amber-900", "text-lg", "gap-4")}>
                        // TODO: Link here
                        <a class={classes!("hover:text-amber-700", "flex-row", "flex", "items-center", "gap-1")}><Icon icon_id={IconId::HeroiconsOutlineDocumentText}/><span class={classes!("hidden", "sm:inline")}>{"Docs"}</span></a>
                        <a target="_blank" href="https://github.com/dzfrias/scurry" class={classes!("hover:text-amber-700", "flex-row", "flex", "items-center", "gap-2")}><Icon icon_id={IconId::BootstrapGithub}/><span class={classes!("hidden", "sm:inline")}>{"GitHub"}</span></a>
                    </ul>
                </div>
            </div>
            <div class={classes!("ml-4", "h-screen", "w-full")}>
                <h1 class={classes!("text-4xl", "border-b-2", "mb-3", "md:w-1/5", "border-orange-900", "text-gray-800", "w-1/3", "ml-2", "font-['Maven_Pro']", "font-bold")}>{"About"}</h1>
                <div class={classes!("flex-col", "md:flex-row", "flex", "mr-8")}>
                    <p class={classes!("mb-5", "flex-1", "rounded", "p-3", "text-lg", "font-['Maven_Pro']", "text-gray-800", "mr-14")}>
                        {"Scurry is a strictly component-based object-oriented language. The goal of Scurry is to limit shared state while still allowing users to use the intuitive nature of object-oriented programming."}
                        <br/><br/>
                        {"While Scurry is dynamically typed, it encourages type annotations and has features that allow nasty type errors to be caught early in a program's lifetime."}
                        <br/><br/>
                        {"On top of this, Scurry aims to be as intuitive as possible, offering both little syntax and syntax that users will feel familiar with if they've programmed before."}
                    </p>
                    <div class={classes!("self-center", "mr-8", "rounded-lg", "bg-gray-300", "[&_code]:hover:blur-[1px]", "[&_p]:hover:visible", "flex", "items-center", "justify-center", "hidden", "md:flex", "drop-shadow-md")}>
                        <code class={classes!("whitespace-pre", "font-mono", "p-4")}>{"decl Example {
    field: String

    fn $new(self) { self.field = \"Hello, world!\"; }

    exp fn print(self) { println(self.field); }
}

example = Example();
// This was a pretty convoluted hello world...
example.print();
// See the docs in the menu bar for more!"}</code>
                        <p class={classes!("invisible", "blur-none", "absolute", "text-amber-700", "text-3xl", "font-['Maven_Pro']", "font-bold")}>{"Copy"}</p>
                    </div>
                    <p class={classes!("hidden", "2xl:inline", "flex-1")}></p>
                </div>
                <h1 class={classes!("text-4xl", "border-b-2", "mb-3", "md:w-1/5", "border-orange-900", "text-gray-800", "w-1/3", "ml-2", "font-['Maven_Pro']", "font-bold")}>{"Playground"}</h1>
                <Editor/>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
