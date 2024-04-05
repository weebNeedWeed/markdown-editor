use crate::components::button::*;
use crate::utils::icons::*;
use save_html::SaveHtml;
use save_markdown::SaveMarkdown;
use yew::prelude::*;

pub mod save_html;
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
                        <SaveHtml />
                    </div>
                </div>
            }
        </div>
    }
}
