use dioxus::prelude::*;

use crate::{
    locale::{Locale, Translation},
    DisplaySkill,
};

#[inline_props]
pub(crate) fn SkillRow<'a>(
    cx: Scope,
    set_skills: &'a UseState<im_rc::Vector<(DisplaySkill, u8)>>,
    index: usize,
    skill: DisplaySkill,
    amount: u8,
    locale: Locale,
) -> Element {
    let (index, skill, amount) = (*index, *skill, *amount);

    let is_minus_disabled = amount == 1;
    let is_plus_disabled = amount == skill.get_limit();

    let remove_skill = move |_| {
        set_skills.make_mut().remove(index);
    };

    let increment = move |_| set_skills.make_mut()[index] = (skill, amount + 1);

    let decrement = move |_| set_skills.make_mut()[index] = (skill, amount - 1);

    let skill = skill.translate(*locale);

    cx.render(rsx! {
        div { class: "control",
            button { class: "button is-danger", onclick: remove_skill,
                span { class: "icon is-small",
                    i { class: "fa-solid fa-trash" }
                }
            }
        }
        div { class: "control",
            input { class: "input", size: "15", readonly: "true", value: "{skill}" }
        },
        div { class: "control",
            button { class: "button is-link", disabled: "{is_minus_disabled}", onclick: decrement,
                span { class: "icon is-small",
                    i { class: "fa-solid fa-minus" }
                }
            }
        }
        div { class: "control",
            input { class: "input", size: "1", readonly: "true", value: "{amount}" }
        },
        div { class: "control",
            button { class: "button is-success", disabled: "{is_plus_disabled}", onclick: increment,
                span { class: "icon is-small",
                    i { class: "fa-solid fa-plus" }
                }
            }
        }
    })
}
