use std::collections::HashMap;

use anyhow::Result as anyResult;

use crate::ptt::get_ptts;

#[derive(Debug)]
pub struct Score {
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
    // Song's id
    pub song_id: String, // String
    // 0 for past, 1 for present, 2 for future, 3 for beyond
    pub song_difficulty: u8, // 0 ~ 3
    // remains health on clear
    pub health: i8, // ~ 100, negative value unknown right now
    //calculated ptt
    pub ptt: f64,
    // song's ptt
    pub song_ptt: u8,
    // song's info
    pub info: Option<Song>,
}

pub fn get_score(path: &str) -> anyResult<Vec<Score>> {
    let connection = sqlite::open(path)?;

    let mut rows: Vec<Score> = vec![];

    let mut statement = connection.prepare("SELECT * FROM scores;")?;

    let ptt_map = get_ptts();

    while let Ok(sqlite::State::Row) = statement.next() {
        let score = statement.read::<i64, _>("score")?.try_into()?;
        let song_id= statement.read::<String, _>("songId")?;
        let song_difficulty= statement.read::<i64, _>("songDifficulty")?.try_into()?;
        let song_ptt = * {ptt_map.get(
           & format!("{}-{}",song_id,song_difficulty)
        ).unwrap_or(&0)  };

        rows.push(Score {
            score,
            max_perfect_count: statement.read::<i64, _>("shinyPerfectCount")?.try_into()?,
            perfect_count: statement.read::<i64, _>("perfectCount")?.try_into()?,
            far_count: statement.read::<i64, _>("nearCount")?.try_into()?,
            lost_count: statement.read::<i64, _>("missCount")?.try_into()?,
            health: statement.read::<i64, _>("health")?.try_into()?,
            ptt:{ song_ptt as f64 }+{
                if score >= 10_000_000 {
                    20 as f64
                } else if score >= 9_800_000 {
                    (score - 9800000 + 200000) as f64 / 20000 as f64
                } else {
                    let p = (score as i64 - 9500000) as f64 / 30000 as f64;

                    if p < 0.0 { 0.0 } else { p}
                }
            },
            info: None,
            song_id,
            song_difficulty,
                song_ptt
        })
    }

    Ok(rows)
}

#[derive(Debug)]
pub struct Song {
    pub name_en: String,
    pub name_jp: Option<String>,
}

/// It takes a Vec<Score> as input, update items' [Score::info] value.
pub fn update_info(mut scores: Vec<Score>) -> anyResult<Vec<Score>> {
    // setup database
    let connection = sqlite::open("arcsong.db")?;

    // create SQL expression
    let mut query = ("SELECT * FROM charts WHERE 0 ").to_string();
    for score in scores.iter() {
        query.push_str(&format!(
            "OR ( song_id = '{}' AND rating_class = {} )",
            score.song_id, score.song_difficulty
        ))
    }

    // create a map to save song info
    let mut map: HashMap<String, Song> = HashMap::new();

    // prepare to iter through database
    let mut statement = connection.prepare(query)?;

    // iteration
    while let Ok(sqlite::State::Row) = statement.next() {
        let song_id = statement.read::<String, _>("song_id")?;
        let name_en = statement.read::<String, _>("name_en")?;
        let name_jp = {
            let t = statement.read::<String, _>("name_jp")?;
            if t == "" {
                None
            } else {
                Some(t)
            }
        };
        let rating_class: u8 = statement.read::<i64, _>("rating_class")?.try_into()?;

        map.insert(
            format!("{}-{}", song_id.clone(), rating_class),
            Song {
                name_en,
                name_jp,
            },
        );
    }

    // Update each item's [Score::info]
    for score in scores.iter_mut() {
        score.info = map.remove(&format!("{}-{}", score.song_id, score.song_difficulty));
    }

    Ok(scores)
}


    fn print_songs_ptt() -> anyResult<()> {
        let connection = sqlite::open("arcsong.db")?;


        let mut statement = connection.prepare("SELECT * FROM charts;")?;

        while let Ok(sqlite::State::Row) = statement.next()  {
            
        let song_id = statement.read::<String, _>("song_id")?;
        let rating_class : u8= statement.read::<i64, _>("rating_class")?.try_into()?;
        let rating: u8 = statement.read::<i64, _>("rating")?.try_into()?;

        println!("map.insert( \"{}-{}\", {} );", song_id,rating_class, rating); 

        }

        Ok(())
    }


#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{Result as anyResult, Ok};

    #[test]
    fn test_print_songs_ptt() -> anyResult<()> {
        let _ =print_songs_ptt();
        Ok(())
    }


    #[test]
    fn test_get_score() -> anyResult<()> {
        let result = get_score("score.db")?;
        println!(
            "        song         |  score   |   perfect   | far  | lost | health | difficulty "
        );
        println!(
            "---------------------|----------|-------------|------|------|--------|------------"
        );

        for score in result.iter() {
            println!(
                "{:20} | {:8} | {:11} | {:4} | {:4} | {:6} | {:3}",
                score.song_id.clone(),
                score.score,
                {
                    let max = score.max_perfect_count;
                    let perfect = score.perfect_count;

                    format!("{:4}{:7}", perfect, format!("(+{})", max))
                },
                score.far_count,
                score.lost_count,
                score.health,
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

    #[test]
    fn test_update_info() -> anyResult<()> {
        let score_data = get_score("score.db")?;
        let mut song_data = update_info(score_data)?;

        song_data.sort_by(|a, b| {
            a.ptt.partial_cmp(&b.ptt).unwrap()
        });

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
                Some(info) => println!(
                    "{} [{}] {:8} {:.2} -> {:.2}",
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
                    song.song_ptt as f32 * 0.1,
                    song.ptt
                ),
            }
            println!(
                "Perfect: {} (+{}) Far: {} Lost: {}",
                song.perfect_count, song.max_perfect_count, song.far_count, song.lost_count,
            );
            println!();
        }

        Ok(())
    }





}
