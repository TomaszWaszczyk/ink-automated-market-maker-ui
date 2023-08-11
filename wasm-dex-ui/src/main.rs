#![allow(non_snake_case)]
// get access to the `rsx!` macro and the `Scope` and `Element` types
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use log::LevelFilter;

#[derive(PartialEq)]
pub enum ExchangeState {
    Started,
    InProgress,
    Completed,
}


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
    let token_a = use_state(cx, || "token_a".to_string());
    let token_b = use_state(cx, || "token_b".to_string());
    let test = use_ref(cx, String::new);
    let swap_id = use_state(cx, || 0);

    cx.render(rsx! {
        h1 { "Index" },
        div {
            background_color: "#d2d0d2" 
        }

        div {
            class: "flex flex-col h-screen items-center",
            a {
                "test"
            }
        }

        input {
            value: "{ token_a }",
            oninput: move |event| token_a.set(event.value.clone()),
        }

        input {
            value: "{ token_b }",
            oninput: move |event| token_b.set(event.value.clone()),
        }

    })
}
