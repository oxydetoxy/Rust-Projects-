use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct Paragraph {
    name: String,
}
#[derive(Serialize, Deserialize)]
struct User {
    article: String,
    author: String,
    paragraph: Vec<Paragraph>,
}
fn main() {
    let json = r#"{
        "article": "none of the above ",
        "author": "ishant",
        "paragraph":[
            {
                "name":"ishuu"
            },{
                "name":"chaudhary"
            }
        ]

        
            }"#;
    let parsed: User = read_json_typed(json);
    println!("the data is {}", parsed.paragraph[1].name);
}
fn read_json_typed(raw: &str) -> User {
    let parsed: User = serde_json::from_str(raw).unwrap();
    return parsed;
}
