use scurry::interpreter::Interpreter;
use scurry::parser::Parser;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{self, HtmlTextAreaElement};
use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    pub set_output: Callback<String>,
}

#[function_component]
pub fn RunButton(props: &Props) -> Html {
    let set_output = props.set_output.clone();
    let onclick = Callback::from(move |_| {
        let editor = web_sys::window()
            .expect_throw("window should be available")
            .document()
            .expect_throw("document should be available")
            .get_element_by_id("editor")
            .expect_throw("editor should exist")
            .dyn_into::<HtmlTextAreaElement>()
            .expect_throw("element should be a textarea");
        let mut out = Vec::new();
        let mut interpreter = Interpreter::new(&mut out);
        match Parser::new(&editor.value()).parse() {
            Ok(program) => {
                interpreter
                    .eval(program)
                    // FIX: Runtime errors
                    .expect_throw("should have no errors");
                set_output
                    .emit(String::from_utf8(out).expect_throw("program should output valid utf-8"));
            }
            Err(errs) => set_output.emit(format!(
                "parser errors:{}",
                errs.into_iter()
                    .map(|err| {
                        web_sys::console::log_1(
                            &err.position().format_on_source(&editor.value()).into(),
                        );
                        format!(
                            "\n{}\n{}\n",
                            err.position().format_on_source(&editor.value()),
                            err
                        )
                    })
                    .collect::<String>()
            )),
        }
    });

    html! {
        <button {onclick} class={classes!("bg-orange-700", "px-3", "rounded", "text-2xl", "hover:bg-yellow-400", "text-yellow-100", "hover:text-amber-900")}>{"Run"}</button>
    }
}
