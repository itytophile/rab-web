use crate::{
    components::{AddSkill, SkillRow, SlotButton},
    locale::{Locale, Translation, UiSymbole},
    storage::MyStorage,
    DisplaySkill, Talisman,
};
use dioxus::prelude::*;
use rab_core::armor_and_skills::Skill;
use web_sys::Storage;

#[component]
pub(crate) fn Talismans(
    skills: Signal<im_rc::Vector<(DisplaySkill, u8)>>,
    locale: Locale,
    talismans: Signal<im_rc::Vector<Talisman>>,
    storage: ReadOnlySignal<Storage>,
) -> Element {
    let all_skills: im_rc::HashSet<DisplaySkill> =
        Skill::ALL.iter().copied().map(DisplaySkill).collect();
    let available_skills: im_rc::HashSet<DisplaySkill> =
        all_skills.difference(skills.read().iter().map(|(skill, _)| *skill).collect());
    let mut talisman_slots = use_signal(|| [0u8; 3]);
    let mut talisman_name = use_signal(String::new);

    let talisman_slots_r = talisman_slots.read();

    let weapon_slot_buttons = talisman_slots_r.iter().enumerate().map(|(index, value)| {
        rsx!(SlotButton {
            slots: talisman_slots,
            value: *value,
            index: index
        })
    });

    let save_disabled = skills.read().is_empty() || talisman_name.read().is_empty();

    let skills_r = skills.read();

    let rows = skills_r.iter().enumerate().map(|(index, (skill, amount))| {
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

    let save_talisman = move |_| {
        talismans.with_mut(|talismans| {
            talismans.push_back(Talisman {
                name: talisman_name.to_string(),
                skills: skills
                    .read()
                    .iter()
                    .map(|&(skill, amount)| (*skill, amount))
                    .collect(),
                slots: talisman_slots
                    .read()
                    .iter()
                    .copied()
                    .filter(|slot| *slot != 0)
                    .collect(),
            });
            storage.read().talismans().save(talismans);
        });
        talisman_name.set(String::new());
        skills.set(im_rc::Vector::new());
        talisman_slots.set(Default::default())
    };

    let talismans_r = talismans.read();

    let talisman_views = talismans_r.iter().enumerate().map(|(index, _)| {
        rsx!(TalismanView {
            locale: locale,
            talismans: talismans,
            index: index,
            storage: storage
        })
    });

    rsx!(
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
                                oninput: move |event| talisman_name.set(event.value().clone())
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
                    {weapon_slot_buttons}
                }
                {rows}
            }
            div { class: "column",
                {talisman_views}
            }
        }
    )
}

#[component] // can't use build a parameter name
fn TalismanView(
    index: usize,
    locale: Locale,
    talismans: Signal<im_rc::Vector<Talisman>>,
    storage: ReadOnlySignal<Storage>,
) -> Element {
    let talisman = &talismans.read()[index];
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
            {DisplaySkill(*skill).translate(locale)}
            "{amount}"
        })
    });

    let talisman_name = talisman.name.clone();

    rsx!(article { class: "panel is-primary",
        p { class: "message-header" ,
            {talisman_name}
            button {
                class: "delete",
                onclick: move |_| {
                    talismans.with_mut(|talismans| {
                        talismans.remove(index);
                        storage.read().talismans().save(talismans)
                    })
                }
            }
        }
        span { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-gem"}
            }
            "{slots}"
        }
        {skills}
    })
}
