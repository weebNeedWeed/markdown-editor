use crate::utils::icons::CaretDownFill;
use yew::prelude::*;

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    html! {
        <nav class="p-4 text-skin-typography">
            <div class="flex flex-row w-full items-center justify-between">
                <div class="flex flex-row items-center">
                    <div class="bg-skin-buttons rounded-md flex flex-row justify-start py-2.5 font-semibold text-md items-center">
                        <button class="h-full px-2">
                            {"Tạo mới"}
                        </button>

                        <span class="border-s-2 h-5 border-skin-typography"></span>

                        <button class="px-2">
                            <CaretDownFill class="fill-skin-typography" />
                        </button>
                    </div>
                </div>
            </div>
        </nav>
    }
}
