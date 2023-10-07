use gloo::console::log;
use yew::prelude::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct MyObject {
    username: String,
    favorite_language: String,
}

#[function_component(App)]
pub fn app() -> Html {
    let name: &str = "Choppar";
    let my_object = MyObject {
        username: name.to_owned(),
        favorite_language: "Rust".to_owned(),
    };

    log!("my name is {}", name);
    log!(serde_json::to_string_pretty(&my_object).unwrap());
    html! {
        <h1>{"Hello World!"}</h1>
    }
}