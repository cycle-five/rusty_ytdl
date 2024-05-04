mod youtube;

pub use youtube::{
    Channel, EmbedOptions, Playlist, PlaylistSearchOptions, RequestOptions, SearchOptions,
    SearchResult, SearchType, Video, YouTube,
};

#[derive(Debug, Clone)]
pub enum LanguageTags {
    AF,
    AM,
    AR,
    AS,
    AZ,
    BE,
    BG,
    BN,
    BS,
    CA,
    CS,
    DA,
    DE,
    EL,
    EnGB,
    EnIN,
    EN,
    ES,
    Es419,
    EsUS,
    ET,
    EU,
    FA,
    FI,
    FIL,
    FrCA,
    FR,
    GL,
    GU,
    HI,
    HR,
    HU,
    HY,
    ID,
    IS,
    IT,
    IW,
    JA,
    KA,
    KK,
    KM,
    KN,
    KO,
    KY,
    LO,
    LT,
    LV,
    MK,
    ML,
    MN,
    MR,
    MS,
    MY,
    NO,
    NE,
    NL,
    OR,
    PA,
    PL,
    PT,
    PtPT,
    RO,
    RU,
    SI,
    SK,
    SL,
    SQ,
    SrLATN,
    SR,
    SV,
    SW,
    TA,
    TE,
    TH,
    TR,
    UK,
    UR,
    UZ,
    VI,
    ZhCN,
    ZhHK,
    ZhTW,
    ZU,
}

impl std::fmt::Display for LanguageTags {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageTags::AF => write!(f, "af"),
            LanguageTags::AM => write!(f, "am"),
            LanguageTags::AR => write!(f, "ar"),
            LanguageTags::AS => write!(f, "as"),
            LanguageTags::AZ => write!(f, "az"),
            LanguageTags::BE => write!(f, "be"),
            LanguageTags::BG => write!(f, "bg"),
            LanguageTags::BN => write!(f, "bn"),
            LanguageTags::BS => write!(f, "bs"),
            LanguageTags::CA => write!(f, "ca"),
            LanguageTags::CS => write!(f, "cs"),
            LanguageTags::DA => write!(f, "da"),
            LanguageTags::DE => write!(f, "de"),
            LanguageTags::EL => write!(f, "el"),
            LanguageTags::EnGB => write!(f, "en-GB"),
            LanguageTags::EnIN => write!(f, "en-IN"),
            LanguageTags::EN => write!(f, "en"),
            LanguageTags::ES => write!(f, "es"),
            LanguageTags::Es419 => write!(f, "es-419"),
            LanguageTags::EsUS => write!(f, "es-US"),
            LanguageTags::ET => write!(f, "et"),
            LanguageTags::EU => write!(f, "eu"),
            LanguageTags::FA => write!(f, "fa"),
            LanguageTags::FI => write!(f, "fi"),
            LanguageTags::FIL => write!(f, "fil"),
            LanguageTags::FrCA => write!(f, "fr-CA"),
            LanguageTags::FR => write!(f, "fr"),
            LanguageTags::GL => write!(f, "gl"),
            LanguageTags::GU => write!(f, "gu"),
            LanguageTags::HI => write!(f, "hi"),
            LanguageTags::HR => write!(f, "hr"),
            LanguageTags::HU => write!(f, "hu"),
            LanguageTags::HY => write!(f, "hy"),
            LanguageTags::ID => write!(f, "id"),
            LanguageTags::IS => write!(f, "is"),
            LanguageTags::IT => write!(f, "it"),
            LanguageTags::IW => write!(f, "iw"),
            LanguageTags::JA => write!(f, "ja"),
            LanguageTags::KA => write!(f, "ka"),
            LanguageTags::KK => write!(f, "kk"),
            LanguageTags::KM => write!(f, "km"),
            LanguageTags::KN => write!(f, "kn"),
            LanguageTags::KO => write!(f, "ko"),
            LanguageTags::KY => write!(f, "ky"),
            LanguageTags::LO => write!(f, "lo"),
            LanguageTags::LT => write!(f, "lt"),
            LanguageTags::LV => write!(f, "lv"),
            LanguageTags::MK => write!(f, "mk"),
            LanguageTags::ML => write!(f, "ml"),
            LanguageTags::MN => write!(f, "mn"),
            LanguageTags::MR => write!(f, "mr"),
            LanguageTags::MS => write!(f, "ms"),
            LanguageTags::MY => write!(f, "my"),
            LanguageTags::NO => write!(f, "no"),
            LanguageTags::NE => write!(f, "ne"),
            LanguageTags::NL => write!(f, "nl"),
            LanguageTags::OR => write!(f, "or"),
            LanguageTags::PA => write!(f, "pa"),
            LanguageTags::PL => write!(f, "pl"),
            LanguageTags::PT => write!(f, "pt"),
            LanguageTags::PtPT => write!(f, "pt-PT"),
            LanguageTags::RO => write!(f, "ro"),
            LanguageTags::RU => write!(f, "ru"),
            LanguageTags::SI => write!(f, "si"),
            LanguageTags::SK => write!(f, "sk"),
            LanguageTags::SL => write!(f, "sl"),
            LanguageTags::SQ => write!(f, "sq"),
            LanguageTags::SrLATN => write!(f, "sr-Latn"),
            LanguageTags::SR => write!(f, "sr"),
            LanguageTags::SV => write!(f, "sv"),
            LanguageTags::SW => write!(f, "sw"),
            LanguageTags::TA => write!(f, "ta"),
            LanguageTags::TE => write!(f, "te"),
            LanguageTags::TH => write!(f, "th"),
            LanguageTags::TR => write!(f, "tr"),
            LanguageTags::UK => write!(f, "uk"),
            LanguageTags::UR => write!(f, "ur"),
            LanguageTags::UZ => write!(f, "uz"),
            LanguageTags::VI => write!(f, "vi"),
            LanguageTags::ZhCN => write!(f, "zh-CN"),
            LanguageTags::ZhHK => write!(f, "zh-HK"),
            LanguageTags::ZhTW => write!(f, "zh-TW"),
            LanguageTags::ZU => write!(f, "zu"),
        }
    }
}
