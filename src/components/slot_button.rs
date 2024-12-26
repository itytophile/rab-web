use dioxus::prelude::*;

#[component]
pub fn SlotButton(slots: Signal<[u8; 3]>, value: u8, index: usize) -> Element {
    let increment = move |_| {
        slots.write()[index] = (value + 1) % 4;
    };

    rsx!(div { class: "control",
        button { class: "button", onclick: increment,
        span {class:"icon is-small", "{value}"}
        }
    })
}
