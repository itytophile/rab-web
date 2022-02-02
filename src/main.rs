use rab_core::armor_and_skills::Skill;
use std::{
    fmt::{Display, Formatter},
    ops::Deref,
};
use sycamore::{prelude::*, rt::Event};

#[component]
fn App<G: Html>(ctx: ScopeRef, _: ()) -> View<G> {
    let wishes: &Signal<Vec<(DisplaySkill, &Signal<u8>)>> = ctx.create_signal(vec![]);
    let available_skills: &Signal<Vec<DisplaySkill>> =
        ctx.create_signal(Skill::ALL.iter().copied().map(DisplaySkill).collect());

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
            AddWish(available_skills, wishes)
            Indexed {
                iterable: wishes,
                view: move |ctx, (skill, amount)| view! { ctx,
                    div(class="field has-addons") {
                        Button(ButtonType::Remove, remove_wish(skill), ||false)
                        WishRow(skill, amount) } } } } } }
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

#[component]
fn AddWish<'a, G: Html>(
    ctx: ScopeRef<'a>,
    available_skills: &'a Signal<Vec<DisplaySkill>>,
    wishes: &'a Signal<Vec<(DisplaySkill, &'a Signal<u8>)>>,
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
    div(class="field") {
        div(class="control") {
            button(class="button is-primary",on:click=|_|is_active.set(true)) {"Add wish"} } }
    Select(available_skills, on_select, is_active) }
}

#[component]
fn WishRow<'a, G: Html>(ctx: ScopeRef<'a>, skill: DisplaySkill, amount: &'a Signal<u8>) -> View<G> {
    let decrement = move |_| amount.set(*amount.get() - 1);
    let increment = move |_| amount.set(*amount.get() + 1);
    let is_max = move || skill.get_limit() == *amount.get();
    let is_min = || *amount.get() == 1;

    view! { ctx,
    SkillText(skill)
    Button(ButtonType::Minus, decrement, is_min)
    AmountText(amount)
    Button(ButtonType::Plus, increment, is_max) }
}

#[component]
fn Select<'a, T: 'static, G: Html>(
    ctx: ScopeRef<'a>,
    options: &'a Signal<Vec<T>>,
    on_select: impl Fn(T) + Copy + 'a,
    is_active: &'a Signal<bool>,
) -> View<G>
where
    T: PartialEq + Clone + Display + Copy,
{
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
            span(class="icon is-small") { i(class=icon_class) } } } }
}

fn main() {
    sycamore::render(|ctx| {
        view! { ctx, App {} }
    });
}
