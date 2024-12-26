use diacritics::remove_diacritics;
use dioxus::prelude::*;
use lexical_sort::natural_lexical_cmp;

use crate::{
    locale::{en::English, Locale, Translation, UiSymbole},
    DisplaySkill,
};

#[component]
pub(crate) fn AddSkill(
    options: im_rc::HashSet<DisplaySkill>,
    skills: Signal<im_rc::Vector<(DisplaySkill, u8)>>,
    locale: Locale,
) -> Element {
    let mut is_active = use_signal(|| false);

    rsx! {
        div { class: "control",
            button { class: "button is-primary", onclick: move |_| is_active.set(true),
                span { class: "icon is-small",
                    i { class: "fa-solid fa-pepper-hot" }
                },
                span { {UiSymbole::AddSkill.translate(locale)} }
            }
        }
        SelectWish {
            options: options,
            wishes: skills,
            is_active: is_active,
            locale: locale
        },
    }
}

#[component]
fn SelectWish(
    options: im_rc::HashSet<DisplaySkill>,
    wishes: Signal<im_rc::Vector<(DisplaySkill, u8)>>,
    is_active: Signal<bool>,
    locale: Locale,
) -> Element {
    // always lowercase
    let mut filter = use_signal(String::new);

    let class = if *is_active.read() {
        "modal is-active"
    } else {
        "modal"
    };

    let on_select = |skill| {
        move |_| {
            wishes.write().push_back((skill, 1));
            is_active.set(false)
        }
    };

    let filter_without_diacritics = remove_diacritics(&filter.read());

    let mut options: Vec<DisplaySkill> = options
        .iter()
        .copied()
        .filter(|skill| {
            remove_diacritics(&skill.translate(locale).to_lowercase())
                .contains(&filter_without_diacritics)
                || skill
                    .to_english()
                    .to_lowercase()
                    .contains(&filter_without_diacritics)
        })
        .collect();

    options.sort_unstable_by(|a, b| natural_lexical_cmp(a.translate(locale), b.translate(locale)));

    let options = options.iter().map(|&skill| {
        let text = match locale {
            Locale::English => rsx! { {skill.to_english()} },
            _ => {
                if !filter.read().is_empty()
                    && skill
                        .to_english()
                        .to_lowercase()
                        .contains(filter.read().as_str())
                {
                    rsx! {
                        {skill.translate(locale)}
                        span { class: "is-italic has-text-grey-light ml-1",
                            {skill.to_english()}
                        }
                    }
                } else {
                    rsx! {
                        {skill.translate(locale)}
                    }
                }
            }
        };
        rsx! {
            a { class: "panel-block", onclick: on_select(skill),
                {text}
            }
        }
    });

    let placeholder = UiSymbole::Filter.translate(locale);

    rsx! {
        div { class: "{class}",
            div { class: "modal-background", onclick: move |_| is_active.set(false) }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    div { class: "modal-card-title mr-5",
                        p { class: "control has-icons-left",
                            input {
                                class: "input is-fullwidth",
                                r#type: "text",
                                placeholder: "{placeholder}",
                                oninput: move |event| filter.set(event.value().to_lowercase())
                            }
                            span { class: "icon is-left",
                                i { class: "fas fa-search" }
                            }
                        }
                    },
                    button { class: "delete", onclick: move |_| is_active.set(false) }
                }
                div { class: "modal-card-body",
                    {options}
                }
            }
        }
    }
}
