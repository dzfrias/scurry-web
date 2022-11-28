use wasm_bindgen::prelude::*;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlElement;
use yew::prelude::*;
use yew_icons::{Icon, IconId};

// Copy text to clipboard. Rust way of doing this is currently unstable
#[wasm_bindgen(inline_js = r#"
export function copy_to_clipboard(value) {
   window.navigator.clipboard.writeText(value);
}
"#)]
extern "C" {
    fn copy_to_clipboard(value: String);
}

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub code: AttrValue,
}

#[function_component]
pub fn Example(props: &Props) -> Html {
    let onclick = Callback::from(move |event: MouseEvent| {
        let elem: HtmlElement = event
            .target()
            .expect_throw("should have a target")
            .dyn_into()
            .expect_throw("should be an html element");
        let text = elem.inner_text();
        // This means they clicked the paragraph element
        if text == "Copy" {
            let previous: HtmlElement = elem
                .previous_sibling()
                .unwrap_throw()
                .dyn_into()
                .expect_throw("previous sibling should be an html element");
            copy_to_clipboard(previous.inner_text())
        } else {
            copy_to_clipboard(elem.inner_text())
        }
    });

    html! {
        <div {onclick} class={classes!("self-center", "mr-8", "rounded-xl", "bg-gray-300", "flex", "items-center", "justify-center", "hidden", "md:flex", "drop-shadow-md", "cursor-pointer", "group")}>
            <div class={classes!("brightness-50", "group-hover:bg-gray-300", "p-4", "rounded-xl")}>
                <code class={classes!("whitespace-pre", "font-mono", "select-none", "group-hover:blur-[1px]")}>{props.code.clone()}</code>
            </div>
            <p class={classes!("invisible", "blur-none", "absolute", "text-amber-600", "text-3xl", "font-main", "font-bold", "flex-row", "flex", "items-center", "gap-2", "select-none", "group-hover:visible", "group-active:text-yellow-100", "group-active:text-4xl")}><Icon icon_id={IconId::FontAwesomeRegularCopy}/>{"Copy"}</p>
        </div>
    }
}
