use anyhow::{Ok, Result as anyResult};
use arcaea_data::{get_charts, get_score, prettier_string, ScoreSort, Scores};

fn print_scores(
    score_data: &mut Scores,
    range: Option<usize>,
    curr_ptt: Option<f64>,
) -> anyResult<()> {
    score_data.sort_by_ptt();

    let mut ptt: f64 = 0.0;
    for i in {
        match range {
            None => 0..30,
            Some(v) => 0..v,
        }
    } {
        let song = match score_data.get(i) {
            None => break,
            Some(v) => v,
        };

        print!("#{}. ", i + 1);

        let (body, sptt) = prettier_string(song);
        ptt += sptt;

        println!("{}", body);
    }

    if range.is_none() {
        if let Some(curr_ptt) = curr_ptt {
            println!(
                "Current PTT: {:.2}\nB30 AVG: {:.4}\nCalculated R10: {:.4}",
                curr_ptt,
                ptt / 30.0,
                curr_ptt * 4.0 - ptt / 10.0
            )
        } else {
            println!("B30 AVG: {:.4} ", ptt / 30.0);
        }
    }

    Ok(())
}

// CLI
fn main() -> anyResult<()> {
    let mut score_data = get_score(get_charts()?)?;

    let args: Vec<String> = std::env::args().collect();

    if let Some(v) = args.get(1) {
        match v.as_str() {
            "list" => {
                let range = match args.get(2) {
                    None => score_data.len(),
                    Some(n) => n.parse::<usize>()?,
                };

                print_scores(&mut score_data, Some(range), None)?;
            }
            "b30" => {
                print_scores(
                    &mut score_data,
                    None,
                    match args.get(2) {
                        None => None,
                        Some(v) => Some(v.parse::<f64>()?),
                    },
                )?;
            }
            _ => {}
        }
    }
    Ok(())
}
