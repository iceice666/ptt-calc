use std::collections::VecDeque;

use crate::score::{get_charts, get_score, Charts, Scores};
use anyhow::{Ok, Result as anyResult};

fn print_scores(
    mut score_data: Scores,
    range: Option<usize>,
    chart_data: &Charts,
) -> anyResult<Scores> {
    score_data.sort_by(|a, b| b.ptt.partial_cmp(&a.ptt).unwrap());

    let mut ptt: f64 = 0.0;
    for i in {
        match range {
            None => 0..30,
            Some(v) => 0..v,
        }
    } {
        let song = &score_data[i];

        print!("#{}. ", i + 1);

        let key = format!("{}-{}", song.song_id, song.song_difficulty);

        match chart_data.get(&key) {
            None => println!(
                "{} {} {}",
                song.song_id,
                {
                    match song.song_difficulty {
                        0 => "PST",
                        1 => "PRS",
                        2 => "FTR",
                        3 => "BYD",
                        _ => "???",
                    }
                },
                song.score
            ),
            Some(info) => {
                ptt += format!("{:.4}", song.ptt).parse::<f64>()?;

                println!(
                    "{} [{}] \n{:08} {:.2} -> {:.4}",
                    match &info.name_jp {
                        None => &info.name_en,
                        Some(v) => &v,
                    },
                    {
                        match song.song_difficulty {
                            0 => "PST",
                            1 => "PRS",
                            2 => "FTR",
                            3 => "BYD",
                            _ => "???",
                        }
                    },
                    song.score,
                    info.rating as f32 * 0.1,
                    song.ptt
                )
            }
        }
        println!(
            "Perfect: {:4} (+{:4}) Far: {:4} Lost: {:4}",
            song.perfect_count, song.max_perfect_count, song.far_count, song.lost_count,
        );
        println!();
    }

    if range.is_none() {
        println!("B30 AVG: {:.4}", ptt / 30.0);
    }

    Ok(score_data)
}

// CLI
pub fn main(mut args: VecDeque<String>) -> anyResult<()> {
    let chart_data = get_charts()?;
    let score_data = get_score(&chart_data)?;

    if let Some(v) = args.pop_front() {
        if v == "list" {
            let range = match args.pop_front() {
                None => score_data.len(),
                Some(n) => n.parse::<usize>()?,
            };

            let _ = print_scores(score_data, Some(range), &chart_data);
        } else {
            let _ = print_scores(score_data, None, &chart_data);
        }
    }

    Ok(())
}
