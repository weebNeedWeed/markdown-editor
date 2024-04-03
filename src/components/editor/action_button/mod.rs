use yew::prelude::*;

pub mod bold;
pub mod heading;
pub mod italic;
pub mod redo;
pub mod undo;

#[derive(Properties, Clone, PartialEq)]
pub struct ActionButtonProps {
    pub icon: Html,
    pub onclick: Callback<MouseEvent>,
    pub title: AttrValue,
}

#[function_component(ActionButton)]
pub fn action_button(props: &ActionButtonProps) -> Html {
    let ActionButtonProps {
        icon,
        onclick,
        title,
    } = props;
    let handle_click = {
        let onclick = onclick.clone();
        Callback::from(move |e| {
            onclick.emit(e);
        })
    };
    html! {
        <div class="relative group">
            <button
                onclick={handle_click}
                class="active:scale-75 transition-all hover:bg-skin-secondary px-2 py-1 rounded-md">
                {icon.clone()}
            </button>
            <span
                class="absolute hidden group-hover:block top-full left-1/2 -translate-x-1/2 bg-skin-buttons rounded-md px-2 py-1 text-sm text-white">
                {title}
            </span>
        </div>

    }
}
