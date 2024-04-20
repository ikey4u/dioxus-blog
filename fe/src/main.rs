#![allow(non_snake_case)]

use dioxus::prelude::*;

const _STYLE: &str = manganis::mg!(file("public/tailwind.css"));

fn main() {
    launch(App);
}

pub fn App() -> Element {
    rsx!(
        div {
            class: "flex justify-center items-center h-screen",
            p {
                class: "text-center text-3xl text-blue-600 font-bold",
                "Hello Dioxus!"
            }
        }
    )
}
