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
        <div class={classes!("mx-4", "grid", "md:grid-cols-2", "gap-2", "h-2/3")}>
            <div class={classes!("h-full")}>
                <ul class={classes!("py-1", "pl-4", "bg-yellow-900", "pt-2", "rounded")}>
                    <RunButton {set_output}/>
                </ul>
                <textarea id="editor" spellcheck="false" placeholder="Enter code here" style="resize: None" class={classes!("w-full", "font-mono", "text-ml", "p-2", "h-full", "focus:outline-none")}></textarea>
            </div>
            <div class={classes!("h-full")}>
                <p class={classes!("py-1", "pl-4", "bg-yellow-900", "pt-2", "rounded", "text-2xl", "text-center", "text-yellow-100")}>{"Output"}</p>
                <p id="output" class={classes!("font-mono", "bg-white", "w-full", "h-full", "p-2", "text-ml", "whitespace-pre")}>{(*output).clone()}</p>
            </div>
        </div>
    }
}
