use std::collections::HashMap;
use std::ops::Deref;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub use anyhow::Result;
pub use entities::*;
use rand::prelude::SliceRandom;
use reqwest::Method;
use serde_json::{json, to_string, Value};
use tokio::sync::Mutex;
use tools::*;
use rand::Rng;

mod entities;
#[cfg(test)]
mod tests;
mod tools;

const APP_VERSION: &'static str = "1.6.1";
const USER_AGENT: &'static str = "Mozilla/5.0 (Linux; Android 13; 8d41w854d Build/TQ1A.230205.002; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/114.0.5735.196 Safari/537.36";
const APP_KEY: &'static str = "0b931a6f4b5ccc3f8d870839d07ae7b2";
// FINAL_KEY 由JS函数 encodeKey(magic+salt) 计算而来
const APP_SALT: &'static str = "18comicAPP";
const APP_CONTENT_SALT: &'static str = "18comicAPPContent";

pub struct Client {
    api_host: Mutex<Option<ApiHost>>,
    cdn_host: Mutex<Option<CdnHost>>,
    pub cookie: Mutex<HashMap<String, String>>,
    pub agent: Mutex<reqwest::Client>,
    pub user_agent: Mutex<String>,
}

impl Client {

    pub fn new() -> Self {
        Self {
            api_host: Mutex::new(None),
            cdn_host: Mutex::new(None),
            cookie: Mutex::new(HashMap::<String, String>::new()),
            agent: Mutex::new(
                reqwest::ClientBuilder::new()
                    .timeout(Duration::new(30, 0))
                    .build()
                    .unwrap(),
            ),
            user_agent: Mutex::new(USER_AGENT.to_string()),
        }
    }

    pub async fn set_agent(&self, agent: reqwest::Client) {
        let mut lock = self.agent.lock().await;
        *lock = agent;
    }

    pub async fn set_user_agent(&self, user_agent: String) {
        let mut lock = self.user_agent.lock().await;
        *lock = user_agent;
    }

    pub async fn init_cookie(&self) -> anyhow::Result<String> {
        self.request_data(Method::GET, "setting", json!({})).await?;
        // self.request_data(Method::GET, "browser_setting", json!({}))
        //     .await?;
        Ok(self.cookie_str().await)
    }

    pub async fn set_cookie(&self, cookie: &str) -> Result<()> {
        self.http_cookie(&cookie.replace(";", "\n").trim()).await?;
        Ok(())
    }

    pub async fn cookie_str(&self) -> String {
        let lock = self.cookie.lock().await;
        let mut cookies = Vec::<String>::new();
        for (k, v) in lock.deref() {
            cookies.push(format!("{}={};", k, v));
        }
        let cstr = cookies.join("");
        return cstr;
    }

    pub async fn set_api_host<T>(&self, api_host: T)
    where
        T: Into<Option<ApiHost>>,
    {
        *(self.api_host.lock().await) = api_host.into();
    }

    pub async fn get_api_host(&self) -> Option<ApiHost> {
        self.api_host.lock().await.clone()
    }

    pub async fn set_cdn_host<T>(&self, cdn_host: T)
    where
        T: Into<Option<CdnHost>>,
    {
        *(self.cdn_host.lock().await) = cdn_host.into();
    }

    pub async fn get_cdn_host(&self) -> Option<CdnHost> {
        self.cdn_host.lock().await.clone()
    }

