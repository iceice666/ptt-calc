use std::collections::VecDeque;

use crate::score::{get_score, Score};
use anyhow::{Ok, Result as anyResult};

fn print_scores(mut score_data: Vec<Score>, range: Option<usize>) -> anyResult<Vec<Score>> {
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

        match &song.info {
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
    let score_data = get_score()?;

    if let Some(v) = args.pop_front() {
        if v == "list" {
            let range = match args.pop_front() {
                None => score_data.len(),
                Some(n) => n.parse::<usize>()?,
            };

            let _ = print_scores(score_data, Some(range));
        } else {
            let _ = print_scores(score_data, None);
        }
    }

    Ok(())
}