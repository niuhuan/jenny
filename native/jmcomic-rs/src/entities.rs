use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategoryData {
    pub categories: Vec<Category>,
    pub blocks: Vec<Block>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Category {
    #[serde(deserialize_with = "fuzzy_i64")]
    pub id: i64,
    pub name: String,
    pub slug: String,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub total_albums: i64,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Block {
    pub title: String,
    pub content: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SearchPage<T> {
    pub search_query: String,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub total: i64,
    pub content: Vec<T>,
    #[serde(deserialize_with = "fuzzy_option_i64", default = "default_option_i64")]
    pub redirect_aid: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicSimple {
    #[serde(deserialize_with = "fuzzy_i64")]
    pub id: i64,
    pub author: String,
    #[serde(deserialize_with = "null_string")]
    pub description: String,
    pub name: String,
    pub image: String,
    pub category: CategorySimple,
    pub category_sub: CategorySimple,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CategorySimple {
    pub id: Option<String>,
    pub title: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicAlbumResponse {
    pub id: i64,
    pub name: String,
    pub author: Vec<String>,
    pub images: Vec<String>,
    #[serde(deserialize_with = "null_string")]
    pub description: String,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub total_views: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub likes: i64,
    pub series: Vec<Series>,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub series_id: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub comment_total: i64,
    pub tags: Vec<String>,
    pub works: Vec<String>,
    // pub actors: Vec<Value>,
    pub related_list: Vec<RelatedList>,
    pub liked: bool,
    pub is_favorite: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ComicChapterResponse {
    #[serde(deserialize_with = "fuzzy_i64")]
    pub id: i64,
    pub series: Vec<Series>,
    pub tags: String,
    pub name: String,
    pub images: Vec<String>,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub series_id: i64,
    pub is_favorite: bool,
    pub liked: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Series {
    #[serde(deserialize_with = "fuzzy_i64")]
    pub id: i64,
    pub name: String,
    pub sort: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RelatedList {
    #[serde(deserialize_with = "fuzzy_i64")]
    pub id: i64,
    pub author: String,
    #[serde(deserialize_with = "null_string")]
    pub description: String,
    pub name: String,
    pub image: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Page<T> {
    pub list: Vec<T>,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub total: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CountPage<T> {
    pub list: Vec<T>,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub total: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub count: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct VideoSimple {
    pub id: String,
    pub photo: String,
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Comment {
    // 仅主要评论
    #[serde(
        deserialize_with = "fuzzy_i64",
        default = "default_i64",
        rename = "AID"
    )]
    pub aid: i64,
    // 仅主要评论
    // #[serde(rename = "BID")]
    // pub bid: Value,
    #[serde(deserialize_with = "fuzzy_i64", rename = "CID")]
    pub cid: i64,
    #[serde(deserialize_with = "fuzzy_i64", rename = "UID")]
    pub uid: i64,
    pub username: String,
    pub nickname: String,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub likes: i64,
    pub gender: String,
    pub update_at: String,
    pub addtime: String,
    #[serde(deserialize_with = "fuzzy_i64", rename = "parent_CID")]
    pub parent_cid: i64,
    pub expinfo: Expinfo,
    // 仅主要评论, 文章名字
    #[serde(default = "default_string")]
    pub name: String,
    pub content: String,
    pub photo: String,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub spoiler: i64,
    // 仅主要评论, 还是错别字
    #[serde(default = "default_vec")]
    pub replys: Vec<Comment>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Expinfo {
    pub level_name: String,
    pub level: i64,
    #[serde(rename = "nextLevelExp")]
    pub next_level_exp: i64,
    pub exp: String,
    #[serde(rename = "expPercent")]
    pub exp_percent: f64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub uid: i64,
    pub badges: Vec<Badge>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Badge {
    pub content: String,
    pub name: String,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SelfInfo {
    #[serde(deserialize_with = "fuzzy_i64")]
    pub uid: i64,
    pub username: String,
    pub email: String,
    pub emailverified: String,
    pub photo: String,
    pub fname: String,
    pub gender: String,
    #[serde(deserialize_with = "null_string")]
    pub message: String,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub coin: i64,
    pub album_favorites: i64,
    pub s: String,
    #[serde(default = "default_vec")]
    pub favorite_list: Vec<FavoriteFolder>,
    pub level_name: String,
    pub level: i64,
    #[serde(rename = "nextLevelExp")]
    pub next_level_exp: i64,
    pub exp: String,
    #[serde(rename = "expPercent")]
    pub exp_percent: f64,
    pub badges: Vec<Badge>,
    pub album_favorites_max: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FavoriteFolder {
    #[serde(rename = "0")]
    pub n0: String,
    #[serde(rename = "FID")]
    pub fid: String,
    #[serde(rename = "1")]
    pub n1: String,
    #[serde(rename = "UID")]
    pub uid: String,
    #[serde(rename = "2")]
    pub n2: String,
    pub name: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ActionResponse {
    pub status: String,
    pub msg: String,
    #[serde(rename = "type")]
    pub action_type: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommentResponse {
    #[serde(deserialize_with = "null_string")]
    pub msg: String,
    pub status: String,
    pub aid: i64,
    pub cid: i64,
    pub spoiler: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GamePage {
    pub games: Vec<Game>,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub games_total: i64,
    pub categories: Vec<GameCategory>,
    pub hot_games: Vec<Game>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Game {
    #[serde(deserialize_with = "fuzzy_i64")]
    pub gid: i64,
    pub title: String,
    pub description: String,
    pub tags: String,
    pub link: String,
    pub link_title: String,
    pub photo: String,
    #[serde(rename = "type")]
    pub game_type: Vec<String>,
    pub categories: GameCategory,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub update_at: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub total_clicks: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub order_rank: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub status: i64,
    pub show_lang: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameCategory {
    #[serde(deserialize_with = "null_string")]
    pub name: String,
    #[serde(deserialize_with = "null_string")]
    pub slug: String,
}

/////////////////////

macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
        }

        impl $name {
            pub fn as_str(&self) -> &'static str {
                match self {
                    $( $name::$variant => $str, )*
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
                match self {
                    $( $name::$variant => write!(f,"{}",$str), )*
                }
            }
        }

        impl ::serde::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::Serializer,
            {
                // 将枚举序列化为字符串。
                serializer.serialize_str(match *self {
                    $( $name::$variant => $str, )*
                })
            }
        }

        impl<'de> ::serde::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where D: ::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> ::serde::de::Visitor<'de> for Visitor {
                    type Value = $name;

                    fn expecting(&self, formatter: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                        write!(formatter, "a string for {}", stringify!($name))
                    }

                    fn visit_str<E>(self, value: &str) -> Result<$name, E>
                        where E: ::serde::de::Error,
                    {
                        match value {
                            $( $str => Ok($name::$variant), )*
                            _ => Err(E::invalid_value(::serde::de::Unexpected::Other(
                                &format!("unknown {} variant: {}", stringify!($name), value)
                            ), &self)),
                        }
                    }
                }

                // 从字符串反序列化枚举。
                deserializer.deserialize_str(Visitor)
            }
        }
    }
}

enum_str!(SortBy {
    Default(""),
    New("mr"),
    Favourite("tf"),
    View("mv"),
    ViewDay("mv_t"),
    ViewWeek("mv_w"),
    ViewMonth("mv_m"),
});

// enum_str!(ApiHost {
//     Default("www.jmapinode.biz"),
//     Branch1("www.jmapinode.top"),
//     Branch2("www.jmapinode2.top"),
//     Branch3("www.jmapinode3.top"),
// });

enum_str!(ApiHost {
    Default("www.asjmapihost.cc"),
    Branch1("www.jmapinode1.top"),
    Branch2("www.jmapinode2.top"),
    Branch3("www.jmapinode3.top"),
});

enum_str!(CdnHost {
    Proxy1("cdn-msp.jmapiproxy1.monster"),
    Proxy2("cdn-msp2.jmapiproxy1.monster"),
});

enum_str!(FavoritesOrder {
    Mr("mr"),
    Mp("mp"),
});

enum_str!(ActionStatus {
    Ok("ok"),
    Fail("fail"),
});

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct JmActionResponse {
    pub status: ActionStatus,
    pub msg: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FFPage {
    pub list: Vec<ComicSimple>,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub total: i64,
    #[serde(deserialize_with = "fuzzy_i64")]
    pub count: i64,
    pub folder_list: Vec<FavoriteFolder>,
}

////////////////

fn null_string<'de, D>(d: D) -> std::result::Result<String, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(d)?;
    if value.is_null() {
        Ok(String::default())
    } else if value.is_string() {
        Ok(value.as_str().unwrap().to_string())
    } else {
        Err(serde::de::Error::custom("type error"))
    }
}

fn fuzzy_i64<'de, D>(d: D) -> std::result::Result<i64, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(d)?;
    if value.is_i64() {
        Ok(value.as_i64().unwrap())
    } else if value.is_string() {
        let str = value.as_str().unwrap();
        let from: std::result::Result<i64, ParseIntError> = std::str::FromStr::from_str(str);
        match from {
            Ok(from) => Ok(from),
            Err(_) => Err(serde::de::Error::custom("parse error")),
        }
    } else {
        Err(serde::de::Error::custom("type error"))
    }
}

fn fuzzy_option_i64<'de, D>(d: D) -> std::result::Result<Option<i64>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value: serde_json::Value = serde::Deserialize::deserialize(d)?;
    if value.is_null() {
        Ok(None)
    } else if value.is_i64() {
        Ok(Some(value.as_i64().unwrap()))
    } else if value.is_string() {
        let str = value.as_str().unwrap();
        let from: std::result::Result<i64, ParseIntError> = std::str::FromStr::from_str(str);
        match from {
            Ok(from) => Ok(Some(from)),
            Err(_) => Err(serde::de::Error::custom("parse error")),
        }
    } else {
        Err(serde::de::Error::custom("type error"))
    }
}

fn default_string() -> String {
    String::default()
}

fn default_i64() -> i64 {
    0
}

fn default_option_i64() -> Option<i64> {
    None
}

fn default_vec<T>() -> Vec<T> {
    vec![]
}