    fn random_api_host(&self) -> ApiHost {
        let vec = vec![
            ApiHost::Default,
            ApiHost::Branch1,
            ApiHost::Branch2,
            ApiHost::Branch3,
        ];
        vec.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    fn random_cdn_host(&self) -> CdnHost {
        let vec = vec![CdnHost::Proxy1, CdnHost::Proxy2];
        vec.choose(&mut rand::thread_rng()).unwrap().clone()
    }

    async fn api_host_string(&self) -> String {
        format!(
            "{}",
            if let Some(api) = self.api_host.lock().await.clone() {
                api
            } else {
                self.random_api_host()
            }
        )
    }

    async fn cdn_host_string(&self) -> String {
        format!(
            "{}",
            if let Some(api) = self.cdn_host.lock().await.clone() {
                api
            } else {
                self.random_cdn_host()
            }
        )
    }

    async fn http_cookie(&self, set_cookie_str: &str) -> Result<()> {
        let set_cookie_str: Vec<&str> = set_cookie_str
            .split("\n")
            .map(|str| str.trim())
            .filter(|str| !str.is_empty())
            .collect();
        for set_cookie in set_cookie_str {
            let set_cookie = set_cookie.split(";").nth(0).unwrap_or("");
            let set_cookie: Vec<&str> = set_cookie.split("=").collect();
            if set_cookie.len() == 2 {
                let mut lock = self.cookie.lock().await;
                lock.insert(
                    set_cookie.get(0).unwrap().to_string(),
                    set_cookie.get(1).unwrap().to_string(),
                );
                drop(lock);
            }
        }
        Ok(())
    }

    pub async fn request_data(&self, method: Method, path: &str, query: Value) -> Result<String> {
        let mut obj = query.as_object().unwrap().clone();
        obj.insert("key".to_string(), Value::from(APP_KEY));
        obj.insert("view_mode_debug".to_string(), Value::from("1"));
        obj.insert("view_mode".to_string(), Value::from("null"));
        let agent = self.agent.lock().await;
        let request = agent.request(
            method.clone(),
            format!("https://{}/{}", &self.api_host_string().await, path),
        );
        drop(agent);
        let time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let token_param = format!("{},{}", &time, &APP_VERSION);
        let token = hex::encode(md5::compute(format!("{}{}", time, APP_SALT)).0);
        let decode_key = hex::encode(md5::compute(format!("{}{}", time, APP_CONTENT_SALT)).0);
        // 18comicAPPContent
        let request = request.header("Tokenparam", token_param);
        let request = request.header("token", token);
        let request = request.header("cookie", &self.cookie_str().await);
        let request = request.header("User-Agent", self.current_user_agent().await);
        let request = request.header("Sec-Fetch-Site", "same-origin");
        let request = request.header("Accept-Language", "zh-CN,zh-Hans;q=0.9");
        let request = request.header("Sec-Fetch-Mode", "cors");
        let request = request.header("Content-Type", "application/json");
        let request = request.header("Origin", "null");
        let request = request.header("Sec-Fetch-Dest", "empty");
        let request = match method {
            Method::GET => request.query(&obj),
            _ => request.form(&obj),
        };
        let response = request.send().await?;
        if path == "setting" || path == "login" {
            for x in response.headers() {
                if x.0 == "set-cookie" {
                    self.http_cookie(std::str::from_utf8(x.1.as_bytes())?)
                        .await?;
                }
            }
        }
        let text = response.text().await?;
        let json: Value = from_str(&text)?;
        let code = json.get("code");
        match code {
            None => return Err(anyhow::anyhow!("error response")),
            Some(code) => {
                if code.is_i64() {
                    match code.as_i64().unwrap() {
                        200 => {}
                        _ => {
                            return match json.get("errorMsg") {
                                None => Err(anyhow::anyhow!("unknown error")),
                                Some(msg) => {
                                    if msg.is_string() {
                                        Err(anyhow::anyhow!(msg.as_str().unwrap().to_string()))
                                    } else {
                                        Err(anyhow::anyhow!("unknown error"))
                                    }
                                }
                            };
                        }
                    }
                } else {
                    return Err(anyhow::anyhow!("code not is number"));
                }
            }
        }
        if path == "browser_setting" {
            return Ok(to_string(&json.get("data"))?);
        }
        match json.get("data") {
            None => Err(anyhow::anyhow!("data error 2")),
            Some(data) => {
                if data.is_string() {
                    let data = data.as_str().unwrap();
                    let data = tools::decrypt_jm(data, decode_key.as_bytes())?;
                    Ok(data)
                } else {
                    Err(anyhow::anyhow!("data error 3"))
                }
            }
        }
    }

    async fn request<T: for<'de> serde::Deserialize<'de>>(
        &self,
        method: Method,
        path: &str,
        query: Value,
    ) -> Result<T> {
        let response = self.request_data(method, path, query).await?;
        Ok(from_str(&response)?)
    }

    pub async fn login(&self, username: String, password: String) -> Result<SelfInfo> {
        Ok(self
            .request(
                Method::POST,
                "login",
                json!({
                    "username": username,
                    "password": password,
                }),
            )
            .await?)
    }

    pub async fn categories(&self) -> Result<CategoryData> {
        Ok(self.request(Method::GET, "categories", json!({})).await?)
    }

    pub async fn latest(&self) -> Result<Vec<ComicSimple>> {
        Ok(self.request(Method::GET, "latest", json!({})).await?)
    }

    pub async fn comics(
        &self,
        categories_slug: String,
        sort_by: SortBy,
        page: i64,
    ) -> Result<SearchPage<ComicSimple>> {
        Ok(self
            .request(
                Method::GET,
                "categories/filter",
                json!({
                    "page": page,
                    "order": "",
                    "c": categories_slug,
                    "o": sort_by,
                }),
            )
            .await?)
    }

    pub async fn comics_search(
        &self,
        search_query: String,
        sort_by: SortBy,
        page: i64,
    ) -> Result<SearchPage<ComicSimple>> {
        Ok(self
            .request(
                Method::GET,
                "search",
                json!({
                    "page":page,
                    "search_query": search_query,
                    "o":sort_by,
                }),
            )
            .await?)
    }

    pub async fn album(&self, id: i64) -> Result<ComicAlbumResponse> {
        Ok(self
            .request(
                Method::GET,
                "album",
                json!({
                    "comicName":"",
                    "id":id,
                }),
            )
            .await?)
    }

    pub async fn chapter(&self, id: i64) -> Result<ComicChapterResponse> {
        Ok(self
            .request(
                Method::GET,
                "chapter",
                json!({
                    "comicName":"",
                    "id":id,
                }),
            )
            .await?)
    }

    pub async fn comment(&self, aid: i64, comment: String) -> Result<CommentResponse> {
        Ok(self
            .request(
                Method::POST,
                "comment",
                json!({
                    "comment":comment,
                    "aid":aid,
                }),
            )
            .await?)
    }

    pub async fn child_comment(
        &self,
        aid: i64,
        comment: String,
        comment_id: i64,
    ) -> Result<CommentResponse> {
        Ok(self
            .request(
                Method::POST,
                "comment",
                json!({
                    "comment":comment,
                    "comment_id":comment_id,
                    "aid":aid,
                }),
            )
            .await?)
    }

    pub async fn comic_cover_url_3x4(&self, comic_id: i64) -> String {
        format!(
            "https://{}/media/albums/{}_3x4.jpg",
            self.cdn_host_string().await,
            comic_id
        )
    }

    pub async fn comic_cover_url_square(&self, comic_id: i64) -> String {
        format!(
            "https://{}/media/albums/{}.jpg",
            self.cdn_host_string().await,
            comic_id
        )
    }

    pub async fn comic_page_url(&self, id: i64, name: String) -> String {
        format!(
            "https://{}/media/photos/{}/{}?v=",
            self.cdn_host_string().await,
            id,
            name,
        )
    }

    // cnd host or api host all allow
    pub async fn photo_url(&self, photo_name: String) -> String {
        format!(
            "https://{}/media/users/{}",
            self.cdn_host_string().await,
            photo_name
        )
    }

    pub async fn videos(&self, sort_by: SortBy, page: i64) -> Result<Page<VideoSimple>> {
        Ok(self
            .request(
                Method::GET,
                "videos",
                json!({
                    "o": sort_by,
                    "page": page,
                }),
            )
            .await?)
    }

    // 评论
    pub async fn forum(
        &self,
        mode: Option<String>,
        aid: Option<i64>,
        page: i64,
    ) -> Result<Page<Comment>> {
        if let Some(mode) = mode {
            if let Some(aid) = aid {
                return self
                    .request(
                        Method::GET,
                        "forum",
                        json!({
                            "mode": mode,
                            "aid": aid,
                            "page": page,
                        }),
                    )
                    .await;
            }
            return self
                .request(
                    Method::GET,
                    "forum",
                    json!({
                        "mode": mode,
                        "page": page,
                    }),
                )
                .await;
        }
        self.request(
            Method::GET,
            "forum",
            json!({
                "page": page,
            }),
        )
        .await
    }

    pub async fn set_favorite(&self, aid: i64) -> Result<ActionResponse> {
        Ok(self
            .request(
                Method::POST,
                "favorite",
                json!({
                    "aid": aid,
                }),
            )
            .await?)
    }

    pub async fn favorites(
        &self,
        folder_id: i64,
        page: i64,
        o: FavoritesOrder,
    ) -> Result<CountPage<ComicSimple>> {
        Ok(self
            .request(
                Method::GET,
                "favorite",
                json!({
                    "folder_id": folder_id,
                    "page": page,
                    "o": o,
                }),
            )
            .await?)
    }

    pub async fn favorites_intro(&self) -> Result<FFPage> {
        Ok(self
            .request(
                Method::GET,
                "favorite",
                json!({
                    "folder_id": 0,
                    "o": FavoritesOrder::Mr,
                }),
            )
            .await?)
    }

    pub async fn create_favorite_folder(&self, name: String) -> Result<()> {
        let rsp: JmActionResponse = self
            .request(
                Method::POST,
                "favorite_folder",
                json!({
                    "type":"add",
                    "folder_name":name,
                }),
            )
            .await?;
        match rsp.status {
            ActionStatus::Ok => Ok(()),
            ActionStatus::Fail => Err(anyhow::Error::msg(rsp.msg)),
        }
    }

    pub async fn delete_favorite_folder(&self, folder_id: i64) -> Result<()> {
        let rsp: JmActionResponse = self
            .request(
                Method::POST,
                "favorite_folder",
                json!({
                    "type":"delete",
                    "folder_id":folder_id,
                }),
            )
            .await?;
        match rsp.status {
            ActionStatus::Ok => Ok(()),
            ActionStatus::Fail => Err(anyhow::Error::msg(rsp.msg)),
        }
    }

    pub async fn comic_favorite_folder_move(&self, comic_id: i64, folder_id: i64) -> Result<()> {
        let rsp: JmActionResponse = self
            .request(
                Method::POST,
                "favorite_folder",
                json!({
                    "type":"move",
                    "folder_id":folder_id,
                    "aid":comic_id,
                }),
            )
            .await?;
        match rsp.status {
            ActionStatus::Ok => Ok(()),
            ActionStatus::Fail => Err(anyhow::Error::msg(rsp.msg)),
        }
    }

    pub async fn games(&self, page: i64) -> Result<GamePage> {
        Ok(self
            .request(
                Method::GET,
                "games",
                json!({
                    "page": page,
                }),
            )
            .await?)
    }

    pub async fn watch_list(&self, page: i64) -> Result<Page<ComicSimple>> {
        Ok(self
            .request(
                Method::GET,
                "watch_list",
                json!({
                    "page": page,
                }),
            )
            .await?)
    }

    pub async fn current_user_agent(&self) -> String {
        let lock = self.user_agent.lock().await;
        lock.clone()
    }

    pub fn rand_user_agent() -> String {
        // mobile version
        let adnroid_version = rand::thread_rng().gen_range(11..=15);
        // webkit version
        // rand a str [0-9a0z]{9} like 8d41w854d
        let boot_id = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(9)
            .map(char::from)
            .collect::<String>();
        // random from TQ1A / TQ2A ... TQ9A ... TQ1Z 
        let mobile_model = format!(
            "TQ{}{}",
             rand::thread_rng().gen_range(0..=9),
             rand::thread_rng().gen_range('A'..='Z'),
        );
        // random 230205 - 330205
        let build_version = rand::thread_rng().gen_range(230205..=330205);
        let build_version2= rand::thread_rng().gen_range(1..=9);
        // 
        format!(
            "Mozilla/5.0 (Linux; Android {adnroid_version}; {boot_id} Build/{mobile_model}.{build_version}.00{build_version2}; wv) AppleWebKit/537.36 (KHTML, like Gecko) Version/4.0 Chrome/114.0.5735.196 Safari/537.36",
        )
    }
}
