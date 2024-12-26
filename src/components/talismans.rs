use crate::{
    components::{AddSkill, SkillRow, SlotButton},
    locale::{Locale, Translation, UiSymbole},
    storage::MyStorage,
    DisplaySkill, Talisman,
};
use dioxus::prelude::*;
use rab_core::armor_and_skills::Skill;
use web_sys::Storage;

#[inline_props]
pub(crate) fn Talismans<'a>(
    cx: Scope,
    skills: &'a UseState<im_rc::Vector<(DisplaySkill, u8)>>,
    locale: Locale,
    talismans: &'a UseState<im_rc::Vector<Talisman>>,
    storage: &'a Storage,
) -> Element<'a> {
    let locale = *locale;
    let all_skills: im_rc::HashSet<DisplaySkill> =
        Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: im_rc::HashSet<DisplaySkill> =
        all_skills.difference(skills.iter().map(|(skill, _)| *skill).collect());
    let talisman_slots = use_state(&cx, || [0u8; 3]);
    let talisman_name = use_state(&cx, String::new);

    let weapon_slot_buttons = talisman_slots.iter().enumerate().map(|(index, value)| {
        rsx!(SlotButton {
            slots: talisman_slots
            value: *value
            index: index
        })
    });

    let save_disabled = skills.is_empty() || talisman_name.is_empty();

    let rows = skills.iter().enumerate().map(|(index, (skill, amount))| {
        rsx! {
            div { class: "field has-addons",
                SkillRow {
                    skills: skills,
                    index: index,
                    skill: *skill,
                    amount: *amount,
                    locale: locale
                }
            }
        }
    });

    let placeholder = UiSymbole::TalismansName.translate(locale);

    let save_talisman = |_| {
        talismans.with_mut(|talismans| {
            talismans.push_back(Talisman {
                name: talisman_name.to_string(),
                skills: skills
                    .iter()
                    .map(|&(skill, amount)| (*skill, amount))
                    .collect(),
                slots: talisman_slots
                    .iter()
                    .copied()
                    .filter(|slot| *slot != 0)
                    .collect(),
            });
            storage.talismans().save(talismans);
        });
        talisman_name.set(String::new());
        skills.set(im_rc::Vector::new());
        talisman_slots.set(Default::default())
    };

    let talisman_views = talismans.iter().enumerate().map(|(index, talisman)| {
        rsx!(TalismanView {
            talisman: talisman,
            locale: locale,
            talismans: talismans,
            index: index,
            storage: storage
        })
    });

    cx.render(rsx!(
        div { class: "columns",
            div { class: "column",
                div { class: "field is-grouped",
                    AddSkill {
                        options: available_skills,
                        skills: skills,
                        locale: locale
                    }
                    div {class: "field has-addons",
                        div { class: "control",
                            input {
                                class: "input",
                                r#type: "text",
                                placeholder: "{placeholder}",
                                oninput: |event| talisman_name.set(event.value.clone())
                            }
                        }
                        div { class: "control",
                            button {
                                class: "button is-info",
                                disabled: "{save_disabled}",
                                onclick: save_talisman,
                                span { class: "icon is-small",
                                    i { class: "fa-solid fa-check" }
                                }
                            }
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
            }
            div { class: "column",
                talisman_views
            }
        }
    ))
}

#[inline_props] // can't use build a parameter name
fn TalismanView<'a>(
    cx: Scope,
    talisman: &'a Talisman,
    locale: Locale,
    talismans: &'a UseState<im_rc::Vector<Talisman>>,
    index: usize,
    storage: &'a Storage,
) -> Element<'a> {
    let locale = *locale;

    let slots = if talisman.slots.is_empty() {
        UiSymbole::NoSlots.translate(locale).to_owned()
    } else {
        let mut slots = String::new();
        for slot in &talisman.slots {
            slots.push_str(&format!("x{slot} "));
        }
        slots
    };

    let skills = talisman.skills.iter().map(|(skill, amount)| {
        let amount = format!(" x{amount}");
        rsx!(span { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-pepper-hot"}
            }
            [DisplaySkill(*skill).translate(locale)]
            "{amount}"
        })
    });

    let delete_talisman = |_| {
        talismans.with_mut(|talismans| {
            talismans.remove(*index);
            storage.talismans().save(talismans)
        })
    };

    cx.render(rsx!(article { class: "panel is-primary",
        p { class: "message-header" ,
            [talisman.name.as_str()]
            button {
                class: "delete",
                onclick: delete_talisman
            }
        }
        span { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-gem"}
            }
            "{slots}"
        }
        skills
    }))
}
