use anyhow::Result;
use rab_core::armor_and_skills::{Armor, Skill};
use reqwasm::http::Request;
use ron::de::from_str;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};
use sycamore::{prelude::*, rt::Event, suspense::Suspense};

const BASE_URL: &str =
    "https://raw.githubusercontent.com/itytophile/monster-hunter-rise-armors/main/";

async fn fetch_armors(name: &str) -> Result<Vec<Armor>> {
    from_str(
        &Request::get(&format!("{BASE_URL}{name}.ron"))
            .send()
            .await?
            .text()
            .await?,
    )
    .map_err(|err| err.into())
}

#[component]
fn App<G: Html>(ctx: ScopeRef) -> View<G> {
    let wishes = ctx.create_signal(vec![]);
    let available_skills = ctx.create_signal(
        Skill::ALL
            .iter()
            .copied()
            .map(DisplaySkill)
            .collect::<Vec<DisplaySkill>>(),
    );

    let remove_wish = move |skill| {
        move |_| {
            let mut tmp = available_skills.get().to_vec();
            tmp.push(skill);
            available_skills.set(tmp);
            let mut tmp = wishes.get().to_vec();
            tmp.remove(tmp.iter().position(|(s, _)| *s == skill).unwrap());
            wishes.set(tmp)
        }
    };

    view! { ctx,
    section(class="section") {
        div(class="container") {
            div(class="columns") {
                div(class="column") {
                    div(class="field is-grouped") {
                        AddWish { available_skills: available_skills, wishes: wishes }
                        div(class="control") {
                            button(class="button is-info") {
                                span(class="icon is-small") {
                                    i(class="fas fa-search") }
                                span {"Search builds"} } } }
                    Indexed {
                        iterable: wishes,
                        view: move |ctx, (skill, amount)| view! { ctx,
                            div(class="field has-addons") {
                                Button {
                                    button_type: ButtonType::Remove,
                                    on_click:remove_wish(skill),
                                    is_disabled: ||false
                                }
                                WishRow { skill: skill, amount: amount } } } } }
                div(class="column") {
                    Suspense {
                        fallback: view! { ctx, "Loading..." },
                        BuildList {}
                    }
                } } } } }
}

#[component]
async fn BuildList<G: Html>(ctx: ScopeRef<'_>) -> View<G> {
    let helmets = fetch_armors("helmets").await.unwrap();
    let chests = fetch_armors("chests").await.unwrap();
    let arms = fetch_armors("arms").await.unwrap();
    let waists = fetch_armors("waists").await.unwrap();
    let legs = fetch_armors("legs").await.unwrap();

    view! {ctx,
    article(class="panel is-primary") {
        p(class="panel-heading")
        a(class="panel-block") {
            "helmet"
        }
        a(class="panel-block") {
            "chest"
        }
        a(class="panel-block") {
            "arms"
        }
        a(class="panel-block") {
            "waist"
        }
        a(class="panel-block") {
            "legs"
        }
    }
    article(class="panel is-primary") {
        p(class="panel-heading")
        a(class="panel-block") {
            "helmet"
        }
        a(class="panel-block") {
            "chest"
        }
        a(class="panel-block") {
            "arms"
        }
        a(class="panel-block") {
            "waist"
        }
        a(class="panel-block") {
            "legs"
        }
    }
    }
}

#[derive(Clone, Copy, PartialEq)]
struct DisplaySkill(Skill);

impl Display for DisplaySkill {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let DisplaySkill(skill) = self;
        f.write_fmt(format_args!("{skill:?}"))
    }
}

impl Deref for DisplaySkill {
    type Target = Skill;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Prop)]
struct AddWishProps<'a> {
    available_skills: &'a Signal<Vec<DisplaySkill>>,
    wishes: &'a Signal<Vec<(DisplaySkill, &'a Signal<u8>)>>,
}

#[component]
fn AddWish<'a, G: Html>(
    ctx: ScopeRef<'a>,
    AddWishProps {
        available_skills,
        wishes,
    }: AddWishProps<'a>,
) -> View<G> {
    let is_active = ctx.create_signal(false);

    let on_select = |skill| {
        let mut tmp = wishes.get().to_vec();
        tmp.push((skill, ctx.create_signal(1)));
        wishes.set(tmp);
        is_active.set(false);

        let mut tmp = available_skills.get().to_vec();
        tmp.remove(tmp.iter().position(|s| *s == skill).unwrap());
        available_skills.set(tmp);
    };

    view! { ctx,
    div(class="control") {
        button(class="button is-primary",on:click=|_|is_active.set(true)) {
            span(class="icon is-small") { i(class="fas fa-pepper-hot")}
            span {"Add wish"}} }
    Select {
        options: available_skills,
        on_select:on_select,
        is_active:is_active
    } }
}

