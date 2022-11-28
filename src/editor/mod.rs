mod run_button;

use run_button::RunButton;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{HtmlElement, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq)]
enum Modifier {
    Alt,
    Meta,
    Ctrl,
    Shift,
}

#[derive(Debug, PartialEq, Eq)]
struct KeyPress {
    key_name: String,
    modifiers: Vec<Modifier>,
    underlying: KeyboardEvent,
}

impl From<KeyboardEvent> for KeyPress {
    fn from(key: KeyboardEvent) -> Self {
        let mut modifiers = Vec::new();
        if key.ctrl_key() {
            modifiers.push(Modifier::Ctrl);
        }
        if key.meta_key() {
            modifiers.push(Modifier::Meta);
        }
        if key.shift_key() {
            modifiers.push(Modifier::Shift);
        }
        if key.alt_key() {
            modifiers.push(Modifier::Alt);
        }
        KeyPress {
            key_name: key.key(),
            underlying: key,
            modifiers,
        }
    }
}

#[function_component]
pub fn Editor() -> Html {
    let output = use_state_eq(String::new);
    let set_output = {
        let output = output.clone();
        move |out: String| output.set(out)
    };
    let onkeydown = {
        move |key: KeyboardEvent| {
            let key: KeyPress = key.into();
            if key.key_name == *"Tab" && key.modifiers == Vec::new() {
                key.underlying.prevent_default();
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
            } else if key.key_name == *"Enter" && key.modifiers == vec![Modifier::Meta] {
                web_sys::window()
                    .expect_throw("window should be available")
                    .document()
                    .expect_throw("document should be available")
                    .get_element_by_id("runButton")
                    .expect_throw("run button should exist")
                    .dyn_into::<HtmlElement>()
                    .expect_throw("element should be an html element")
                    .click();
            }
        }
    };

    html! {
        <div class={classes!("mr-8", "flex", "gap-2", "flex-col", "md:flex-row", "h-full")}>
            <div class={classes!("md:w-1/2", "cursor-col-resize", "h-full")}>
                <ul class={classes!("py-1", "pl-4", "bg-gray-800", "pt-2", "rounded")}>
                    <RunButton {set_output}/>
                    // TODO: Toggle REPL button
                </ul>
                // TODO: Autopairs
                <textarea id="editor" autocorrect="off" autocapitalize="none" {onkeydown} spellcheck="false" placeholder="println(\"Hello, world!\");" class={classes!("w-full", "font-mono", "text-ml", "p-2", "focus:outline-none", "overflow-y-auto", "h-2/3", "resize-none", "bg-gray-300", "focus:bg-gray-100")}></textarea>
            </div>
            <div class={classes!("md:w-1/2", "overflow-y-auto", "h-full")}>
                <p class={classes!("py-1", "pl-4", "bg-gray-800", "pt-2", "rounded", "text-2xl", "text-center", "text-yellow-100", "overflow-y-clip", "font-main")}>{"Output"}</p>
                <p id="output" class={classes!("font-mono", "w-full", "p-2", "text-ml", "whitespace-pre", "overflow-y-scroll", "h-2/3", "bg-gray-300", "active:bg-gray-100")}>{&(*output)}</p>
            </div>
        </div>
    }
}
