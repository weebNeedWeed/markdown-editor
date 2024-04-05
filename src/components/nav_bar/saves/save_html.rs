use crate::{contexts::markdown_context::use_markdown, utils::icons::FiletypeHtml};
use gloo::net::http::Request;
use js_sys::wasm_bindgen::JsCast;
use web_sys::{HtmlAnchorElement, HtmlDocument};
use yew::prelude::*;

#[function_component(SaveHtml)]
pub fn save_html() -> Html {
    let styles = use_state(|| AttrValue::from(""));
    {
        let styles = styles.clone();
        use_effect_with((), move |_| {
            let document = gloo::utils::document();
            let document: HtmlDocument = document.dyn_into().unwrap();

            let sheet = document.style_sheets();
            let sheet = sheet.get(0).unwrap();
            let sheet_url = sheet.href().unwrap().unwrap();

            wasm_bindgen_futures::spawn_local(async move {
                let css = Request::get(&sheet_url)
                    .send()
                    .await
                    .unwrap()
                    .text()
                    .await
                    .unwrap();
                // let re = Regex::new(r#"(?ms)^\s*\.[^{]*?prose[^{]*?\s*\{[\s\S]*?\}"#).unwrap();
                // let mut results = vec![];
                // for (full, []) in re.captures_iter(&css).map(|c| c.extract()) {
                //    results.push(full);
                // }
                styles.set(css.into());
            });
        });
    }

    let handle_download = {
        let styles = styles.clone();
        Callback::from(move |_| {
            let anchor = gloo::utils::document()
                .get_element_by_id("dl_html")
                .unwrap()
                .unchecked_into::<HtmlAnchorElement>();
            let html = gloo::utils::document()
                .get_element_by_id("preview_box")
                .unwrap()
                .outer_html();
            let html = format!(
                "<!DOCTYPE html>\n<html>\n<head>\n<style>\n{}\n</style>\n</head>\n<body>\n{}\n</body>\n</html>",
                *styles,
                html
            );
            let encoded_html = urlencoding::encode(&html);
            let text_dl = format!("data:text/html,{}", encoded_html);
            anchor.set_href(&text_dl);
            anchor.click();
        })
    };

    let md_state = use_markdown().state();
    let download_name = format!(
        "{}.html",
        md_state
            .key
            .unwrap()
            .split(".")
            .collect::<Vec<&str>>()
            .get(0)
            .unwrap()
    );

    html! {
        <>
            <button onclick={handle_download} class="w-full gap-x-2 py-2 transition-all justify-center hover:opacity-80 flex items-center">
                <FiletypeHtml class="stroke-skin-typography"/>
                {".html"}
            </button>
            <a id="dl_html" class="hidden" download={download_name} target="_blank"></a>
        </>
    }
}
