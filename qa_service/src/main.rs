use std::io::{Error, ErrorKind};
use std::str::FromStr;
use warp::Filter;

#[derive(Debug)]
struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>,
}

#[derive(Debug)]
struct QuestionId(String);

impl Question {
    fn new(id: QuestionId, title: String, content: String, tags: Option<Vec<String>>) -> Self {
        Question {
            id: id,
            title: title,
            content: content,
            tags: tags,
        }
    }
}

impl FromStr for QuestionId {
    type Err = std::io::Error;

    fn from_str(id: &str) -> Result<Self, Self::Err> {
        match id.is_empty() {
            false => Ok(QuestionId(id.to_string())),
            true => Err(Error::new(ErrorKind::InvalidInput, "No id provided")),
        }
    }
}

// fn main() {
//     let question = Question::new(
//         QuestionId::from_str("").expect("No id provided"),
//         "First Question".to_string(),
//         "Content of question".to_string(),
//         Some(vec!["faq".to_string()]),
//     );
//     println!("{:?}", question);
// }

#[tokio::main]
async fn main() {
    let hello = warp::get().map(|| format!("Hello, world"));
    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
