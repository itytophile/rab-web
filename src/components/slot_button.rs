use dioxus::prelude::*;

#[inline_props]
pub fn SlotButton<'a>(cx: Scope, slots: &'a UseState<[u8; 3]>, value: u8, index: usize) -> Element {
    let index = *index;

    let increment = move |_| {
        slots.make_mut()[index] = (value + 1) % 4;
    };

    cx.render(rsx!(div { class: "control",
        button { class: "button", onclick: increment,
        span {class:"icon is-small", "{value}"}
        }
    }))
}
