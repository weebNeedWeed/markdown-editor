use yew::AttrValue;

use super::markdown_context::Markdown;

pub trait Memento<T> {
    fn restore(self) -> T;
}

#[derive(Clone, Debug)]
pub struct MarkdownBackup {
    pub key: Option<AttrValue>,
    pub text: AttrValue,
}

impl MarkdownBackup {
    pub fn new(key: Option<AttrValue>, text: AttrValue) -> Self {
        Self { key, text }
    }
}

impl Memento<Markdown> for MarkdownBackup {
    fn restore(self) -> Markdown {
        Markdown::new(self.key, self.text)
    }
}

#[derive(Clone, Debug)]
pub struct Caretaker {
    size: usize,
    pub backups: Vec<MarkdownBackup>,
}

impl Caretaker {
    pub fn new(size: usize) -> Self {
        Self {
            backups: vec![],
            size,
        }
    }

    pub fn last(&mut self) -> MarkdownBackup {
        self.backups.pop().unwrap()
    }

    pub fn add(&mut self, backup: MarkdownBackup) {
        if self.backups.len() > self.size {
            self.backups.remove(0);
        }

        self.backups.push(backup);
        gloo::console::log!("{:?}", self.backups.len())
    }
}
