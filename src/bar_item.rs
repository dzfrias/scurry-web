use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, PartialEq, Eq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub target: AttrValue,
    pub icon_id: IconId,
}

#[function_component]
pub fn BarItem(props: &Props) -> Html {
    html! {
        <a target="_blank" href={props.target.clone()} class={classes!("hover:text-amber-700", "flex-row", "flex", "items-center", "gap-2", "text-amber-900", "font-main")}><Icon icon_id={props.icon_id}/><span class={classes!("hidden", "sm:inline", "text-lg")}>{props.name.clone()}</span></a>
    }
}
