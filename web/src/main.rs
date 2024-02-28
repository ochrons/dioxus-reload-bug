#![allow(non_snake_case, unused)]
use dioxus::prelude::*;
use dioxus_fullstack::prelude::*;

fn main() {
    launch(App);
}

fn App() -> Element {
    rsx! {
        div { class: "flex flex-col justify-center overflow-hidden py-6",
            div { class: "mx-auto max-w-2xl", "Just testingggg" }
        }
    }
}
