// (Lines like the one below ignore selected Clippy rules
//  - it's useful when you want to check your code with `cargo make verify`
// but some rules are too "annoying" or are not applicable for your case.)
#![allow(clippy::wildcard_imports)]

use std::cmp::min;

use lexical_sort::natural_lexical_cmp;
use rab_core::{
    armor_and_skills::{Armor, Gender, Skill},
    build_search::{pre_selection_then_brute_force_search, AllArmorSlices, Build},
};
use seed::{prelude::*, *};

// ------ ------
//     Init
// ------ ------

// `init` describes what should happen when your app started.

enum Page {
    Wishes,
    Results,
    ArmorsFetching,
}

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    let mut sorted_skills = Skill::ALL.to_vec();
    sorted_skills.lexical_sort();
    let wishes = vec![(sorted_skills[0], 1)];
    let filtered_skills = sorted_skills
        .iter()
        .filter(|s| !wishes.iter().map(|(s, _)| s).any(|s0| *s == s0))
        .copied()
        .collect();

    orders.perform_cmd(async {
        Msg::EndArmorInitialization(fetch_armors().await.unwrap_or_else(|err| {
            log!(err);
            ArmorLists::default()
        }))
    });

    // orders.perform_cmd(async { log!(fetch_armors().await.expect("lol")) });

    Model {
        is_loading: false,
        is_choosing_skill: false,
        sorted_skills,
        filtered_skills,
        wishes,
        filter: "".to_owned(),
        changing_wish_index: None,
        page: Page::ArmorsFetching,
        armors: ArmorLists::default(),
        searched_builds: vec![],
    }
}

use futures::future::join_all;
const BASE_URL_ARMORS: &str =
    "https://raw.githubusercontent.com/itytophile/monster-hunter-rise-armors/main/";
const HELMETS_FILE: &str = "helmets.ron";
const CHESTS_FILE: &str = "chests.ron";
const ARMS_FILE: &str = "arms.ron";
const WAISTS_FILE: &str = "waists.ron";
const LEGS_FILE: &str = "legs.ron";
const ARMOR_FILE_NAMES: &[&str] = &[HELMETS_FILE, CHESTS_FILE, ARMS_FILE, WAISTS_FILE, LEGS_FILE];

#[derive(Debug, Default, Clone)]
struct ArmorLists {
    helmets: Vec<Armor>,
    chests: Vec<Armor>,
    arms: Vec<Armor>,
    waists: Vec<Armor>,
    legs: Vec<Armor>,
}

async fn fetch_armors() -> Result<ArmorLists, &'static str> {
    // TODO real error management
    let fetch_futures = ARMOR_FILE_NAMES
        .iter()
        .map(|s| fetch(format!("{}{}", BASE_URL_ARMORS, s)));

    let responses: Vec<Response> = join_all(fetch_futures).await.drain(..).flatten().collect();
    let text_futures = responses.iter().map(|resp| resp.text());
    let armors: Vec<Vec<Armor>> = join_all(text_futures)
        .await
        .drain(..)
        .flatten()
        .map(|ron_file| ron::de::from_str(&ron_file))
        .flatten()
        .collect();

    if armors.len() == 5 {
        Ok(ArmorLists {
            helmets: armors[0].clone(),
            chests: armors[1].clone(),
            arms: armors[2].clone(),
            waists: armors[3].clone(),
            legs: armors[4].clone(),
        })
    } else {
        Err("Error while fetching armors")
    }
}

trait LexicalSort {
    fn lexical_sort(&mut self);
}

impl LexicalSort for Vec<Skill> {
    fn lexical_sort(&mut self) {
        self.sort_unstable_by(|a, b| natural_lexical_cmp(&format!("{:?}", a), &format!("{:?}", b)));
    }
}

// ------ ------
//     Model
// ------ ------

