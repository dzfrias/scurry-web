use yew::prelude::*;
use yew_icons::{Icon, IconId};

#[derive(Debug, PartialEq, Properties)]
pub struct Props {
    pub name: AttrValue,
    pub target: AttrValue,
    pub icon_id: IconId,
}

#[function_component]
pub fn BarItem(props: &Props) -> Html {
    html! {
        <a target="_blank" href={props.target.clone()} class={classes!("hover:text-amber-700", "flex-row", "flex", "items-center", "gap-2")}><Icon icon_id={props.icon_id}/><span class={classes!("hidden", "sm:inline")}>{props.name.clone()}</span></a>
    }
}
