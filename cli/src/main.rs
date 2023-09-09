use anyhow::{Ok, Result as anyResult};
use arcaea_data::{get_charts, get_score, prettier_string, Charts, ScoreSort, Scores};

fn print_scores(
    score_data: &mut Scores,
    range: Option<usize>,
    chart_data: &Charts,
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

        let (body, sptt) = prettier_string(song, chart_data);
        ptt += sptt;

        println!("{}", body);
    }

    if range.is_none() {
        println!("B30 AVG: {:.4}", ptt / 30.0);
    }

    Ok(())
}

// CLI
fn main() -> anyResult<()> {
    let chart_data = get_charts()?;
    let mut score_data = get_score(&chart_data)?;

    let args: Vec<String> = std::env::args().collect();

    if let Some(v) = args.get(1) {
        match v.as_str() {
            "list" => {
                let range = match args.get(2) {
                    None => score_data.len(),
                    Some(n) => n.parse::<usize>()?,
                };

                print_scores(&mut score_data, Some(range), &chart_data)?;
            }
            _ => {}
        }
    } else {
        print_scores(&mut score_data, None, &chart_data)?;
    }

    Ok(())
}
