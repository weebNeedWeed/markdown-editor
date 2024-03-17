use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct ModalButtonProps {
    #[prop_or_default]
    pub classes: AttrValue,
    pub title: AttrValue,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(ModalButton)]
pub fn modal_button(props: &ModalButtonProps) -> Html {
    html! {
        <button onclick={props.onclick.clone()} class={classes!(
            AttrValue::from("rounded-md px-3 py-2 bg-neutral-500/50 font-semibold ransition-all active:scale-90"),
            props.classes.clone())}>
            {props.title.clone()}
        </button>
    }
}
