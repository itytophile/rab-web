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
    set_skills: &'a UseState<im_rc::Vector<(DisplaySkill, u8)>>,
    locale: Locale,
    set_talismans: &'a UseState<im_rc::Vector<Talisman>>,
    storage: &'a Storage,
) -> Element {
    let skills = set_skills.get().as_ref();
    let talismans = set_talismans.get().as_ref();

    let locale = *locale;
    let all_skills: im_rc::HashSet<DisplaySkill> =
        Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: im_rc::HashSet<DisplaySkill> =
        all_skills.difference(skills.iter().map(|(skill, _)| *skill).collect());
    let (talisman_slots, set_talisman_slots) = use_state(&cx, || [0u8; 3]);
    let (talisman_name, set_talisman_name) = use_state(&cx, String::new);

    let weapon_slot_buttons = talisman_slots.iter().enumerate().map(|(index, value)| {
        rsx!(SlotButton {
            set_slots: set_talisman_slots
            value: *value
            index: index
        })
    });

    let save_disabled = skills.is_empty() || talisman_name.is_empty();

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

    let placeholder = UiSymbole::TalismansName.translate(locale);

    let save_talisman = |_| {
        set_talismans.with_mut(|talismans| {
            talismans.push_back(Talisman {
                name: talisman_name.clone(),
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
            storage.save_talismans(talismans);
        });
        set_talisman_name(String::new());
        set_skills(im_rc::Vector::new());
        set_talisman_slots(Default::default())
    };

    let talisman_views = talismans.iter().map(|talisman| {
        rsx!(TalismanView {
            talisman: talisman,
            locale: locale
        })
    });

    cx.render(rsx!(
        div { class: "columns",
            div { class: "column",
                div { class: "field is-grouped",
                    AddSkill {
                        options: available_skills,
                        set_skills: set_skills,
                        locale: locale
                    }
                    div {class: "field has-addons",
                        div { class: "control",
                            input {
                                class: "input",
                                r#type: "text",
                                placeholder: "{placeholder}",
                                oninput: |event| set_talisman_name(event.value.clone())
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
fn TalismanView<'a>(cx: Scope, talisman: &'a Talisman, locale: Locale) -> Element {
    let locale = *locale;

    let slots = if talisman.slots.is_empty() {
        UiSymbole::NoSlots.translate(locale).to_owned()
    } else {
        talisman
            .slots
            .iter()
            .map(|slot| format!("x{slot} "))
            .collect()
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

    cx.render(rsx!(article { class: "panel is-primary",
        p { class: "panel-heading",
            [talisman.name.as_str()]
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
