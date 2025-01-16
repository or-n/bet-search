use crate::bookmaker;
use crate::shared::team::{EN::*, ES::*, IT::*, PL::*, *};
use crate::utils::{self, browser::Browser};

pub struct Book;

impl bookmaker::Name for Book {
    const NAME: &'static str = "efortuna";
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
                    "https://live.efortuna.pl/",
                    fantoccini::Locator::Css(
                        r#"button[id="cookie-consent-button-accept"]"#,
                    ),
                )
                .await?,
            )),
            Err(connect_error) => Err(connect_error),
        })
    }
}

impl Into<String> for &LivePage {
    fn into(self) -> String {
        self.0.clone()
    }
}

use bookmaker::Error;

impl bookmaker::SportBets for LivePage {
    fn sport_bets(
        &self,
    ) -> Result<Vec<(bookmaker::Teams, bookmaker::Odds)>, Error> {
        use scraper::Selector;
        let team1 = Selector::parse("div.live-match-info__team--1").unwrap();
        let team2 = Selector::parse("div.live-match-info__team--2").unwrap();
        utils::sport_bets::extract(
            &self.0,
            Selector::parse("div.live-match").unwrap(),
            Selector::parse("span.odds_button__value").unwrap(),
            |x| {
                Ok([
                    x.select(&team1).next().ok_or(Error::MissingTeam1)?,
                    x.select(&team2).next().ok_or(Error::MissingTeam2)?,
                ])
            },
        )
    }
}

pub fn football_teams() -> Vec<(&'static str, PL)> {
    vec![
        ("U.Skierniewice", USkierniewice),
        ("Ol.Grudziądz", OlGrudziadz),
        ("ŁKS Łódź", LKSLodz),
        ("Legia W.", LegiaWarszawa),
        ("R.Radom", RRadom),
        ("Górnik Z.", GornikZ),
        ("Lech P.", LechP),
        ("Lechia G.", LechiaG),
        ("Śląsk W.", SlaskW),
        ("S.Mielec", SMielec),
        ("Raków Cz.", RakowCz),
        ("M.Lublin", MLublin),
        ("Piast G.", PiastG),
        ("P.Niepolomice", PNiepolomice),
        ("Zag.Lublin", ZagLublin),
        ("Korona K.", KoronaK),
        ("Pogoń Sz.", PogonSz),
        ("S.Stalowa Wola", SStalowaWola),
        ("Miedź L.", MiedzL),
        ("Polonia W.", PoloniaW),
        ("Wisła K.", WislaK),
        ("K.Kołobrzeg", KKolobrzeg),
        ("Ch.Głogłów", ChGlogow),
        ("S.Rzeszów", SRzeszow),
        ("G.Łęczna", GLeczna),
        ("P.Siedlce", PSiedlce),
        ("O.Opole", OOpole),
        ("Z.Pruszków", ZPruszkow),
        ("W.Płock", WPlock),
        ("Warta Poz.", WartaPoz),
        ("Arka G.", ArkaG),
    ]
}

fn football_foreign_teams() -> Vec<(&'static str, Team)> {
    use Team::*;
    vec![
        ("Man.City", EN(ManCity)),
        ("Man.United", EN(ManUnited)),
        ("Q.P.R.", EN(QPR)),
        ("Dagenham and Red.", EN(DagenhamAndRed)),
        ("Sheffield Wed.", EN(SheffieldWed)),
        ("Oxford Utd.", EN(OxfordUtd)),
        ("Real M.", ES(RealMadrid)),
    ]
}

pub fn football_foreign_team_normal() -> Vec<(&'static str, Team)> {
    use Team::*;
    vec![
        ("Bilbao", ES(Bilbao)),
        ("Fiorentina", IT(Fiorentina)),
        ("Empoli", IT(Empoli)),
    ]
}
