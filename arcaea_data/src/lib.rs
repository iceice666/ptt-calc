use anyhow::Result as anyResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ScoreData {
    // Song's id
    pub song_id: String, // String
    // 0 for past, 1 for present, 2 for future, 3 for beyond
    pub song_difficulty: u8, // 0 ~ 3
    // Just score
    pub score: u32, // 0 ~ 10,002,221
    // perfect count in +- 25ms. (Original called `shinyPerfectCount`)
    pub max_perfect_count: u16, // 0 ~ 2221
    // perfect count include maxPerfect count
    pub perfect_count: u16, // 0 ~ 2221
    // Far count (Original called `nearCount`)
    pub far_count: u16, // 0 ~ 2221
    // Lost count (Original called `missCount`)
    pub lost_count: u16, // 0 ~ 2221
    //calculated ptt
    pub ptt: f64,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ChartData {
    pub name_en: String,
    pub name_jp: Option<String>,
    pub rating: u8,
}

pub type Scores = Vec<ScoreData>;
pub type Charts = HashMap<String, ChartData>;

pub trait ScoreSort {
    fn sort_by_ptt(&mut self);
}

impl ScoreSort for Scores {
    fn sort_by_ptt(&mut self) {
        self.sort_by(|a, b| b.ptt.partial_cmp(&a.ptt).unwrap());
    }
}

/// pass a <&Charts> to calculate ptt
pub fn get_score(chart_info: &Charts) -> anyResult<Scores> {
    // check database file exist

    let mut data_path = "score.db";

    if fs::metadata("/data/data/moe.low.arc/files/st3").is_ok() {
        data_path = "/data/data/moe.low.arc/files/st3";
        // println!("Using arcaea's data\n");
    } else if fs::metadata("score.db").is_ok() {
        // println!("Using local data\n");
    } else {
        anyhow::bail!(
            "Score database ( called `score.db` ) do not exist! 
You have to copy it at /data/data/moe.low.arc/files/st3 ( and you will need your device rooted).
And rename to `score.db`."
        )
    }

    let connection = sqlite::open(data_path)?;

    let mut rows: Scores = vec![];

    let mut statement = connection.prepare("SELECT * FROM scores;")?;

    while let Ok(sqlite::State::Row) = statement.next() {
        let score = statement.read::<i64, _>("score")?.try_into()?;
        let song_id = statement.read::<String, _>("songId")?;
        let song_difficulty = statement.read::<i64, _>("songDifficulty")?.try_into()?;
        let max_perfect_count = statement.read::<i64, _>("shinyPerfectCount")?.try_into()?;
        let perfect_count = statement.read::<i64, _>("perfectCount")?.try_into()?;
        let far_count = statement.read::<i64, _>("nearCount")?.try_into()?;
        let lost_count = statement.read::<i64, _>("missCount")?.try_into()?;

        let info = chart_info.get(&format!("{}-{}", song_id.clone(), song_difficulty));

        let ptt = {
            let p = ({
                match info {
                    None => 0.0 as f64,
                    Some(v) => v.rating as f64,
                }
            } + {
                if score >= 10_000_000 {
                    20 as f64
                } else if score >= 9_800_000 {
                    (score - 9800000 + 200000) as f64 / 20000 as f64
                } else {
                    (score as i64 - 9500000) as f64 / 30000 as f64
                }
            }) * 0.1;

            if p < 0.0 {
                0.0
            } else {
                p
            }
        };

        rows.push(ScoreData {
            ptt,
            score,
            song_id,
            song_difficulty,
            max_perfect_count,
            perfect_count,
            far_count,
            lost_count,
        })
    }

    Ok(rows)
}

/// provide a way to check `argsong.db` existt
fn check_arcsong() -> anyResult<()> {
    // check database file exist
    if fs::metadata("arcsong.db").is_ok() {
        Ok(())
    } else {
        anyhow::bail!(
            "Song database ( called `arcsong.db` ) do not exist! 
Did you delete this file by accident?
You can download it back!
Goto: https://raw.githubusercontent.com/iceice666/ptt-calc/master/arcsong.db"
        )
    }
}

/// get all charts' info in `arcsong.db`
pub fn get_charts() -> anyResult<HashMap<String, ChartData>> {
    let connection = sqlite::open("arcsong.db")?;

    // create a map to save song info
    let mut map: Charts = HashMap::new();

    // prepare to iter through database
    let mut statement = connection.prepare("SELECT * FROM charts;")?;

    // iteration
    while let Ok(sqlite::State::Row) = statement.next() {
        let rating = statement.read::<i64, _>("rating")?.try_into()?;
        let name_en = statement.read::<String, _>("name_en")?;
        let name_jp = {
            let t = statement.read::<String, _>("name_jp")?;
            if t == "" {
                None
            } else {
                Some(t)
            }
        };

        let song_id = statement.read::<String, _>("song_id")?;
        let rating_class: u8 = statement.read::<i64, _>("rating_class")?.try_into()?;

        map.insert(
            format!("{}-{}", song_id.clone(), rating_class),
            ChartData {
                name_en,
                name_jp,
                rating,
            },
        );
    }
    Ok(map)
}

pub fn prettier_string(
    song: &ScoreData,
    chart_data: &Charts,
) -> (String, f64) {
    let key = format!("{}-{}", song.song_id, song.song_difficulty);

    match chart_data.get(&key) {
        None => (
            format!(
                "{} {} {}\r\nPerfect: {:4} (+{:4}) Far: {:4} Lost: {:4}\r\n",
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
                song.score,
                song.perfect_count,
                song.max_perfect_count,
                song.far_count,
                song.lost_count,
            ),
            0.0f64,
        ),
        Some(info) => (
            format!(
                "{} [{}] \n{:08} {:.1} -> {:.4}\r\nPerfect: {:4} (+{:4}) Far: {:4} Lost: {:4}\r\n",
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
                song.ptt,
                song.perfect_count,
                song.max_perfect_count,
                song.far_count,
                song.lost_count,
            ),
            song.ptt,
        ),
    }
}

fn _print_songs_ptt() -> anyResult<()> {
    let connection = sqlite::open("arcsong.db")?;

    let mut statement = connection.prepare("SELECT * FROM charts;")?;

    while let Ok(sqlite::State::Row) = statement.next() {
        let song_id = statement.read::<String, _>("song_id")?;
        let rating_class: u8 = statement.read::<i64, _>("rating_class")?.try_into()?;
        let rating: u8 = statement.read::<i64, _>("rating")?.try_into()?;

        println!("\"{}-{}\" : {} ", song_id, rating_class, rating);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Ok, Result as anyResult};

    #[test]
    fn print_songs_ptt() -> anyResult<()> {
        let _ = _print_songs_ptt();
        Ok(())
    }

    #[test]
    fn print_score() -> anyResult<()> {
        let result = get_score(&get_charts()?)?;
        println!(
            "        song         |  score   |   perfect   | far  | lost | health | difficulty "
        );
        println!(
            "---------------------|----------|-------------|------|------|--------|------------"
        );

        for score in result.iter() {
            println!(
                "{:20} | {:8} | {:11} | {:4} | {:4} |  {:3}",
                score.song_id.clone(),
                score.score,
                {
                    let max = score.max_perfect_count;
                    let perfect = score.perfect_count;

                    format!("{:4}{:7}", perfect, format!("(+{})", max))
                },
                score.far_count,
                score.lost_count,
                {
                    match score.song_difficulty {
                        0 => "PST",
                        1 => "PRS",
                        2 => "FTR",
                        3 => "BYD",
                        _ => "???",
                    }
                }
            );
        }

        Ok(())
    }
}
