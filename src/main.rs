mod score;

use std::fs;

use anyhow::{Ok, Result as anyResult};
use score::{get_score, Score};

fn prelude() -> anyResult<Vec<Score>> {
    // check file exist
    if !fs::metadata("arcsong.db").is_ok() {
        println!(
            "Song database ( called `arcsong.db` ) do not exist! 
Did you delete this file by accident?
You can download it back!
Goto: https://raw.githubusercontent.com/iceice666/ptt-calc/master/arcsong.db"
        )
    }

    let mut data_path = "score.db";

    if fs::metadata("/data/data/moe.low.arc/files/st3").is_ok() {
        data_path = "/data/data/moe.low.arc/files/st3";
        println!("Using arcaea's data\n");
    } else {
        if fs::metadata("score.db").is_ok() {
            println!("Using local data\n");
        } else {
            panic!(
                "Score database ( called `score.db` ) do not exist! 
You have to copy it at /data/data/moe.low.arc/files/st3 ( and you will need your device rooted).
And rename to `score.db`."
            )
        }
    }

    Ok(get_score(data_path)?)
}

fn print_b30(mut score_data: Vec<Score>) -> anyResult<Vec<Score>> {
    score_data.sort_by(|a, b| b.ptt.partial_cmp(&a.ptt).unwrap());

    let mut ptt: f64 = 0.0;
    for i in 0..30 {
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

    println!("B30 AVG: {:.4}", ptt / 30.0);

    Ok(score_data)
}

fn main() -> anyResult<()> {
    let score_data = prelude()?;
    let score_data = print_b30(score_data);

    Ok(())
}
