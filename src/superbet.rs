use crate::bookmaker;
use crate::utils::{self, browser::Browser};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "superbet";
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
                    "https://superbet.pl/zaklady-bukmacherskie/live",
                    fantoccini::Locator::Css(
                        r#"button[id="onetrust-accept-btn-handler"]"#,
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
        let match_selector = Selector::parse("div.event-card").unwrap();
        let odds_selector =
            Selector::parse("span.odd-button__odd-value-new").unwrap();
        let team1_selector =
            Selector::parse("div.e2e-event-team1-name").unwrap();
        let team2_selector =
            Selector::parse("div.e2e-event-team2-name").unwrap();
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
                let team1 = x.select(&team1_selector).next().unwrap();
                let team2 = x.select(&team2_selector).next().unwrap();
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