#[derive(Prop)]
struct WishRowProps<'a> {
    skill: DisplaySkill,
    amount: &'a Signal<u8>,
}

#[component]
fn WishRow<'a, G: Html>(
    ctx: ScopeRef<'a>,
    WishRowProps { skill, amount }: WishRowProps<'a>,
) -> View<G> {
    let decrement = move |_| amount.set(*amount.get() - 1);
    let increment = move |_| amount.set(*amount.get() + 1);
    let is_max = move || skill.get_limit() == *amount.get();
    let is_min = || *amount.get() == 1;

    view! { ctx,
    SkillText(skill)
    Button {
        button_type: ButtonType::Minus,
        on_click: decrement,
        is_disabled: is_min
    }
    AmountText(amount)
    Button {
        button_type: ButtonType::Plus,
        on_click: increment,
        is_disabled: is_max
    } }
}

#[derive(Prop)]
struct Select<'a, T, F> {
    options: &'a Signal<Vec<T>>,
    on_select: F,
    is_active: &'a Signal<bool>,
}

#[component]
fn Select<'a, T: 'static + PartialEq + Clone + Display + Copy, G: Html>(
    ctx: ScopeRef<'a>,
    Select {
        options,
        on_select,
        is_active,
    }: Select<'a, T, impl Fn(T) + Copy + 'a>,
) -> View<G> {
    let class = || {
        if *is_active.get() {
            "modal is-active"
        } else {
            "modal"
        }
    };

    // to tell the compiler what the lifetimes are
    let indexed_props = IndexedProps::<'a> {
        iterable: options,
        view: move |ctx, option| {
            view! { ctx,
            div(class="panel-block",on:click=move |_|on_select(option)) { (option) } }
        },
    };

    view! { ctx,
    div(class=(class())) {
        div(class="modal-background",on:click=|_|is_active.set(false))
        div(class="modal-card") {
            header(class="modal-card-head") {
                div(class="modal-card-title mr-5") {
                    p(class="control has-icons-left") {
                        input(class="input is-fullwidth",type="text",placeholder="Search")
                        span(class="icon is-left") {
                            i(class="fas fa-search",aria-hidden=true) } } }
                button(class="delete",aria-label="close",on:click=|_|is_active.set(false)) }
            div(class="modal-card-body") {
                Indexed(indexed_props) } } } }
}

#[component]
fn SkillText<G: Html>(ctx: ScopeRef, skill: DisplaySkill) -> View<G> {
    view! { ctx,
    div(class="control") {
        input(class="input", size=15, readonly=true, value=(skill)) } }
}

#[component]
fn AmountText<'a, G: Html>(ctx: ScopeRef<'a>, amount: &'a Signal<u8>) -> View<G> {
    view! { ctx,
    div(class="control") {
        input(class="input", size=1, readonly=true, value=(amount.get())) } }
}

enum ButtonType {
    Remove,
    Plus,
    Minus,
}

#[derive(Prop)]
struct ButtonProps<C, D> {
    button_type: ButtonType,
    on_click: C,
    is_disabled: D,
}

#[component]
fn Button<'a, G: Html>(
    ctx: &'a Scope<'a>,
    ButtonProps {
        button_type,
        on_click,
        is_disabled,
    }: ButtonProps<impl Fn(Event) + 'a, impl Fn() -> bool + 'a>,
) -> View<G> {
    let (button_class, icon_class) = match button_type {
        ButtonType::Remove => ("button is-danger", "fas fa-trash"),
        ButtonType::Plus => ("button is-success", "fas fa-plus"),
        ButtonType::Minus => ("button is-link", "fas fa-minus"),
    };

    view! { ctx,
    div(class="control") {
        button(class=button_class,on:click=on_click,disabled=is_disabled()) {
            span(class="icon is-small") { i(class=icon_class) } } } }
}

fn main() {
    sycamore::render(|ctx| {
        view! { ctx, App {} }
    });
}
