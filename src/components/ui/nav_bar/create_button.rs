use crate::components::button::*;
use crate::utils::icons::*;
use yew::prelude::*;

#[function_component(CreateButton)]
pub fn create_button() -> Html {
    let displaying = use_state(|| false);
    let onclick = {
        let displaying = displaying.clone();
        Callback::from(move |_| displaying.set(!*displaying))
    };
    html! {
        <div class="relative">
            <Button {onclick}>
                <span class="text-md text-skin-typography font-medium">
                    {"Tạo mới"}
                </span>

                <span class="border-s-2 h-5 border-skin-typography my-auto"></span>

                <CaretDownFill class="fill-skin-typography" />
            </Button>

            if *displaying {
                <div class="absolute top-full w-full shadow-lg shadow-skin-buttons bg-skin-buttons">
                    <div class="flex flex-col items-center justify-start">
                        <button class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                            <PlusSquare class="stroke-skin-typography"/>
                            {"Tạo mới"}
                        </button>
                        <button class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                            <CloudArrowUp class="stroke-skin-typography"/>
                            {"Tải lên"}
                        </button>
                    </div>
                </div>
            }
        </div>
    }
}
