mod score;
mod song;

use std::fs;

use anyhow::Result as anyResult;
use score::get_score;
use song::get_song;

fn main() -> anyResult<()> {
    // check file exist
    if !fs::metadata("arcsong.db").is_ok() {
        panic!(
            "Song database ( called `arcsong.db` ) do not exist! 
You can download it at https://github.com/Arcaea-Infinity/ArcaeaSongDatabase"
        )
    }
    if !fs::metadata("score.db").is_ok() {
        panic!(
            "Score database ( called `score.db` ) do not exist! 
You can download it at https://github.com/Arcaea-Infinity/ArcaeaSongDatabase"
        )
    }

    // Get local score data
    let score_data = get_score()?;

    let song_data: Vec<(String, u8)> = score_data
        .iter()
        .map(|score| (score.song_id.clone(), score.song_difficulty))
        .collect();

    let song_data = get_song(Some(song_data))?;

    for score in score_data.iter() {
        let song = song_data.get(&format!("{}-{}", score.song_id, score.song_difficulty));
        println!(
            "{} {:8} {} \n Perfect: {} (+{}) Far: {} Lost: {}\n",
            {
                match song {
                    Some(_) => match &song.unwrap().name_jp {
                        Some(v) => v,
                        None => &song.unwrap().name_en,
                    },
                    None => &score.song_id,
                }
            },
            score.score,
            {
                match song {
                    Some(_) => format!(
                        "{:.2} -> {:.2}",
                        song.unwrap().rating as f64 / 10.0,
                        (song.unwrap().rating as f64 + {
                            if score.score >= 10_000_000 {
                                20 as f64
                            } else if score.score >= 9_800_000 {
                                (score.score - 9800000 + 200000) as f64 / 20000 as f64
                            } else {
                                (score.score as i64 - 9500000) as f64 / 30000 as f64
                            }
                        }) * 0.1
                    ),

                    None => "".to_string(),
                }
            },
            score.perfect_count,
            score.max_perfect_count,
            score.far_count,
            score.lost_count
        )
    }

    Ok(())
}
