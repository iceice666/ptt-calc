// todo remove deprecated
#![allow(non_snake_case, deprecated)]

use dioxus::prelude::*;
use dioxus_tui::TuiContext;

use anyhow::Result as anyResult;

use crate::score::{get_charts, get_score, Charts};

fn generate_scores<'a>(cx: Scope<'a>, chart_data: &'a Charts) -> Element<'a> {
    // Can I return an Err for Result's Err?
    let score_data = get_score(chart_data).unwrap();

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
                li {
                    "#{i}.\n
                    {title} {difficulty}\n
                    {score.score} {chart.rating} -> {score.ptt}\n
                    Perfect: {perfect} (+{max_perfect}) Far: {far} Lost: {lost}"
                }
            )
        } else {
            rsx!(
                li {
                    "#{i}.\n
                    {score.song_id} {difficulty}\n
                    {score.score}\n
                    Perfect: {perfect} (+{max_perfect}) Far: {far} Lost: {lost}"
                }
            )
        }
    });

    render!(ol { charts })
}

pub fn main() {
    dioxus_tui::launch_cfg(
        App,
        dioxus_tui::Config::new()
            .without_ctrl_c_quit()
            // Some older terminals only support 16 colors or ANSI colors
            // If your terminal is one of these, change this to BaseColors or ANSI
            .with_rendering_mode(dioxus_tui::RenderingMode::Rgb),
    );
}

fn App(cx: Scope) -> Element {
    let chart_data = get_charts();

    let tui_ctx: TuiContext = cx.consume_context().unwrap();
    cx.render(rsx! {
        generate_scores {}
    })
}
