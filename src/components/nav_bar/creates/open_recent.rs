use crate::components::modal::Modal;
use crate::contexts::markdown_context::{use_markdown, Markdown};
use crate::utils::icons::ClockHistory;
use yew::prelude::*;

#[function_component(OpenRecent)]
pub fn open_recent() -> Html {
    let display_modal = use_state(|| false);
    let handle_toggle_modal = {
        let display_modal = display_modal.clone();
        Callback::from(move |_| display_modal.set(!*display_modal))
    };

    let markdown = use_markdown();
    let make_handle_load_file_fn = {
        let markdown = markdown.clone();
        let display_modal = display_modal.clone();
        move |file_key: &AttrValue| {
            let file_key = file_key.clone();
            Callback::from(move |_| {
                let md = Markdown::load_from_storage(file_key.clone()).unwrap();
                display_modal.set(false);
                markdown.update_markdown(md).unwrap()
            })
        }
    };
    let recent_open_list = Markdown::fetch_all_markdown_keys().iter().map(|file_key| html! {
        <button onclick={make_handle_load_file_fn.clone()(file_key)}
            key={file_key.to_string()} 
            class="w-full text-left bg-neutral-400/50 p-2 font-semibold hover:bg-skin-buttons rounded-md transition-all">
            {file_key}
        </button>
    }).collect::<Html>();

    html! {
        <>
            <button onclick={handle_toggle_modal.clone()} class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                <ClockHistory class="stroke-skin-typography"/>
                {"Gần đây"}
            </button>
            <Modal title={"Gần đây"} open={*display_modal} onclose={handle_toggle_modal}>
                <div class="flex flex-col h-full overflow-auto py-2 gap-y-2">
                    { recent_open_list }
                </div>
            </Modal>
        </>
    }
}
