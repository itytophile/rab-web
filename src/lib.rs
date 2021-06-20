// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.

fn init(_: Url, _: &mut impl Orders<Msg>) -> Model {
    Model { is_loading: false }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    is_loading: bool,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    ToggleLoading,
}

// `update` describes how to handle each `Msg`.
fn update(msg: Msg, model: &mut Model, _: &mut impl Orders<Msg>) {
    match msg {
        Msg::ToggleLoading => model.is_loading ^= true,
    }
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    div![
        C!["container", "has-text-centered"],
        view_search_button(model.is_loading),
        view_filter_input(),
        std::iter::once(view_wish_field()).cycle().take(5),
    ]
}

fn view_search_button(is_loading: bool) -> Node<Msg> {
    div![
        C!["field"],
        button![
            C!["button", "is-primary", IF!(is_loading => "is-loading")],
            span![C!["icon"], i![C!["fas", "fa-search"]]],
            span!["Search builds"],
            ev(Ev::Click, |_| Msg::ToggleLoading)
        ]
    ]
}

fn view_wish_field() -> Node<Msg> {
    div![
        C!["field", "has-addons", "has-addons-centered"],
        div![
            C!["control"],
            div![
                C!["select"],
                select![option!["Select dropdown"], option!["With options"]]
            ]
        ],
        div![
            C!["control"],
            input![
                C!["input"],
                attrs! {
                    At::Type => "number",
                    At::Value => 1,
                    At::Min => 1,
                    At::Max => 5
                }
            ]
        ],
        div![
            C!["control"],
            a![
                C!["button", "is-danger"],
                span![C!["icon"], i![C!["fas", "fa-trash-alt"]]]
            ]
        ],
    ]
}

fn view_filter_input() -> Node<Msg> {
    div![
        C!["field"],
        div![
            C!["control", "has-icons-left", "has-icons-right"],
            input![
                C!["input"],
                attrs! {
                    At::Type => "text",
                    At::Placeholder => "Skill filter"
                },
                input_ev(Ev::Input, |_| log!("zebi"))
            ],
            span![
                C!["icon", "is-small", "is-left"],
                i![C!["fas", "fa-filter"]]
            ],
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::start("root", init, update, view);
}
