#![feature(rustc_private)]

use serde_derive::Deserialize;
use std::collections::HashMap;
use std::env;
use std::fs;

#[derive(Deserialize)]
struct Answer {
    text: String,
    target: String,
}

#[derive(Deserialize)]
struct Question {
    text: String,
    answers: Option<Vec<Answer>>,
}

#[derive(Deserialize)]
struct Config {
    questions: HashMap<String, Question>,
}

fn generate_options(question: &Question) -> String {
    match &question.answers {
        Some(answers) => answers
            .iter()
            .map(|answer| {
                format!(
                    "<a href=\"{}\">{}</a>",
                    format!("{}.html", answer.target),
                    answer.text,
                )
            })
            .collect::<Vec<String>>()
            .join(" "),
        None => String::from(""),
    }
}

fn generate_page(name: String, question: &Question) {
    let text = format!(
        "
        <html>
            <body>
                <h1>{}</h1>
                {}
            </body>
        </html>
    ",
        question.text,
        generate_options(question)
    );
    let filename = format!("{}.html", name);

    fs::write(format!("build/{}", filename), text).expect("Unable to write file");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config_name = &args[1];
    let config_text = fs::read_to_string(config_name).unwrap();
    let config: Config = toml::from_str(config_text.as_str()).unwrap();

    fs::create_dir_all("build").expect("Unable to create build directory");

    for (name, question) in config.questions {
        generate_page(name, &question);
    }
}
