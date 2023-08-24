use anyhow::Result as anyResult;
use std::collections::HashMap;

// field by arcaea's database
#[derive(Debug)]
pub struct Song {
    pub song_id: String,
    pub name_en: String,
    pub name_jp: Option<String>,
    pub rating_class: u8,
    pub rating: u8,
}

pub fn get_song(requires: Option<Vec<(String, u8)>>) -> anyResult<HashMap<String, Song>> {
    let connection = sqlite::open("arcsong.db")?;

    let query: String;

    match requires {
        Some(keys) => {
            query = {
                let mut r = ("SELECT * FROM charts WHERE 0 ").to_string();
                for (song_id, difficulty) in keys {
                    r.push_str(&format!(
                        "OR ( song_id = '{}' AND rating_class = {} )",
                        song_id, difficulty
                    ))
                }

                r
            };
        }
        None => {
            query = "SELECT * FROM charts ".to_string();
        }
    }

    let mut statement = connection.prepare(query)?;

    let mut map = HashMap::new();
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
        let rating_class = statement.read::<i64, _>("rating_class")?.try_into()?;
        let rating = statement.read::<i64, _>("rating")?.try_into()?;

        map.insert(
            format!("{}-{}", song_id.clone(), rating_class),
            Song {
                song_id,
                name_en,
                name_jp,
                rating_class,
                rating,
            },
        );
    }

    Ok(map)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result as anyResult;

    #[test]
    fn test_get_song() -> anyResult<()> {
        let result = get_song(None)?;
        println!("        song         |  name en   |   name jp   | rating | difficulty ");
        println!("---------------------|------------|-------------|--------|------------");
        for (_, song) in result.iter() {
            println!(
                "{:20} | {:35} | {:20} | {:4} | {:3}",
                song.song_id,
                song.name_en,
                song.name_jp.clone().unwrap(),
                {
                    match song.rating_class {
                        0 => "PST",
                        1 => "PRS",
                        2 => "FTR",
                        3 => "BYD",
                        _ => "???",
                    }
                },
                song.rating / 10
            );
        }

        Ok(())
    }

    #[test]
    fn test_get_song_with_given() -> anyResult<()> {
        let requires: Vec<(String, u8)> = vec![
            ("ii".to_string(), 0),
            ("ifi".to_string(), 1),
            ("aegleseeker".to_string(), 2),
            ("testify".to_string(), 3),
        ];

        let result = get_song(Some(requires))?;
        println!("        song         |  name en   |   name jp   | rating | difficulty ");
        println!("---------------------|------------|-------------|--------|------------");
        for (id, song) in result.iter() {
            println!(
                "{} | {} | {} | {} | {} | {}",
                id,
                song.song_id,
                song.name_en,
                song.name_jp.clone().unwrap(),
                {
                    match song.rating_class {
                        0 => "PST",
                        1 => "PRS",
                        2 => "FTR",
                        3 => "BYD",
                        _ => "???",
                    }
                },
                song.rating / 10
            );
        }

        Ok(())
    }
}
