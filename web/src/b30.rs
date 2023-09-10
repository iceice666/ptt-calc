use arcaea_data::{get_charts, get_score, ScoreSort};
use rocket::response::content;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct ScoreDataForDisplay {
    // Song's id
    song_name: String, // String
    // difficulty
    song_difficulty: String, // 0 ~ 3
    // Just score
    score: String, // 0 ~ 10,002,221
    // perfect count in +- 25ms. (Original called `shinyPerfectCount`)
    max_perfect_count: u16, // 0 ~ 2221
    // perfect count include maxPerfect count
    perfect_count: u16, // 0 ~ 2221
    // Far count (Original called `nearCount`)
    far_count: u16, // 0 ~ 2221
    // Lost count (Original called `missCount`)
    lost_count: u16, // 0 ~ 2221
    //calculated ptt
    ptt: String,
    //chart rating,
    rating: String,
}

fn format_number(number: u32) -> String {
    // 将数字转换为字符串，并在前面添加一个零（如果是7位数字）
    let number_str = if number >= 10_000_000 {
        number.to_string()
    } else {
        format!("0{}", number)
    };

    // 使用chunks方法将字符串按每3个字符分隔，并使用逗号连接
    let formatted_number = number_str
        .chars()
        .rev()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>()
        .join(",")
        .chars()
        .rev()
        .collect::<String>();

    formatted_number
}

fn format_data() -> Vec<ScoreDataForDisplay> {
    let mut score_data = get_score(get_charts().unwrap()).unwrap();
    score_data.sort_by_ptt();
    let best30: Vec<ScoreDataForDisplay> = score_data[0..30]
        .iter()
        .map(|song| ScoreDataForDisplay {
            song_name: {
                match &song.chart_info {
                    Some(info) => match &info.name_jp {
                        None => info.name_en.clone(),
                        Some(v) => v.to_string(),
                    },
                    None => song.song_id.clone(),
                }
            },
            song_difficulty: match song.song_difficulty {
                0 => "PST".to_string(),
                1 => "PRS".to_string(),
                2 => "FTR".to_string(),
                3 => "BYD".to_string(),
                _ => "???".to_string(),
            },
            score: format_number(song.score),
            max_perfect_count: song.max_perfect_count,
            perfect_count: song.perfect_count,
            far_count: song.far_count,
            lost_count: song.lost_count,
            ptt: format!("{:.4}", song.ptt),
            rating: match &song.chart_info {
                None => "???".to_string(),
                Some(v) => v.rating.to_string(),
            },
        })
        .collect();

    best30
}

///////////////////////////////////////////////////////////////////////////////////////////
#[get("/b30")]
pub fn main() -> content::RawHtml<String> {
    content::RawHtml(format!(
        r#"
    <!doctype html>
    <head>
    {}
    </head>
    <body>
    {}
    </body>
    "#,
        get_header(),
        get_b30_html()
    ))
}

///////////////////////////////////////////////////////////////////////////////////////////
fn get_header() -> String {
    r#"
<style>

body {
  background-color: #1e1e2e;
  display: flex;
  justify-content: center;
  align-items: center;
  flex-direction: column;
}

.b30-container {
  display: grid;
  grid-template-columns: repeat(3, 1fr); 
  grid-template-rows: repeat(10, 1fr);    
  gap: 10px; 
}

.b30-score {
  color: #cdd6f4;
  display: flex;
  flex-direction: column;
  padding: 10px; 
}

#song_name {
  color: #b1b2b3;
}

#song_difficulty.PRS-label {color: #74c7ec;}
#song_difficulty.PST-label {color: #a6e3a1;}
#song_difficulty.FTR-label {color: #cba6f7;}
#song_difficulty.BYD-label {color: #f38ba8;}

</style>

"#
    .to_string()
}

///////////////////////////////////////////////////////////////////////////////////////////
fn get_b30_html() -> String {
    format!(r#"<div class="b30-container">{}</div>"#, {
        let mut score_html = "".to_string();
        let best30 = format_data();

        for score in best30.iter() {
            score_html += &format!(
                r#"
<div class="b30-score">
    <div style="display: flex;">
        <span id="song_name">{}</span>
        <span id="song_difficulty" style="margin-left: auto;" class="{}-label">
            {}
        </span>
    </div>


    <span>{}</span>

    <span>{} => {}</span>


    <div style="display: flex; justify-content: space-between;">
        <span>Perfect: {} (+{})</span>
        <span>Far: {}</span>
        <span>Lost: {}</span>
    </div>
</div>

"#,
                score.song_name,
                score.song_difficulty,
                score.song_difficulty,
                score.score,
                score.rating,
                score.ptt,
                score.perfect_count,
                score.max_perfect_count,
                score.far_count,
                score.lost_count
            )
        }
        score_html
    },)
}
