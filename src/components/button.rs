use yew::prelude::*;

#[derive(PartialEq, Clone, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(Button)]
pub fn button(
    ButtonProps {
        children,
        class,
        onclick,
    }: &ButtonProps,
) -> Html {
    let onclick = {
        let onclick = onclick.clone();
        Callback::from(move |e: MouseEvent| onclick.emit(e))
    };

    html! {
        <button onclick={onclick} class={classes!(String::from("p-2.5 gap-x-2 shadow shadow-skin-buttons bg-skin-buttons transition-all hover:opacity-80 rounded flex justify-start items-center active:scale-90"),class.clone())}>
            {children.clone()}
        </button>
    }
}
