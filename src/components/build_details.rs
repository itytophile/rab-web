use crate::{
    armors,
    locale::{Locale, Translation, UiSymbole},
    DisplaySkill,
};
use dioxus::prelude::*;
use rab_core::{
    armor_and_skills::{Armor, Skill},
    build_search::Build,
};
use std::str::FromStr;

fn armor_to_string(armor: Option<&(Armor, [Option<Skill>; 3])>, locale: Locale) -> &str {
    if let Some((armor, _)) = armor {
        if let Ok(name) = armors::Armor::from_str(&armor.name) {
            name.translate(locale)
        } else {
            &armor.name
        }
    } else {
        "Free"
    }
}

#[inline_props] // can't use build as parameter name
pub(crate) fn BuildDetails<'a>(cx: Scope, b: &'a Build, locale: Locale) -> Element {
    let (visible, set_visible) = use_state(&cx, || false);
    let locale = *locale;
    let b = *b;

    let mut all_skills: Vec<(Skill, u8)> = b.get_all_skills_and_amounts().drain().collect();
    all_skills.sort_unstable_by_key(|(_, amount)| *amount);

    let skills = all_skills.iter().rev().map(|(skill, amount)| {
        let amount = format!(" x{amount}");
        rsx!(span { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-pepper-hot"}
            }
            [DisplaySkill(*skill).translate(locale)]
            "{amount}"
        })
    });

    cx.render(rsx!(
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-hat-cowboy"}
            }
            [armor_to_string(b.helmet.as_ref(), locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-shirt"}
            }
            [armor_to_string(b.chest.as_ref(), locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-mitten"}
            }
            [armor_to_string(b.arm.as_ref(),locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-archway"}
            }
            [armor_to_string(b.waist.as_ref(),locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-socks"}
            }
            [armor_to_string(b.leg.as_ref(),locale)]
        }
        a { class: "panel-block",
            span {class:"panel-icon", aria_hidden:"true",
                i {class:"fa-solid fa-lightbulb"}
            }
            [armor_to_string(b.talisman.as_ref(),locale)]
        }
        div { class: "panel-block",
            button {
                class: "button is-link is-outlined is-fullwidth",
                onclick: |_| *set_visible.make_mut() ^= true,
                [if *visible { UiSymbole::HideSkills } else { UiSymbole::ShowSkills }.translate(locale)]
            }
        }
        visible.then(|| rsx!(skills))
    ))
}
