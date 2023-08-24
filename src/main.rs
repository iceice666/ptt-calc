mod score;

use std::fs;

use anyhow::Result as anyResult;
use score::get_score;
use score::update_info;

fn main() -> anyResult<()> {
    // check file exist
    if !fs::metadata("arcsong.db").is_ok() {
        println!(
            "Song database ( called `arcsong.db` ) do not exist! 
PTT calculation is tempeorarily disabled.
You can download it at https://github.com/Arcaea-Infinity/ArcaeaSongDatabase"
        )
    }

    let mut data_path = "score.db";

    if fs::metadata("/data/data/moe.low.arc/files/st3").is_ok() {
        data_path = "/data/data/moe.low.arc/files/st3";
    } else {
        if !fs::metadata("score.db").is_ok() {
            panic!(
                "Score database ( called `score.db` ) do not exist! 
You have to copy it at /data/data/moe.low.arc/files/st3 ( and you will need your device be rooted).
And rename to `score.db`."
            )
        }
    }

    let score_data = get_score(data_path)?;
    let mut song_data = update_info(score_data)?;

    song_data.sort_by(|a, b| {
        let mut ptt_a: f64 = -1.0;
        let mut ptt_b: f64 = -1.0;

        if let Some(v) = &a.info {
            ptt_a = (v.rating as f64 + a.ptt_offset) * 0.1;
        }

        if let Some(v) = &b.info {
            ptt_b = (v.rating as f64 + b.ptt_offset) * 0.1;
        }
        ptt_b.partial_cmp(&ptt_a).unwrap()
    });

    let mut ptt: f64 = 0.0;
    for i in 0..30 {
        let song = &song_data[i];

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
                ptt += format!("{:.4}", (info.rating as f64 + song.ptt_offset) * 0.1)
                    .parse::<f64>()?;

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
                    (info.rating as f64 + song.ptt_offset) * 0.1
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

    Ok(())
}
