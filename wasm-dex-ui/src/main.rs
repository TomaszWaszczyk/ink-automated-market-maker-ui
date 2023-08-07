#![allow(non_snake_case)]
// get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use log::LevelFilter;

fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Failed to initialize logger");
    console_error_panic_hook::set_once();

    log::info!("Starting WebAssembly Decentralized Exchange");
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
