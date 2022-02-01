use rab_core::armor_and_skills::Skill;
use sycamore::{prelude::*, rt::Event};

type Wish = (Skill, u8);

#[component]
fn App<G: Html>(ctx: ScopeRef, _: ()) -> View<G> {
    let wishes = ctx.create_signal(vec![ctx.create_signal((Skill::Botanist, 1u8))]);

    let on_click = |_| {
        let mut tmp = wishes.get().to_vec();
        tmp.push(ctx.create_signal((Skill::Botanist, 1u8)));
        wishes.set(tmp);
    };

    view! { ctx,
        section(class="section") {
            div(class="container") {
                div(class="field is-grouped") {
                    p(class="control") {
                        button(class="button is-success",on:click=on_click) {"Add Wish"}
                    }
                }
                Indexed {
                    iterable: wishes,
                    view: move |ctx, wish| view! { ctx,
                        div(class="field has-addons") {
                            Button(ButtonType::Remove, |_|{}, ||false)
                            WishRow(wish)
                        }
                    }
                }
            }
        }
    }
}

trait Lol {}

impl Lol for Signal<Vec<((Skill, u8), usize)>> {}

#[component]
fn WishRow<'a, G: Html>(ctx: ScopeRef<'a>, wish: &'a Signal<Wish>) -> View<G> {
    let decrement = move |_| {
        let (skill, amount) = *wish.get();
        wish.set((skill, amount - 1))
    };

    let increment = move |_| {
        let (skill, amount) = *wish.get();
        wish.set((skill, amount + 1))
    };

    let is_limit = move || {
        let (skill, amount) = *wish.get();
        skill.get_limit() == amount
    };

    view! { ctx,
        SelectSkill(wish)
        Button(ButtonType::Minus, decrement, || wish.get().1 == 1)
        AmountText(wish)
        Button(ButtonType::Plus, increment, is_limit)
    }
}

#[component]
fn SelectSkill<G: Html>(ctx: ScopeRef, wish: &Signal<Wish>) -> View<G> {
    let name = format!("{:?}", wish.get().0);

    let options = View::new_fragment(
        Skill::ALL
            .iter()
            .map(|skill| format!("{skill:?}"))
            .map(|name| {
                let label = name.clone();
                view! {ctx, option(value=name){(label)}}
            })
            .collect(),
    );

    view! { ctx,
        div(class="control") {
            div(class="select") {
                select(value=name) {
                    (options)
                }
            }
        }
    }
}

#[component]
fn AmountText<'a, G: Html>(ctx: ScopeRef<'a>, wish: &'a Signal<Wish>) -> View<G> {
    let lol = || wish.get().1;

    view! { ctx,
        div(class="control") {
            input(class="input", size=1, readonly=true, value=(lol()))
        }
    }
}

enum ButtonType {
    Remove,
    Plus,
    Minus,
}

#[component]
fn Button<'a, G: Html>(
    ctx: &'a Scope<'a>,
    button_type: ButtonType,
    on_click: impl Fn(Event) + 'a,
    is_disabled: impl Fn() -> bool + 'a,
) -> View<G> {
    let (button_class, icon_class) = match button_type {
        ButtonType::Remove => ("button is-danger", "fas fa-trash"),
        ButtonType::Plus => ("button is-success", "fas fa-plus"),
        ButtonType::Minus => ("button is-link", "fas fa-minus"),
    };

    view! { ctx,
        div(class="control") {
            button(class=button_class,on:click=on_click,disabled=is_disabled()) {
                span(class="icon is-small") { i(class=icon_class) }
            }
        }
    }
}

fn main() {
    sycamore::render(|ctx| {
        view! { ctx,
            App {}
        }
    });
}
