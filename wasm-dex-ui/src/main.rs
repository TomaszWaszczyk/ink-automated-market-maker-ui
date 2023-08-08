#![allow(non_snake_case)]
// get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;


fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("Failed to initialize logger");
    console_error_panic_hook::set_once();

    log::info!("Starting WebAssembly Decentralized Exchange");
    dioxus_web::launch(App);
}

#[inline_props]
fn Wrapper(cx: Scope) -> Element {
    render! {
        header { "header" }
        // The index route will be rendered here
        Outlet::<Route> { }
        footer { "footer" }
    }
}


#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Index {},
}

#[inline_props]
fn Index(cx: Scope) -> Element {
    render! {
        h1 { "Index" }
    }
}

fn App(cx: Scope) -> Element {
    cx.render(rsx! {
        h1 { "Index" },
        div {
            background_color: "#d2d0d2" 
        }

        "Hello, wasm dex!"
    })
}
