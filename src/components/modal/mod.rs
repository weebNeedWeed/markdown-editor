use crate::utils::icons::Close;
use modal_button::ModalButton;
use yew::prelude::*;

pub mod modal_button;

#[derive(Properties, PartialEq, Clone)]
pub struct ModalProps {
    pub title: AttrValue,
    pub open: bool,
    pub onclose: Callback<MouseEvent>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Modal)]
pub fn modal(props: &ModalProps) -> Html {
    html! {
        <div class={classes!(
            AttrValue::from("w-screen h-screen top-0 left-0 fixed z-40 bg-black/50 flex items-center justify-center"),
            if props.open { "" } else {"hidden"})}>
            <div class="w-1/3 bg-white p-6 rounded-md border-skin-buttons border-4 h-[600px]
                flex flex-col shadow-md shadow-slate-600">
                <div class="flex w-full justify-between">
                    <h3 class="text-2xl text-skin-buttons font-semibold">
                        {props.title.clone()}
                    </h3>
                    <button onclick={props.onclose.clone()} class="transition-all active:scale-75">
                        <Close class="fill-skin-buttons stoke-2 w-5 h-5"/>
                    </button>
                </div>

                <div class="w-full grow overflow-auto">
                    {props.children.clone()}
                </div>

                <div class="shrink-0 grow-0 flex items-center justify-end">
                    <ModalButton onclick={props.onclose.clone()} title={"Đóng"}/>
                </div>
            </div>
        </div>
    }
}
