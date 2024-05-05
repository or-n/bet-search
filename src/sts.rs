use crate::bookmaker;
use crate::utils::{self, browser::Browser};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "sts";
}

pub struct LivePage(String);

impl utils::download::Download for Browser<Book> {
    type Output = Result<LivePage, utils::browser::Error>;
    type Error = fantoccini::error::CmdError;

    async fn download(&self) -> Result<Self::Output, Self::Error> {
        Ok(match utils::browser::client(self.port).await {
            Ok(client) => Ok(LivePage(
                utils::download::download(
                    client,
                    "https://www.sts.pl/live",
                    fantoccini::Locator::Css(
                        r#"button[id="CybotCookiebotDialogBodyLevelButtonLevelOptinAllowAll"]"#,
                    ),
                )
                .await?,
            )),
            Err(connect_error) => Err(connect_error),
        })
    }
}

impl bookmaker::SportBets for LivePage {
    fn sport_bets(
        &self,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, ()> {
        use scraper::{Html, Selector};
        let document = Html::parse_document(&self.0);
        let match_selector =
            Selector::parse("div.match-tile-container").unwrap();
        let odds_selector =
            Selector::parse("span.odds-button__odd-value").unwrap();
        let team_selector =
            Selector::parse("div.match-tile-scoreboard-team__name span")
                .unwrap();
        let team = |x: scraper::ElementRef| x.inner_html().trim().to_string();
        let ratio_result = |x: scraper::ElementRef| {
            x.inner_html()
                .trim()
                .parse::<f32>()
                .map_err(|_| x.inner_html())
        };
        let matches: Vec<_> = document
            .select(&match_selector)
            .map(|x| {
                let mut teams = x.select(&team_selector);
                let team1 = teams.next().unwrap();
                let team2 = teams.next().unwrap();
                let teams = bookmaker::Teams {
                    team1: team(team1),
                    team2: team(team2),
                };
                let odds = x.select(&odds_selector).map(ratio_result).collect();
                (teams, odds)
            })
            .collect();
        Ok(matches)
    }
}
