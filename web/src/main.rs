#[macro_use]
extern crate rocket;

use arcaea_data::{get_charts, get_score, ScoreSort};
use rocket::fs::{relative, FileServer};
use rocket_dyn_templates::{context, Template};
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

///////////////////////////////////////////////////////////////
#[get("/")]
fn root() -> &'static str {
    "Hello, world!"
}

#[get("/b30")]
fn b30() -> Template {
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

    Template::render("b30", context! { scores:best30})
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![root, b30])
        .mount("/assets", FileServer::from(relative!("/assets")))
        .attach(Template::fairing())
}
