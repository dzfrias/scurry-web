use scurry::interpreter::Interpreter;
use scurry::parser::Parser;
use std::io::Write;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{self, HtmlTextAreaElement};
use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug)]
pub struct Output {
    vector: Vec<u8>,
    state: UseStateHandle<String>,
}

impl Write for Output {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let written = self.vector.write(buf);
        self.state.set(
            String::from_utf8(self.vector.clone())
                .expect_throw("program should output valid utf-8"),
        );
        written
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.vector.flush()?;
        self.state.set(
            String::from_utf8(self.vector.clone())
                .expect_throw("program should output valid utf-8"),
        );
        Ok(())
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub output: UseStateHandle<String>,
}

#[function_component]
pub fn RunButton(props: &Props) -> Html {
    let output = props.output.clone();
    let onclick = Callback::from(move |_| {
        let editor = web_sys::window()
            .expect_throw("window should be available")
            .document()
            .expect_throw("document should be available")
            .get_element_by_id("editor")
            .expect_throw("editor should exist")
            .dyn_into::<HtmlTextAreaElement>()
            .expect_throw("element should be a textarea");
        let mut out = Output {
            vector: Vec::new(),
            state: output.clone(),
        };
        let mut interpreter = Interpreter::new(&mut out);
        match Parser::new(&editor.value()).parse() {
            Ok(program) => {
                if let Err(err) = interpreter.eval(program) {
                    output.set(format!("runtime error:\n{err}"))
                }
            }
            Err(errs) => output.set(format!(
                "parser errors:{}",
                errs.into_iter()
                    .map(|err| {
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
        <button {onclick} id="runButton" class={
            classes!("bg-orange-700",
                     "px-3",
                     "rounded",
                     "text-2xl",
                     "hover:bg-yellow-400",
                     "text-yellow-100",
                     "hover:text-amber-900",
                     "flex-row",
                     "flex",
                     "items-center",
                     "gap-1",
                     "font-main")}>
            {"Run"}<Icon icon_id={IconId::FontAwesomeSolidChevronRight} width={"23px".to_owned()} height={"23px".to_owned()}/>
        </button>
    }
}
