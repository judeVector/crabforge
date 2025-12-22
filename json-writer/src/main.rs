use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}

#[derive(Serialize, Deserialize)]
struct Article {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}

fn main() {
    let article = Article {
        article: String::from("How to work with Rust"),
        author: String::from("Vector"),
        paragraph: vec![
            Paragraph {
                name: String::from("first paragraph"),
            },
            Paragraph {
                name: String::from("second paragraph"),
            },
            Paragraph {
                name: String::from("footer and last paragraph"),
            },
        ],
    };

    let json = serde_json::to_string(&article).unwrap();
    println!("{}", json)
}