// `Model` describes our app state.
struct Model {
    is_loading: bool,
    is_choosing_skill: bool,
    sorted_skills: Vec<Skill>,
    filtered_skills: Vec<Skill>,
    wishes: Vec<(Skill, u8)>,
    filter: String,
    changing_wish_index: Option<usize>,
    page: Page,
    armors: ArmorLists,
    searched_builds: Vec<Build>,
}

// ------ ------
//    Update
// ------ ------

enum Msg {
    SearchBuilds,
    ChangeFilter(String),
    AddWish,
    DeleteWish(IndexWish),
    ChangeWishAmount(IndexWish, AmountSkill),
    ToggleWishModal(Option<IndexWish>),
    ChangeWish(IndexWish, Skill),
    Nothing,
    //ChangePage(Page),
    EndArmorInitialization(ArmorLists),
    EndSearch(Vec<Build>),
}

type IndexWish = usize;
type AmountSkill = u8;

// I'm doing some dangerous manipulations for the sake of performance and
// memory, this is pretty useless. I'll remove them when it will be horrible
fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::ChangeFilter(filter) => {
            model.filter = filter;
            model.filtered_skills = model
                .sorted_skills
                .iter()
                .copied()
                .filter(|s| {
                    !model.wishes.iter().map(|(s, _)| s).any(|s0| s == s0)
                        && format!("{:?}", s)
                            .to_lowercase()
                            .contains(&model.filter.to_lowercase())
                })
                .collect()
        }
        Msg::AddWish => {
            // sometimes a double click can pass through a disabled button
            if model.wishes.len() < Skill::ALL.len() {
                // we can't use the filtered list because
                // the user can't see the filter input text so
                // we use the whole list (ux)
                let new_skill = model
                    .sorted_skills
                    .iter()
                    .filter(|s| !model.wishes.iter().map(|(s, _)| s).any(|s0| *s == s0))
                    .take(1)
                    .last()
                    .copied()
                    .unwrap();
                // when we add a wish we have to be sure that it's removed from the
                // filtered list
                model.filtered_skills.retain(|s| s != &new_skill);
                model.wishes.push((new_skill, 1));
            }
        }
        Msg::DeleteWish(index) => {
            if model.wishes.len() > 1 {
                let (skill, _) = model.wishes.remove(index);
                if format!("{:?}", skill)
                    .to_lowercase()
                    .contains(&model.filter.to_lowercase())
                {
                    model.filtered_skills.push(skill);
                    model.filtered_skills.lexical_sort();
                }
            }
        }
        Msg::ChangeWishAmount(index, amount) => model.wishes[index].1 = amount,
        Msg::ToggleWishModal(index) => {
            model.changing_wish_index = index;
            model.is_choosing_skill ^= true
        }
        Msg::ChangeWish(index, skill) => {
            let to_delete = model
                .filtered_skills
                .iter()
                .position(|x| *x == skill)
                .expect("skill not found in filtered list");
            model.filtered_skills.swap_remove(to_delete);
            model.filtered_skills.push(model.wishes[index].0);
            model.filtered_skills.lexical_sort();

            model.wishes[index] = (skill, 1);

            // toggling modal
            model.changing_wish_index = None;
            model.is_choosing_skill ^= true
        }
        Msg::Nothing => {}
        //Msg::ChangePage(page) => model.page = page,
        Msg::SearchBuilds => {
            model.is_loading = true;
            let wishes = model.wishes.clone();
            let armors = model.armors.clone();
            // I don't understand why there is no animation
            // orders.after_next_render(|_| search_builds(wishes, armors));

            orders
                .perform_cmd(cmds::timeout(10, || search_builds(wishes, armors)))
                .force_render_now();
        }
        Msg::EndArmorInitialization(armors) => {
            model.armors = armors;
            model.page = Page::Wishes
        }
        Msg::EndSearch(builds) => {
            model.searched_builds = builds;
            model.is_loading = false;
            model.page = Page::Results;
        }
    }
}

