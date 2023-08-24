use anyhow::Result as anyResult;

// field by arcaea's database
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
}

pub fn get_score() -> anyResult<Vec<Score>> {
    let connection = sqlite::open("score.db")?;

    let mut rows: Vec<Score> = vec![];

    let mut statement = connection.prepare("SELECT * FROM scores;")?;

    while let Ok(sqlite::State::Row) = statement.next() {
        rows.push(Score {
            score: statement.read::<i64, _>("score")?.try_into()?,
            max_perfect_count: statement.read::<i64, _>("shinyPerfectCount")?.try_into()?,
            perfect_count: statement.read::<i64, _>("perfectCount")?.try_into()?,
            far_count: statement.read::<i64, _>("nearCount")?.try_into()?,
            lost_count: statement.read::<i64, _>("missCount")?.try_into()?,
            song_id: statement.read::<String, _>("songId")?,
            song_difficulty: statement.read::<i64, _>("songDifficulty")?.try_into()?,
            health: statement.read::<i64, _>("health")?.try_into()?,
        })
    }

    Ok(rows)
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result as anyResult;

    #[test]
    fn test_get_score() -> anyResult<()> {
        let result = get_score()?;
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
}
