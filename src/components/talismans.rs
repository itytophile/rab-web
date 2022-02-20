use crate::{
    components::{AddSkill, SkillRow, SlotButton},
    locale::{Locale, Translation, UiSymbole},
    DisplaySkill,
};
use dioxus::prelude::*;
use rab_core::armor_and_skills::Skill;

#[inline_props]
pub fn Talismans(cx: Scope, locale: Locale) -> Element {
    let locale = *locale;
    let (skills, set_skills) = use_state(&cx, im_rc::Vector::<(DisplaySkill, u8)>::new);
    let all_skills: im_rc::HashSet<DisplaySkill> =
        Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: im_rc::HashSet<DisplaySkill> =
        all_skills.difference(skills.iter().map(|(skill, _)| *skill).collect());
    let (talisman_slots, set_talisman_slots) = use_state(&cx, || [0u8; 3]);

    let weapon_slot_buttons = talisman_slots.iter().enumerate().map(|(index, value)| {
        rsx!(SlotButton {
            set_slots: set_talisman_slots
            value: *value
            index: index
        })
    });

    let save_disabled = skills.is_empty();

    let rows = skills.iter().enumerate().map(|(index, (skill, amount))| {
        rsx! {
            div { class: "field has-addons",
                SkillRow {
                    set_skills: set_skills,
                    index: index,
                    skill: *skill,
                    amount: *amount,
                    locale: locale
                }
            }
        }
    });

    cx.render(rsx!(div { class: "field is-grouped",
            AddSkill {
                options: available_skills,
                set_skills: set_skills,
                locale: locale
            }
            div { class: "control",
                button { class: "button is-info", disabled: "{save_disabled}",
                    span { class: "icon is-small",
                        i { class: "fa-solid fa-check" }
                    }
                    span { [UiSymbole::Save.translate(locale)] }
                }
            }
        }
        div { class: "field has-addons",
            div { class: "control",
                button { class: "button is-static",
                    span { class: "icon is-small",
                        i { class: "fa-solid fa-gem" }
                    }
                }
            }
            weapon_slot_buttons
        }
        rows
    ))
}
