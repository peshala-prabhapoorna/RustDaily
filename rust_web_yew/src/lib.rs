use gloo::console::log;
use serde::{Deserialize, Serialize};
use stylist::{style, yew::styled_component, Style};
use yew::prelude::*;

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[styled_component(App)]
pub fn app() -> Html {
    let name: &str = "Choppar";
    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("my name is {}", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());

    let class: &str = "my_title";
    let message: Option<&str> = None;
    let tasks = vec!["record video", "grocerry shopping", "pet choppar"];

    let stylesheet = style!(
        r#" 
            h1 {
                color: teal 
            }
            p {
                color: lightgreen
            }
            ul {
                color: aqua
            }
        "#
    )
    .unwrap();

    html! {
        <div class={stylesheet}>
            <h1 class={class}>{"Hello World!"}</h1>
            if class == "my_title" {
                <p>{"Hey there!"}</p>
            } else {
                <p>{"I am not a title"}</p>
            }
            if let Some(message) = message {
                <p>{message}</p>
            } else {
                <p>{"No messages today"}</p>
            }
            <ul>
                {list_to_html(tasks)}
            </ul>
        </div>
    }
}

fn list_to_html(list: Vec<&str>) -> Vec<Html> {
    list.iter()
        .map(|item| html! {<li>{format!("{} - ☑",item)}</li>})
        .collect()
}
