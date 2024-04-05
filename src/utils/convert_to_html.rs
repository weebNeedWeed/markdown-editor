use markdown::{to_html_with_options, CompileOptions, Options, ParseOptions};

pub fn convert_to_html(md: &str) -> String {
    let options = Options {
        parse: ParseOptions::gfm(),
        compile: CompileOptions {
            allow_dangerous_html: true,
            allow_dangerous_protocol: true,
            ..CompileOptions::default()
        },
    };

    to_html_with_options(md, &options).unwrap()
}
