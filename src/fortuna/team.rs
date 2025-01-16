use crate::shared::team::{EN::*, ES::*, IT::*, PL::*, *};

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

pub fn football_foreign_teams() -> Vec<(&'static str, Team)> {
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