fn search_builds(wishes: Vec<(Skill, u8)>, armors: ArmorLists) -> Msg {
    Msg::EndSearch(pre_selection_then_brute_force_search(
        &wishes,
        AllArmorSlices {
            helmets: &armors.helmets,
            chests: &armors.chests,
            waists: &armors.waists,
            arms: &armors.arms,
            legs: &armors.legs,
            talismans: &[],
        },
        Gender::Male,
        [0, 0, 0],
    ))
}

// ------ ------
//     View
// ------ ------

// `view` describes what to display.
fn view(model: &Model) -> Node<Msg> {
    let disabled_delete = model.wishes.len() <= 1;
    match model.page {
        Page::Wishes => div![
            C!["container", "has-text-centered"],
            view_modal(
                model.is_choosing_skill,
                &model.filtered_skills,
                model.changing_wish_index
            ),
            view_buttons(model.is_loading, model.wishes.len() == Skill::ALL.len()),
            div![
                style! {
                    St::Display => "inline-block"
                },
                model
                    .wishes
                    .iter()
                    .enumerate()
                    .map(|(index, wish)| view_wish_field(
                        *wish,
                        disabled_delete,
                        index,
                        model.is_loading
                    ))
            ]
        ],
        Page::Results => view_builds(&model.searched_builds),
        Page::ArmorsFetching => div![
            C!["container", "has-text-centered"],
            h1![C!["title"], "Fetching armors..."]
        ],
    }
}

fn view_builds(builds: &[Build]) -> Node<Msg> {
    div![builds.iter().enumerate().map(|(index, build)| div![
        C!["columns", "is-mobile", "is-multiline", "has-text-centered"],
        div![
            C!["column", "is-half-mobile"],
            p![
                C![
                    "notification",
                    if index % 2 == 0 {
                        "is-primary"
                    } else {
                        "is-info"
                    }
                ],
                match build.helmet.as_ref() {
                    Some((a, _)) => &a.name,
                    _ => "None",
                }
            ]
        ],
        div![
            C!["column", "is-half-mobile"],
            p![
                C![
                    "notification",
                    if index % 2 == 0 {
                        "is-primary"
                    } else {
                        "is-info"
                    }
                ],
                match build.chest.as_ref() {
                    Some((a, _)) => &a.name,
                    _ => "None",
                }
            ]
        ],
        div![
            C!["column", "is-half-mobile"],
            p![
                C![
                    "notification",
                    if index % 2 == 0 {
                        "is-primary"
                    } else {
                        "is-info"
                    }
                ],
                match build.arm.as_ref() {
                    Some((a, _)) => &a.name,
                    _ => "None",
                }
            ]
        ],
        div![
            C!["column", "is-half-mobile"],
            p![
                C![
                    "notification",
                    if index % 2 == 0 {
                        "is-primary"
                    } else {
                        "is-info"
                    }
                ],
                match build.waist.as_ref() {
                    Some((a, _)) => &a.name,
                    _ => "None",
                }
            ]
        ],
        div![
            C!["column", "is-half-mobile"],
            p![
                C![
                    "notification",
                    if index % 2 == 0 {
                        "is-primary"
                    } else {
                        "is-info"
                    }
                ],
                match build.leg.as_ref() {
                    Some((a, _)) => &a.name,
                    _ => "None",
                }
            ]
        ],
        div![
            C!["column", "is-half-mobile"],
            p![
                C![
                    "notification",
                    if index % 2 == 0 {
                        "is-primary"
                    } else {
                        "is-info"
                    }
                ],
                match build.talisman.as_ref() {
                    Some((a, _)) => &a.name,
                    _ => "None",
                }
            ]
        ]
    ])]
}

