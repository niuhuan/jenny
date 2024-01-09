use jmcomic::{FavoritesOrder, SelfInfo, SortBy};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct BackendContext {
    pub login: bool,
    pub last_login: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DartQuery {
    pub method: String,
    pub params: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ResponseToDart {
    pub error_message: String,
    pub response_data: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PreLoginResponse {
    pub pre_set: bool,
    pub pre_login: bool,
    pub self_info: Option<SelfInfo>,
    pub message: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct LoginQuery {
    pub username: String,
    pub password: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComicsQuery {
    pub categories_slug: String,
    pub sort_by: SortBy,
    pub page: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ComicSearchQuery {
    pub search_query: String,
    pub sort_by: SortBy,
    pub page: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ForumQuery {
    pub mode: Option<String>,
    pub aid: Option<i64>,
    pub page: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct CommentQuery {
    pub aid: i64,
    pub comment: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ChildCommentQuery {
    pub aid: i64,
    pub comment: String,
    pub comment_id: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaveProperty {
    pub k: String,
    pub v: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PageImageQuery {
    pub id: i64,
    pub image_name: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ImageSize {
    pub w: u32,
    pub h: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpdateViewLogQuery {
    pub id: i64,
    pub last_view_chapter_id: i64,
    pub last_view_page: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadCreate {
    pub album: DownloadCreateAlbum,
    pub chapters: Vec<DownloadCreateChapter>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadCreateAlbum {
    pub id: i64,
    pub name: String,
    pub author: Vec<String>,
    pub tags: Vec<String>,
    pub works: Vec<String>,
    pub description: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DownloadCreateChapter {
    pub id: i64,
    pub name: String,
    pub sort: String,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FavoursQuery {
    pub folder_id: i64,
    pub page: i64,
    pub o: FavoritesOrder,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PerUserFavourFolder {
    pub username: String,
    pub ff_json: String,
}

////////////////////////////

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportQuery {
    pub dir: String,
    pub comic_id: Vec<i64>,
    pub delete_exported: bool,
}

macro_rules! enum_str {
    ($name:ident { $($variant:ident($str:expr), )* }) => {
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        pub enum $name {
            $($variant,)*
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

enum_str!(SyncDirection {
    Merge("Merge"),
});

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SyncWebdav {
    pub url: String,
    pub username: String,
    pub password: String,
    pub direction: SyncDirection,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ExportSingleQuery {
    pub id: i64,
    pub folder: String,
    pub rename: Option<String>,
    pub delete_exported: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SaveImage {
    pub folder: String,
    pub path: String,
}
