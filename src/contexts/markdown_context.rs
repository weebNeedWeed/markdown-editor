use std::collections::HashMap;
use std::ops::Deref;

use gloo::storage::{LocalStorage, Storage};
use serde_json::Value;
use yew::prelude::*;
use yew::ContextProvider;
use yew::{use_state, AttrValue, UseStateHandle};

#[derive(Clone, PartialEq, Debug)]
pub struct Markdown {
    pub key: Option<AttrValue>,
    pub text: AttrValue,
}

impl Default for Markdown {
    fn default() -> Self {
        let text = AttrValue::from("# Hello world");
        let key = Some(AttrValue::from("new_file.md"));
        Self { text, key }
    }
}

impl Markdown {
    pub fn new(key: Option<AttrValue>, text: AttrValue) -> Self {
        Self { key, text }
    }

    pub fn fetch_all_markdown_keys() -> Vec<AttrValue> {
        let mut md_keys = vec![];
        let records: HashMap<String, Value> = LocalStorage::get_all().unwrap();
        records.iter().for_each(|item| {
            if item.0.ne("config") {
                md_keys.push(AttrValue::from(item.0.clone()));
            }
        });
        md_keys
    }

    pub fn load_from_storage(key: AttrValue) -> Option<Markdown> {
        let text = match LocalStorage::get::<String>(key.to_string()) {
            Ok(v) => v,
            Err(_) => return None,
        };
        let key = Some(key);
        Some(Self::new(key, text.into()))
    }

    pub fn load_latest_from_storage() -> Option<Markdown> {
        let files = Self::fetch_all_markdown_keys();
        match files.last() {
            Some(file) => Self::load_from_storage(file.into()),
            None => None,
        }
    }

    pub fn save_to_storage(&self) -> Result<(), &'static str> {
        let key = match self.key.as_ref() {
            Some(k) => k,
            None => return Err("Key must be found"),
        };
        let key_str = key.as_str();
        match LocalStorage::set(key_str, self.text.as_str()) {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string().leak()),
        }
    }
}

impl MarkdownContext {
    pub fn new(inner: UseStateHandle<Markdown>) -> Self {
        Self { inner }
    }

    pub fn add_markdown(&self, md: Markdown) -> Result<(), &'static str> {
        md.save_to_storage()?;
        self.inner.set(md);
        Ok(())
    }

    pub fn update_markdown(&self, md: Markdown) -> Result<(), &'static str> {
        self.inner.set(md.clone());
        md.save_to_storage()?;
        Ok(())
    }

    pub fn state(&self) -> Markdown {
        (*self.inner).clone()
    }
}

impl Deref for MarkdownContext {
    type Target = Markdown;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl PartialEq for MarkdownContext {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

#[derive(Clone, Debug)]
pub struct MarkdownContext {
    inner: UseStateHandle<Markdown>,
}

#[yew_autoprops::autoprops]
#[function_component(MarkdownContextProvider)]
pub fn markdown_context_provider(children: &Html) -> Html {
    let markdown = Markdown::load_latest_from_storage().unwrap_or_default();
    let state = use_state(|| markdown);
    let markdown_ctx = MarkdownContext::new(state);
    html! {
        <ContextProvider<MarkdownContext> context={markdown_ctx}>
            {children.clone()}
        </ContextProvider<MarkdownContext>>
    }
}

#[hook]
pub fn use_markdown() -> MarkdownContext {
    use_context::<MarkdownContext>().expect("Markdown context must be found")
}
