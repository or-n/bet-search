use super::book::{Error, Odds, Teams};

pub fn extract(
    page: &String,
    bet: scraper::Selector,
    odds: scraper::Selector,
    teams: impl Fn(
        scraper::ElementRef<'_>,
    ) -> Result<[scraper::ElementRef; 2], Error>,
) -> Result<Vec<(Teams, Odds)>, Error> {
    let document = scraper::Html::parse_document(page);
    let team = |x: scraper::ElementRef| x.inner_html().trim().to_string();
    let ratio_result = |x: scraper::ElementRef| {
        x.inner_html()
            .trim()
            .parse::<f32>()
            .map_err(|_| x.inner_html())
    };
    document
        .select(&bet)
        .map(|x| {
            let [team1, team2] = teams(x)?;
            let teams = Teams {
                team1: team(team1),
                team2: team(team2),
            };
            let odds = x.select(&odds).map(ratio_result).collect();
            Ok((teams, odds))
        })
        .collect()
}
