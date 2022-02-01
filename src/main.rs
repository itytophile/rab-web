use rab_core::armor_and_skills::Skill;
use sycamore::{prelude::*, rt::Event};

type Wish = (Skill, u8);

#[component]
fn App<G: Html>(ctx: ScopeRef, _: ()) -> View<G> {
    let wishes = ctx.create_signal(vec![((Skill::Botanist, 1u8), 0)]);

    fn decrement(index: usize, wishes: &Signal<Vec<(Wish, usize)>>) -> impl Fn(Event) + '_ {
        move |_| {
            let mut tmp = wishes.get().to_vec();
            let ((_, ref mut amount), _) = tmp[index];
            *amount -= 1;
            wishes.set(tmp)
        }
    }

    fn increment(index: usize, wishes: &Signal<Vec<(Wish, usize)>>) -> impl Fn(Event) + '_ {
        move |_| {
            let mut tmp = wishes.get().to_vec();
            let ((_, ref mut amount), _) = tmp[index];
            *amount += 1;
            wishes.set(tmp)
        }
    }

    fn remove(index: usize, wishes: &Signal<Vec<(Wish, usize)>>) -> impl Fn(Event) + '_ {
        move |_| {
            let mut tmp = wishes.get().to_vec();
            tmp.remove(index);
            wishes.set(tmp)
        }
    }

    view! { ctx,
        section(class="section") {
            div(class="container") {
                Indexed {
                    iterable: wishes,
                    view: move |ctx, (wish, index)| view! { ctx,
                        div(class="field has-addons") {
                            Button(ButtonType::Remove, remove(index, wishes))
                            WishRow(wish, decrement(index, wishes), increment(index, wishes))
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
fn WishRow<'a, G, F0, F1>(
    ctx: &'a Scope<'a>,
    wish: (Skill, u8),
    decrement: F0,
    increment: F1,
) -> View<G>
where
    F0: Fn(Event) + 'a,
    F1: Fn(Event) + 'a,
    G: Html,
{
    let (skill, amount) = wish;

    view! { ctx,
        SelectSkill()
        Button(ButtonType::Minus, decrement)
        AmountText(amount)
        Button(ButtonType::Plus, increment)
    }
}

#[component]
fn SelectSkill<G: Html>(ctx: ScopeRef, _: ()) -> View<G> {
    view! { ctx,
        div(class="control") {
            div(class="select") {
                select {
                    option {"lol"}
                }
            }
        }
    }
}

#[component]
fn AmountText<G: Html>(ctx: ScopeRef, amount: u8) -> View<G> {
    view! { ctx,
        div(class="control") {
            input(class="input", size=1, readonly=true, value=amount)
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
) -> View<G> {
    let (button_class, icon_class) = match button_type {
        ButtonType::Remove => ("button is-danger", "fas fa-trash"),
        ButtonType::Plus => ("button is-success", "fas fa-plus"),
        ButtonType::Minus => ("button is-link", "fas fa-minus"),
    };

    view! { ctx,
        div(class="control") {
            button(class=button_class,on:click=on_click) {
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
