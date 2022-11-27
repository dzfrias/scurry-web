use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub code: AttrValue,
}

#[function_component]
pub fn Example(props: &Props) -> Html {
    // TODO: Copy on click
    html! {
        <div class={classes!("self-center", "mr-8", "rounded-lg", "bg-gray-300", "[&_code]:hover:blur-[1px]", "[&_p]:hover:visible", "flex", "items-center", "justify-center", "hidden", "md:flex", "drop-shadow-md")}>
            <code class={classes!("whitespace-pre", "font-mono", "p-4")}>{props.code.clone()}</code>
            <p class={classes!("invisible", "blur-none", "absolute", "text-amber-700", "text-3xl", "font-['Maven_Pro']", "font-bold")}>{"Copy"}</p>
        </div>
    }
}
