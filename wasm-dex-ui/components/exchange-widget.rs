pub fn ExchangeWidget(cx: Scope) -> Element {
    cx.render(rsx!(
        p {
            input { class: "edit", value: "test" }
            b { "Exchange Widget" }
            "_"
        }
    ))
}
