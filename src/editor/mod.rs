mod run_button;

use run_button::RunButton;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlTextAreaElement;
use yew::prelude::*;

#[function_component]
pub fn Editor() -> Html {
    let output = use_state_eq(|| "".to_owned());
    let set_output = {
        let output = output.clone();
        move |out: String| output.set(out)
    };
    let onkeydown = {
        move |key: KeyboardEvent| {
            if key.key() == *"Tab" {
                key.prevent_default();
                let editor = web_sys::window()
                    .expect_throw("window should be available")
                    .document()
                    .expect_throw("document should be available")
                    .get_element_by_id("editor")
                    .expect_throw("editor should exist")
                    .dyn_into::<HtmlTextAreaElement>()
                    .expect_throw("element should be a textarea");
                let start = editor.selection_start().unwrap_throw().unwrap_throw();
                // Insert tab
                editor.set_range_text("    ").unwrap_throw();
                // Move cursor forward 4 spaces
                editor.set_selection_start(Some(start + 4)).unwrap_throw();
            }
        }
    };

    html! {
        <div class={classes!("mx-4", "flex", "gap-2", "h-2/3", "flex-col", "md:flex-row")}>
            <div class={classes!("h-full", "md:w-1/2", "md:resize-x", "overflow-auto", "min-w-[25%]", "cursor-col-resize")}>
                <ul class={classes!("py-1", "pl-4", "bg-yellow-900", "pt-2", "rounded")}>
                    <RunButton {set_output}/>
                    // TODO: Toggle REPL button
                </ul>
                <textarea id="editor" {onkeydown} spellcheck="false" placeholder="Enter code here" class={classes!("w-full", "font-mono", "text-ml", "p-2", "h-full", "focus:outline-none")}></textarea>
            </div>
            <div class={classes!("h-full", "md:w-1/2", "overflow-auto", "flex-auto")}>
                <p class={classes!("py-1", "pl-4", "bg-yellow-900", "pt-2", "rounded", "text-2xl", "text-center", "text-yellow-100")}>{"Output"}</p>
                <p id="output" class={classes!("font-mono", "bg-white", "w-full", "h-full", "p-2", "text-ml", "whitespace-pre")}>{(*output).clone()}</p>
            </div>
        </div>
    }
}
