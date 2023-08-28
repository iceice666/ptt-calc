#![allow(non_snake_case)]

use dioxus::prelude::*;

// use anyhow::Result as anyResult;

use crate::score::{get_charts, get_score, Charts};

#[inline_props]
fn generate_scores(cx: Scope, chart_data: Charts) -> Element {
    // Can I return an Err for Result's Err?
    let mut score_data = get_score(chart_data).unwrap();
    score_data.sort_by(|a, b| b.ptt.partial_cmp(&a.ptt).unwrap());

    let charts = score_data.iter().enumerate().map(|(i, score)| {
        let key = format!("{}-{}", score.song_id, score.song_difficulty);
        let difficulty = match score.song_difficulty {
            0 => "PST",
            1 => "PRS",
            2 => "FTR",
            3 => "BYD",
            _ => "???",
        };

        let perfect = score.perfect_count;
        let max_perfect = score.max_perfect_count;
        let far = score.far_count;
        let lost = score.lost_count;

        if let Some(chart) = chart_data.get(&key) {
            let title = match &chart.name_jp {
                None => &chart.name_en,
                Some(v) => v,
            };

            rsx!(
               li{ p {
                    "#{i+1}."
                    }
            br {}
            p {
            "{title} {difficulty}"}
            br {}
            p{"{score.score} {chart.rating} -> {score.ptt}"}
            br{}
            p{"Perfect: {perfect} (+{max_perfect}) Far: {far} Lost: {lost}"}
            br{}
                }
            )
        } else {
            rsx!(
               li{ p {
                    "#{i+1}.\n{score.song_id} {difficulty}\n{score.score}\nPerfect: {perfect} (+{max_perfect}) Far: {far} Lost: {lost}\n"
                }}
            )
        }
    });

    render!(rsx!(
    div{ul { charts }}
    ))
}

// build up applications
pub fn App(cx: Scope) -> Element {
    let chart_data = get_charts().unwrap();

    // let tui_ctx: TuiContext = cx.consume_context().unwrap();

    cx.render(rsx! {
        generate_scores { chart_data: chart_data }
    })
}



