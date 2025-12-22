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

fn read_json_type(raw_json: &str) -> Article {
    let result: Article = serde_json::from_str(&raw_json).unwrap();

    result
}

fn main() {
    let json = r#"
    {
        "article": "How to code in rust",
        "author": "Vector",
        "paragraph": [
            {
                "name": "The first paragraph"
            },
            {
                "name": "The second page"
            },
            {
                "name": "The footer and closing"
            }
        ]
        }"#;

    let parsed = read_json_type(json);
    println!(
        "The name of the first paragraph is: {}",
        parsed.paragraph[0].name
    );
}
