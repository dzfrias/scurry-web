mod run_button;

use run_button::RunButton;
use yew::prelude::*;

#[function_component]
pub fn Editor() -> Html {
    let output = use_state_eq(|| "".to_owned());
    let set_output = {
        let output = output.clone();
        move |out: String| output.set(out)
    };

    html! {
        <div class={classes!("mx-4")}>
            <ul class={classes!("py-1", "pl-4", "bg-teal-100", "pt-2", "rounded")}>
                <RunButton {set_output}/>
            </ul>
            <textarea id="editor" rows=20 spellcheck="false" placeholder="Enter code here" style="resize: None" class={classes!("w-full", "font-mono", "text-ml", "p-2")}></textarea>
            <p id="output" style="white-space: pre-line;" class={classes!("font-mono")}>{(*output).clone()}</p>
        </div>
    }
}
