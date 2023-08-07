#![allow(non_snake_case)]
// get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;

fn main() {
    dioxus_web::launch(App);
}

// pub enum Route {
//     #[layout(HeaderFooter)]
// }

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        div {
            background_color: "#d2d0d2" 
        }

        "Hello, wasm dex!"
    })
}
