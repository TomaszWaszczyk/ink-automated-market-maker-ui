pub fn About(cx: Scope) -> Element {
    cx.render(rsx!(
        p {
            b { "Dioxus Labs" }
            " An Open Source project dedicated to making Rust UI wonderful."
        }
    ))
}
