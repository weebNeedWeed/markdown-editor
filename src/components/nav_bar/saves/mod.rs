use crate::components::button::*;
use crate::contexts::markdown_context::use_markdown;
use crate::utils::icons::*;
use save_markdown::SaveMarkdown;
use yew::prelude::*;

pub mod save_markdown;

#[derive(Properties, Clone, PartialEq)]
pub struct SaveDropdownProps {
    pub open: bool,
}

#[function_component(SaveDropdown)]
pub fn save_dropdown(SaveDropdownProps { open }: &SaveDropdownProps) -> Html {
    let displaying = use_state(|| false);
    {
        let displaying = displaying.clone();
        use_effect_with(*open, move |open| {
            if !open {
                displaying.set(false);
            }
        });
    }
    let onclick = {
        let displaying = displaying.clone();
        Callback::from(move |_| displaying.set(!*displaying))
    };

    let md_state = use_markdown().state();
    let encoded_md = urlencoding::encode(md_state.text.as_str());
    let text_dl = format!("data:text/markdown,{}", encoded_md);
    let download_name = md_state.key.unwrap();

    html! {
        <div class="relative">
            <Button {onclick}>
                <span class="text-md text-skin-typography font-medium">
                    {"Lưu"}
                </span>

                <span class="border-s-2 h-5 border-skin-typography my-auto"></span>

                <CaretDownFill class="fill-skin-typography" />
            </Button>

            if *displaying {
                <div class="absolute z-10 top-full w-full shadow shadow-skin-buttons bg-skin-buttons">
                    <div class="flex flex-col items-center justify-start">
                        <SaveMarkdown />
                        <button class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                            <FiletypePdf class="stroke-skin-typography"/>
                            {".pdf"}
                        </button>
                    </div>
                </div>
            }
            <a id="dl" class="hidden" href={text_dl} download={download_name} target="_blank"></a>
        </div>
    }
}
