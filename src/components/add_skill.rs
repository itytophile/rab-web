use diacritics::remove_diacritics;
use dioxus::prelude::*;
use lexical_sort::natural_lexical_cmp;

use crate::{
    locale::{en::English, Locale, Translation, UiSymbole},
    DisplaySkill,
};

#[inline_props]
pub(crate) fn AddSkill<'a>(
    cx: Scope,
    options: im_rc::HashSet<DisplaySkill>,
    skills: &'a UseState<im_rc::Vector<(DisplaySkill, u8)>>,
    locale: Locale,
) -> Element {
    let is_active = use_state(&cx, || false);

    cx.render(rsx! {
        div { class: "control",
            button { class: "button is-primary", onclick: |_| is_active.set(true),
                span { class: "icon is-small",
                    i { class: "fa-solid fa-pepper-hot" }
                },
                span { [UiSymbole::AddSkill.translate(*locale)] }
            }
        }
        SelectWish {
            options: options,
            wishes: skills,
            is_active: is_active,
            locale: *locale
        },
    })
}

#[inline_props]
fn SelectWish<'a>(
    cx: Scope,
    options: &'a im_rc::HashSet<DisplaySkill>,
    wishes: &'a UseState<im_rc::Vector<(DisplaySkill, u8)>>,
    is_active: &'a UseState<bool>,
    locale: Locale,
) -> Element {
    // always lowercase
    let filter = use_state(&cx, String::new);

    let class = if ***is_active {
        "modal is-active"
    } else {
        "modal"
    };

    let on_select = |skill| {
        move |_| {
            wishes.make_mut().push_back((skill, 1));
            is_active.set(false)
        }
    };

    let filter_without_diacritics = remove_diacritics(filter);

    let locale = *locale;

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
            Locale::English => cx.render(rsx! { [skill.to_english()] }),
            _ => {
                if !filter.is_empty() && skill.to_english().to_lowercase().contains(filter.as_str())
                {
                    cx.render(rsx! {
                        [skill.translate(locale)]
                        span { class: "is-italic has-text-grey-light ml-1",
                            [skill.to_english()]
                        }
                    })
                } else {
                    cx.render(rsx! {
                        [skill.translate(locale)]
                    })
                }
            }
        };
        rsx! {
            a { class: "panel-block", onclick: on_select(skill),
                text
            }
        }
    });

    let placeholder = UiSymbole::Filter.translate(locale);

    cx.render(rsx! {
        div { class: "{class}",
            div { class: "modal-background", onclick: |_| is_active.set(false) }
            div { class: "modal-card",
                header { class: "modal-card-head",
                    div { class: "modal-card-title mr-5",
                        p { class: "control has-icons-left",
                            input {
                                class: "input is-fullwidth",
                                r#type: "text",
                                placeholder: "{placeholder}",
                                oninput: |event| filter.set(event.value.to_lowercase())
                            }
                            span { class: "icon is-left",
                                i { class: "fas fa-search" }
                            }
                        }
                    },
                    button { class: "delete", onclick: |_| is_active.set(false) }
                }
                div { class: "modal-card-body",
                    options
                }
            }
        }
    })
}
