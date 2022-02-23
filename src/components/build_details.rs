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

    let pieces = [
        (
            b.helmet.as_ref(),
            "fa-solid fa-hat-cowboy",
            use_state(&cx, || false),
        ),
        (
            b.chest.as_ref(),
            "fa-solid fa-shirt",
            use_state(&cx, || false),
        ),
        (
            b.arm.as_ref(),
            "fa-solid fa-mitten",
            use_state(&cx, || false),
        ),
        (
            b.waist.as_ref(),
            "fa-solid fa-archway",
            use_state(&cx, || false),
        ),
        (
            b.leg.as_ref(),
            "fa-solid fa-socks",
            use_state(&cx, || false),
        ),
        (
            b.talisman.as_ref(),
            "fa-solid fa-lightbulb",
            use_state(&cx, || false),
        ),
    ]
    .map(|(piece, icon, (is_expanded, set_is_expanded))| {
        let panel_class = if *is_expanded {
            "panel-block is-active"
        } else {
            "panel-block"
        };
        let expansion = if *is_expanded {
            if let Some((_, skills)) = piece {
                Some(rsx!(skills.iter().flatten().map(
                    |skill| rsx!(div { class: "panel-block",
                        span {class:"panel-icon ml-5",
                            i {class:"fa-solid fa-gem"}
                        }
                        [DisplaySkill(*skill).translate(locale)]
                    })
                )))
            } else {
                None
            }
        } else {
            None
        };

        let jewels_count = piece.map(|(_, skills)| skills.iter().flatten().count());

        let jewels_count = if jewels_count == Some(0) {
            None
        } else {
            jewels_count
                .map(|count| format!("{count} {}", if count == 1 { "jewel" } else { "jewels" }))
                .map(|count| {
                    rsx!(span { class: "is-italic has-text-grey-light ml-1",
                        "{count}"
                    })
                })
        };

        rsx!(
            a { class: "{panel_class}", onclick: |_| *set_is_expanded.make_mut() ^= true,
                span {class:"panel-icon",
                    i {class:"{icon}"}
                }
                [armor_to_string(piece, locale)]
                jewels_count

            }
            expansion
        )
    });

    cx.render(rsx!(
        pieces
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
