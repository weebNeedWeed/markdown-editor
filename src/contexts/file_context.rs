use yew::prelude::*;
use yew::ContextProvider;

pub enum FileAction {}

#[derive(Clone, PartialEq)]
pub struct FileState {
    pub file_name: String,
}

impl FileState {
    fn default() -> Self {
        Self {
            file_name: String::default(),
        }
    }
}

impl Reducible for FileState {
    type Action = FileAction;

    fn reduce(self: std::rc::Rc<Self>, action: Self::Action) -> std::rc::Rc<Self> {
        match action {
            _ => self,
        }
    }
}

type FileContext = UseReducerHandle<FileState>;

#[yew_autoprops::autoprops]
#[function_component(FileContextProvider)]
pub fn file_context_provider(children: &Html) -> Html {
    let reducer = use_reducer(FileState::default);
    html! {
        <ContextProvider<FileContext> context={reducer}>
            {children.clone()}
        </ContextProvider<FileContext>>
    }
}

#[hook]
pub fn use_file_context() -> FileContext {
    use_context::<FileContext>().expect("FileContext must be found")
}