fn view_buttons(is_loading: bool, add_disabled: bool) -> Node<Msg> {
    div![
        C!["field", "is-grouped", "is-grouped-centered"],
        p![
            C!["control"],
            button![
                C!["button", "is-success"],
                attrs! {At::Disabled => (is_loading | add_disabled).as_at_value()},
                span![C!["icon"], i![C!["fas", "fa-plus"]]],
                span!["Add wish"],
                ev(Ev::Click, |_| Msg::AddWish)
            ]
        ],
        p![
            C!["control"],
            button![
                C!["button", "is-info", IF!(is_loading => "is-loading")],
                span![C!["icon"], i![C!["fas", "fa-search"]]],
                span!["Search builds"],
                ev(Ev::Click, |_| Msg::SearchBuilds)
            ]
        ],
    ]
}

fn view_wish_field(
    wish: (Skill, u8),
    disabled_delete: bool,
    index: usize,
    is_loading: bool,
) -> Node<Msg> {
    let (skill, amount) = wish;
    let limit = skill.get_limit();
    div![
        C!["field", "has-addons", "has-addons-right"],
        div![
            C!["control"],
            button![
                C!["button", "is-primary"],
                attrs! {At::Disabled => is_loading.as_at_value()},
                span![C!["icon"], i![C!["fas", "fa-undo"]]],
                span![format!("{:?}", skill)],
                ev(Ev::Click, move |_| Msg::ToggleWishModal(Some(index)))
            ]
        ],
        div![
            C!["control"],
            input![
                C!["input"],
                attrs! {
                    At::Type => "number",
                    At::Value => amount,
                    At::Min => 1,
                    At::Max => limit,
                    At::Disabled => is_loading.as_at_value()
                },
                input_ev(Ev::Blur, move |new_amount| Msg::ChangeWishAmount(
                    index,
                    min(new_amount.parse::<u8>().unwrap_or(amount), limit)
                ))
            ]
        ],
        div![
            C!["control"],
            button![
                C!["button", "is-danger"],
                attrs! {At::Disabled => (is_loading | disabled_delete).as_at_value()},
                span![C!["icon"], i![C!["fas", "fa-trash-alt"]]],
                ev(Ev::Click, move |_| Msg::DeleteWish(index))
            ]
        ],
    ]
}

fn view_filter_input() -> Node<Msg> {
    div![
        C!["field"],
        div![
            C!["control", "has-icons-left", "has-icons-right"],
            input![
                C!["input"],
                attrs! {
                    At::Type => "text",
                    At::Placeholder => "Skill filter"
                },
                input_ev(Ev::Input, Msg::ChangeFilter)
            ],
            span![
                C!["icon", "is-small", "is-left"],
                i![C!["fas", "fa-filter"]]
            ],
        ]
    ]
}

fn view_modal(is_active: bool, skills: &[Skill], changing_wish_index: Option<usize>) -> Node<Msg> {
    div![
        C!["modal", IF!(is_active => "is-active")],
        div![C!["modal-background"]],
        div![
            C!["modal-card"],
            header![
                C!["modal-card-head"],
                p![C!["modal-card-title"], view_filter_input()]
            ],
            section![
                C!["modal-card-body"],
                // performances
                if is_active {
                    div![
                        C!["buttons"],
                        skills.iter().map(|&s| {
                            button![
                                C!["button"],
                                format!("{:?}", s),
                                if let Some(index) = changing_wish_index {
                                    ev(Ev::Click, move |_| Msg::ChangeWish(index, s))
                                } else {
                                    ev(Ev::Click, |_| Msg::Nothing)
                                },
                            ]
                        })
                    ]
                } else {
                    empty![]
                }
            ],
            footer![
                C!["modal-card-foot"],
                button![
                    C!["button", "is-link"],
                    span![C!["icon"], i![C!["fas", "fa-arrow-left"]]],
                    span!["Back"],
                    ev(Ev::Click, |_| Msg::ToggleWishModal(None))
                ],
            ]
        ]
    ]
}

// ------ ------
//     Start
// ------ ------

// (This function is invoked by `init` function in `index.html`.)
#[wasm_bindgen(start)]
pub fn start() {
    console_error_panic_hook::set_once();

    App::start("root", init, update, view);
}
