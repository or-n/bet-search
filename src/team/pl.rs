pub enum PL {
    USkierniewice,
    RuchChorzow,
    OlGrudziadz,
    Jagiellonia,
    LKSLodz,
    LegiaWarszawa,
    RRadom,
    GKSKatowice,
    GornikZ,
    LechP,
    LechiaG,
    SlaskW,
    Widzew,
    SMielec,
    RakowCz,
    MLublin,
    PiastG,
    Cracovia,
    PNiepolomice,
    ZagLublin,
    KoronaK,
    PogonSz,
    SStalowaWola,
    MiedzL,
    PoloniaW,
    WislaK,
    GKSTychy,
    KKolobrzeg,
    Nieciecza,
    ChGlogow,
    SRzeszow,
    GLeczna,
    PSiedlce,
    OOpole,
    ZPruszkow,
    WPlock,
    WartaPoz,
    ArkaG,
}

pub fn normal() -> Vec<(&'static str, PL)> {
    use PL::*;
    vec![
        ("Jagiellonia", Jagiellonia),
        ("Widzew", Widzew),
        ("Cracovia", Cracovia),
        ("Niecziecza", Nieciecza),
    ]
}

pub fn space() -> Vec<(&'static str, PL)> {
    use PL::*;
    vec![
        ("Ruch Chorzów", RuchChorzow),
        ("GKS Katowice", GKSKatowice),
        ("GKS Tychy", GKSTychy),
        ("ŁKS Łódź", LKSLodz),
    ]
}
