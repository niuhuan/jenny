use std::ffi::{CStr, CString};
use std::ops::Deref;
use std::path::Path;
use std::time::Duration;

use anyhow::{anyhow, Result};
use image::codecs::png::PngEncoder;
use image::EncodableLayout;
use image::ImageEncoder;
use image::{ColorType, GenericImageView};
use jmcomic::*;
use libc::c_char;
use rand::random;
use serde_derive::{Deserialize, Serialize};
use serde_json::{from_str, to_string};
use tokio::spawn;

use database::image_cache_db::{clean_all_image_cache, use_image_cache};
pub use define::RUNTIME;
use define::*;
use types::*;

use crate::active_db::{db_clear_a_search_log, db_clear_all_search_log};
use crate::database::active_db::{
    clear_view_log, find_view_log, last_view_album, load_last_search_histories, page_view_log,
    save_search_history, update_view_log,
};
use crate::database::property_db::{load_property, save_property};
use crate::database::web_cache_db::{
    clean_all_web_cache, clean_web_cache_by_patten, use_web_cache,
};
use crate::database::{active_db, image_cache_db, property_db, web_cache_db};
use crate::download::{
    all_downloads, create_download, delete_download, dl_image_by_chapter_id, download_by_id,
    jm_3x4_cover_by_id, jm_square_cover_by_id, page_image_by_key, renew_all_downloads,
    DOWNLOAD_AND_EXPORT_TO, RESTART_FLAG,
};
use crate::export::{
    export_cbzs_zip_single, export_jm_jmi, export_jm_jmi_single, export_jm_jpegs,
    export_jm_jpegs_zip_single, export_jm_zip, export_jm_zip_single, import_jm_dir, import_jm_jmi,
    import_jm_zip,
};
use crate::sync::sync_webdav;

mod database;
mod define;
mod download;
mod export;
mod sync;
mod tools;
mod types;

#[no_mangle]
pub unsafe extern "C" fn init_ffi(c: *const c_char) {
    init_sync(CStr::from_ptr(c).to_str().unwrap());
}

#[no_mangle]
pub unsafe extern "C" fn migration_ffi(from: *const c_char, to: *const c_char) {
    let from_string = CStr::from_ptr(from).to_str().unwrap();
    let to_string = CStr::from_ptr(to).to_str().unwrap();
    RUNTIME.block_on(migration(from_string, to_string));
}

async fn migration(from: &str, _: &str) {
    let source = Path::new(from);
    if source.exists() {
        let mut rd = tokio::fs::read_dir(source).await.unwrap();
        while let Some(item) = rd.next_entry().await.unwrap() {
            if item
                .file_name()
                .to_str()
                .unwrap()
                .starts_with("property.db")
            {
                tokio::fs::remove_file(item.path()).await.unwrap();
            }
        }
    } else {
        tokio::fs::create_dir_all(source).await.unwrap();
    }
}

pub fn init_sync(params: &str) {
    RUNTIME.block_on(init(params));
}

async fn init(params: &str) {
    let mut init_lock = INITED.lock().await;
    if *init_lock {
        drop(init_lock);
        return;
    }
    *init_lock = true;
    drop(init_lock);

    tokio::fs::create_dir_all(params.to_string()).await.unwrap();
    let mut lock = FOLDER.lock().await;
    *lock = params.to_string();

    println!("RUST INIT : {}", lock.clone());

    drop(lock);

    image_cache_db::init_dir().await;
    download::init_dir().await;

    // init_database
    active_db::init_db().await;
    image_cache_db::init_db().await;
    property_db::init_db().await;
    web_cache_db::init_db().await;

    //

    // auto clean
    let mut auto_clean_time_str = load_property("auto_clean".to_string()).await.unwrap();
    if auto_clean_time_str == "" {
        auto_clean_time_str = format!("{}", 3600 * 24 * 30);
        save_property("auto_clean".to_string(), auto_clean_time_str.clone())
            .await
            .unwrap();
    }
    let timestamp: i64 = auto_clean_time_str.parse::<i64>().unwrap();
    let timestamp = chrono::Local::now().timestamp() - timestamp;
    image_cache_db::clean_image_cache_by_time(timestamp)
        .await
        .unwrap();
    web_cache_db::clean_web_cache_by_time(timestamp)
        .await
        .unwrap();

    let proxy_url = load_property("proxy_url".to_owned()).await.unwrap();
    if proxy_url.len() > 0 {
        let _ = set_proxy(&proxy_url).await;
    }

    //

    let ua = load_property("ua1".to_owned()).await.unwrap();
    if ua.len() > 0 {
        CLIENT.set_user_agent(ua).await;
    } else {
        let ua = Client::rand_user_agent();
        save_property("ua1".to_owned(), ua.clone()).await.unwrap();
        CLIENT.set_user_agent(ua).await;
    }
    save_property("ua".to_owned(), "".to_owned()).await.unwrap();

    if let Ok(is_pro) = is_pro().await {
        if is_pro.is_pro {
            *DOWNLOAD_AND_EXPORT_TO.lock().await =
                load_property("download_and_export_to".to_owned())
                    .await
                    .unwrap();
        }
    }

    let _ = spawn(download::start_download());
}

async fn get_download_and_export_to() -> Result<String> {
    Ok((*DOWNLOAD_AND_EXPORT_TO.lock().await).clone())
}

async fn set_download_and_export_to(params: &str) -> Result<String> {
    save_property("download_and_export_to".to_owned(), params.to_string()).await?;
    *DOWNLOAD_AND_EXPORT_TO.lock().await = params.to_string();
    Ok("".to_string())
}

#[no_mangle]
pub unsafe extern "C" fn invoke_ffi(params: *const c_char) -> *mut c_char {
    let params = CStr::from_ptr(params).to_str().unwrap();
    let response = invoke(params);
    CString::new(response).unwrap().into_raw()
}

#[no_mangle]
pub unsafe extern "C" fn free_str_ffi(c: *mut c_char) {
    drop(CString::from_raw(c));
}

pub fn invoke(params: &str) -> String {
    RUNTIME.block_on(invoke_async(params))
}

pub async fn invoke_async(params: &str) -> String {
    let query: DartQuery = serde_json::from_str(params).unwrap();
    let result: Result<String> = match_method(query.method.as_str(), query.params.as_str()).await;
    let result: ResponseToDart = match result {
        Ok(str) => ResponseToDart {
            response_data: str,
            error_message: "".to_string(),
        },
        Err(err) => ResponseToDart {
            response_data: "".to_string(),
            error_message: err.to_string(),
        },
    };
    serde_json::to_string(&result).unwrap()
}

async fn match_method(method: &str, params: &str) -> Result<String> {
    match method {
        "test" => Ok("".to_string()),
        "save_api_host" => save_api_host(params).await,
        "load_api_host" => load_api_host().await,
        "save_cdn_host" => save_cdn_host(params).await,
        "load_cdn_host" => load_cdn_host().await,
        "load_username" => load_username().await,
        "loadLastLoginUsername" => load_last_login_username().await,
        "load_password" => load_password().await,
        "init" => init_dart().await,
        "pre_login" => pre_login().await,
        "login" => login(params).await,
        "logout" => logout(params).await,
        "save_property" => {
            let sp: SaveProperty = serde_json::from_str(params)?;
            save_property(sp.k, sp.v).await
        }
        "load_property" => load_property(params.to_owned()).await,
        "comics" => comics(params).await,
        "comic_search" => comic_search(params).await,
        "categories" => categories().await,
        "album" => album(params).await,
        "chapter" => chapter(params).await,
        "forum" => forum(params).await,
        "comment" => comment(params).await,
        "child_comment" => child_comment(params).await,
        "set_favorite" => set_favorite(params).await,
        "favorites" => favorites(params).await,
        "games" => games(params).await,
        "jm_3x4_cover" => jm_3x4_cover(params).await,
        "jm_square_cover" => jm_square_cover(params).await,
        "jm_page_image" => jm_page_image(params).await,
        "jm_photo_image" => jm_photo_image(params).await,
        "image_size" => image_size(params).await,
        "http_get" => http_get(params).await,
        "clean_all_cache" => clean_all_cache().await,
        "update_view_log" => update_view_log(from_str(params)?).await,
        "find_view_log" => Ok(to_string(&find_view_log(params.parse::<i64>()?).await?)?),
        "page_view_log" => Ok(to_string(&page_view_log(params.parse::<i64>()?).await?)?),
        "clear_view_log" => clear_view_log().await,
        "last_search_histories" => last_search_histories(params).await,
        "create_download" => create_download(params).await,
        "all_downloads" => all_downloads().await,
        "download_by_id" => download_by_id(params.parse::<i64>()?).await,
        "dl_image_by_chapter_id" => dl_image_by_chapter_id(params.parse::<i64>()?).await,
        "delete_download" => delete_download(params.parse::<i64>()?).await,
        "renew_all_downloads" => renew_all_downloads().await,
        "export_jm_jpegs" => export_jm_jpegs(params).await,
        "export_jm_zip" => export_jm_zip(params).await,
        "export_jm_zip_single" => export_jm_zip_single(params).await,
        "export_jm_jpegs_zip_single" => export_jm_jpegs_zip_single(params).await,
        "export_jm_jmi" => export_jm_jmi(params).await,
        "export_jm_jmi_single" => export_jm_jmi_single(params).await,
        "export_cbzs_zip_single" => export_cbzs_zip_single(params).await,
        "import_jm_zip" => import_jm_zip(params).await,
        "import_jm_jmi" => import_jm_jmi(params).await,
        "import_jm_dir" => import_jm_dir(params).await,
        "reload_pro" => reload_pro().await,
        "is_pro" => async { Ok(to_string(&is_pro().await?)?) }.await,
        "input_cd_key" => input_cd_key(params).await,
        "set_download_thread" => set_download_thread(params.parse::<i64>()?).await,
        "load_download_thread" => Ok(load_download_thread().await?.to_string()),
        "clear_all_search_log" => clear_all_search_log().await,
        "clear_a_search_log" => clear_a_search_log(params).await,
        "set_proxy" => set_proxy(params).await,
        "get_proxy" => get_proxy().await,
        "sync_webdav" => sync_webdav(params).await,
        "set_download_and_export_to" => set_download_and_export_to(params).await,
        "get_download_and_export_to" => get_download_and_export_to().await,
        "ping_server" => ping_server(params).await,
        "getHomeDir" => get_home_dir(),
        "mkdirs" => mkdirs(params),
        "copyPictureToFolder" => copy_picture_to_folder(params).await,
        "set_pro_server_name" => set_pro_server_name(params).await,
        "get_pro_server_name" => get_pro_server_name().await,
        name => return Err(anyhow!("NO FLAT : {}", name)),
    }
}

async fn init_dart() -> Result<String> {
    let mut api_host_db = load_property("api_host".to_string()).await?;
    if api_host_db.is_empty() {
        api_host_db = "1".to_owned();
        save_property("api_host".to_string(), api_host_db.clone()).await?;
    }
    CLIENT
        .set_api_host(match api_host_db.as_str() {
            "0" => ApiHost::Default,
            "1" => ApiHost::Branch1,
            "2" => ApiHost::Branch2,
            "3" => ApiHost::Branch3,
            _ => ApiHost::Default,
        })
        .await;
    let cdn_host_db = load_property("cdn_host".to_string()).await?;
    if cdn_host_db.is_empty() {
        save_property("cdn_host".to_string(), "1".to_owned()).await?;
    }
    CLIENT
        .set_cdn_host(match cdn_host_db.as_str() {
            "0" => None,
            "1" => Some(CdnHost::Proxy1),
            "2" => Some(CdnHost::Proxy2),
            _ => None,
        })
        .await;
    Ok("".to_string())
}

async fn save_api_host(params: &str) -> Result<String> {
    let api_host = match params.parse::<i64>()? {
        0 => ApiHost::Default,
        1 => ApiHost::Branch1,
        2 => ApiHost::Branch2,
        3 => ApiHost::Branch3,
        _ => return Err(anyhow!("不支持的分流")),
    };
    let _ = save_property("api_host".to_string(), params.to_string()).await?;
    CLIENT.set_api_host(api_host).await;
    CONTEXT.lock().await.last_login = 0;
    Ok("".to_owned())
}

async fn load_api_host() -> Result<String> {
    Ok(match CLIENT.get_api_host().await {
        None => "0".to_owned(),
        Some(ApiHost::Default) => "0".to_owned(),
        Some(ApiHost::Branch1) => "1".to_owned(),
        Some(ApiHost::Branch2) => "2".to_owned(),
        Some(ApiHost::Branch3) => "3".to_owned(),
    })
}

async fn save_cdn_host(params: &str) -> Result<String> {
    let cdn_host = match params.parse::<i64>()? {
        0 => None,
        1 => Some(CdnHost::Proxy1),
        2 => Some(CdnHost::Proxy2),
        _ => return Err(anyhow!("不支持的分流")),
    };
    let _ = save_property("cdn_host".to_string(), params.to_string()).await?;
    CLIENT.set_cdn_host(cdn_host).await;
    Ok("".to_owned())
}

async fn load_cdn_host() -> Result<String> {
    Ok(match CLIENT.get_cdn_host().await {
        None => "0".to_owned(),
        Some(CdnHost::Proxy1) => "1".to_owned(),
        Some(CdnHost::Proxy2) => "2".to_owned(),
    })
}

async fn load_download_thread() -> Result<i64> {
    if is_pro().await?.is_pro {
        Ok(property_db::load_int_property("download_thread_count".to_owned(), 1).await)
    } else {
        Ok(1)
    }
}

async fn set_download_thread(count: i64) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("需要发电鸭"));
    }
    property_db::save_property("download_thread_count".to_owned(), format!("{count}")).await?;
    let mut restart_flag = RESTART_FLAG.lock().await;
    if *restart_flag.deref() {
        *restart_flag = true;
    }
    Ok("".to_string())
}

async fn load_username() -> Result<String> {
    load_property("username".to_owned()).await
}

async fn load_last_login_username() -> Result<String> {
    load_property("last_login_username".to_owned()).await
}

async fn load_password() -> Result<String> {
    load_property("password".to_owned()).await
}

async fn pre_login() -> Result<String> {
    let username = load_property("username".to_owned()).await?;
    let password = load_property("password".to_owned()).await?;
    check_first().await?;
    let data = match username != "" && password != "" {
        true => match CLIENT.login(username.clone(), password).await {
            Ok(mut info) => {
                let mut lock = CONTEXT.lock().await;
                lock.login = true;
                lock.last_login = chrono::Local::now().timestamp();
                save_property("cookie".to_string(), CLIENT.cookie_str().await).await?;
                save_property("last_login_username".to_owned(), username.to_string()).await?;
                drop(lock);
                if let Ok(json) = load_property("fav_list".to_string()).await {
                    if json.len() > 0 {
                        if let Ok(ff) = from_str::<PerUserFavourFolder>(&json) {
                            if username.eq(&ff.username) {
                                if let Ok(ff) = from_str::<Vec<FavoriteFolder>>(&ff.ff_json) {
                                    info.favorite_list = ff;
                                }
                            }
                        }
                    }
                }
                PreLoginResponse {
                    pre_set: true,
                    pre_login: true,
                    self_info: Some(info),
                    message: None,
                }
            }
            Err(err) => PreLoginResponse {
                pre_set: true,
                pre_login: false,
                self_info: None,
                message: Some(format!("{:?}", err)),
            },
        },
        false => PreLoginResponse {
            pre_set: false,
            pre_login: false,
            self_info: None,
            message: None,
        },
    };
    Ok(to_string(&data)?)
}

pub(crate) async fn is_pro() -> Result<IsPro> {
    Ok(IsPro {
        is_pro: true,
        expire: chrono::Local::now().timestamp() + 3600 * 24 * 30,
    })
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct IsPro {
    pub is_pro: bool,
    pub expire: i64,
}

async fn reload_pro() -> Result<String> {
    Ok("".to_string())
}

 
async fn input_cd_key(_params: &str) -> Result<String> {
    Ok("".to_string())
}

pub(crate) async fn check_first() -> Result<()> {
    let mut lock = FIRST_LOGIN.lock().await;
    if !lock.deref() {
        let cookie = load_property("cookie".to_string()).await?;
        if cookie.is_empty() {
            save_property("cookie".to_string(), CLIENT.cookie_str().await).await?;
        } else {
            CLIENT.set_cookie(&cookie).await?;
        }
    }
    *lock = true;
    Ok(())
}

async fn login(params: &str) -> Result<String> {
    check_first().await?;
    let query: LoginQuery = from_str(params)?;
    let mut data = match CLIENT
        .login(query.username.clone(), query.password.clone())
        .await
    {
        Ok(data) => data,
        Err(err) => {
            println!("LOGIN ERROR : {:?}", err);
            return Err(err);
        }
    };
    let mut lock = CONTEXT.lock().await;
    lock.login = true;
    lock.last_login = chrono::Local::now().timestamp();
    save_property("cookie".to_string(), CLIENT.cookie_str().await).await?;
    drop(lock);
    save_property("username".to_owned(), query.username.clone()).await?;
    save_property("password".to_owned(), query.password).await?;
    let lpp = load_property("last_login_username".to_owned()).await?;
    save_property("last_login_username".to_owned(), query.username.to_string()).await?;
    if !lpp.eq(&query.username) {
        let _ = reload_pro().await;
    }
    if data.favorite_list.len() > 0 {
        if let Ok(json) = to_string(&data.favorite_list) {
            if let Ok(json) = to_string(&PerUserFavourFolder {
                username: query.username.clone(),
                ff_json: json,
            }) {
                let _ = save_property("fav_list".to_string(), json).await;
            }
        }
    } else {
        if let Ok(json) = load_property("fav_list".to_string()).await {
            if json.len() > 0 {
                if let Ok(ff) = from_str::<PerUserFavourFolder>(&json) {
                    if query.username.eq(&ff.username) {
                        if let Ok(ff) = from_str::<Vec<FavoriteFolder>>(&ff.ff_json) {
                            data.favorite_list = ff;
                        }
                    }
                }
            }
        }
    }
    Ok(to_string(&data)?)
}

async fn logout(_: &str) -> Result<String> {
    save_property("cookie".to_string(), "".to_owned()).await?;
    save_property("username".to_owned(), "".to_owned()).await?;
    save_property("password".to_owned(), "".to_owned()).await?;
    save_property("last_login_username".to_owned(), "".to_owned()).await?;
    save_property("fav_list".to_string(), "".to_owned()).await?;
    Ok("".to_owned())
}

async fn categories() -> Result<String> {
    use_web_cache("CATEGORIES".to_string(), Duration::new(3600, 0), || async {
        Ok(to_string(&CLIENT.categories().await?)?)
    })
    .await
}

const NO_PRO_MAX: i64 = 10;

async fn comics(params: &str) -> Result<String> {
    let query: ComicsQuery = from_str(params)?;
    let key = format!(
        "COMICS:{}:{}:{}",
        query.categories_slug.clone(),
        query.sort_by.clone(),
        query.page.clone(),
    );
    if !is_pro().await?.is_pro && query.page > NO_PRO_MAX {
        return Err(anyhow!("需要发电鸭"));
    }
    use_web_cache(key, Duration::new(3600, 0), || async {
        Ok(to_string(
            &CLIENT
                .comics(query.categories_slug, query.sort_by, query.page)
                .await?,
        )?)
    })
    .await
}

async fn comic_search(params: &str) -> Result<String> {
    let query: ComicSearchQuery = from_str(params)?;
    if !is_pro().await?.is_pro && query.page > NO_PRO_MAX {
        return Err(anyhow!("需要发电鸭"));
    }
    save_search_history(query.search_query.clone()).await?;
    let key = format!(
        "COMIC_SEARCH:{}:{}:{}",
        query.search_query.clone(),
        query.sort_by.clone(),
        query.page.clone(),
    );
    use_web_cache(key, Duration::new(3600, 0), || async {
        Ok(to_string(
            &CLIENT
                .comics_search(query.search_query, query.sort_by, query.page)
                .await?,
        )?)
    })
    .await
}

async fn clear_all_search_log() -> Result<String> {
    db_clear_all_search_log().await?;
    Ok("".to_string())
}

async fn clear_a_search_log(content: &str) -> Result<String> {
    db_clear_a_search_log(content.to_string()).await?;
    Ok("".to_string())
}

async fn album(params: &str) -> Result<String> {
    let comic_id = params.parse::<i64>()?;
    let key = format!("ALBUM:{}", comic_id);
    let data = use_web_cache(key, Duration::new(3600, 0), || async {
        Ok(to_string(&CLIENT.album(comic_id).await?)?)
    })
    .await?;
    let model: ComicAlbumResponse = from_str(&data)?;
    last_view_album(model).await?;
    Ok(data)
}

async fn chapter(params: &str) -> Result<String> {
    let comic_id = params.parse::<i64>()?;
    let key = format!("CHAPTER:{}", comic_id);
    use_web_cache(key, Duration::new(3600, 0), || async {
        Ok(to_string(&CLIENT.chapter(comic_id).await?)?)
    })
    .await
}

async fn forum(params: &str) -> Result<String> {
    let query: ForumQuery = serde_json::from_str(params)?;
    let key = format!(
        "FORUM:{}:{}:{}",
        if let Some(mode) = query.mode.clone() {
            mode
        } else {
            "null".to_string()
        },
        if let Some(aid) = query.aid.clone() {
            aid.to_string()
        } else {
            "null".to_string()
        },
        query.page
    );
    use_web_cache(key, Duration::new(3600, 0), || async {
        let mut comments = CLIENT.forum(query.mode, query.aid, query.page).await?;
        for i in 0..comments.list.len() {
            let tmp = comments.list[i]
                .content
                .trim_start_matches("<div style='flex-direction:row;flex-wrap:wrap;'>")
                .trim_end_matches("</div>")
                .to_string();
            comments.list[i].content = tmp;
        }
        Ok(to_string(&comments)?)
    })
    .await
}

async fn comment(params: &str) -> Result<String> {
    check_login().await?;
    let query: CommentQuery = serde_json::from_str(params)?;
    clean_web_cache_by_patten(format!("FORUM:%:{}:%", query.aid.clone())).await?;
    clean_web_cache_by_patten(format!("CHAPTER:{}", query.aid.clone())).await?;
    Ok(to_string(&CLIENT.comment(query.aid, query.comment).await?)?)
}

async fn child_comment(params: &str) -> Result<String> {
    check_login().await?;
    let query: ChildCommentQuery = serde_json::from_str(params)?;
    clean_web_cache_by_patten(format!("FORUM:%:{}:%", query.aid.clone())).await?;
    clean_web_cache_by_patten(format!("CHAPTER:{}", query.aid.clone())).await?;
    Ok(to_string(
        &CLIENT
            .child_comment(query.aid, query.comment, query.comment_id)
            .await?,
    )?)
}

async fn check_login() -> Result<String> {
    let time = chrono::Local::now().timestamp();
    let mut lock = CONTEXT.lock().await;
    if !lock.login {
        drop(lock);
        return Err(anyhow!("请登录"));
    }
    if time - 600 > lock.last_login {
        println!("need re login");
        // 调用login会重复加锁死锁
        let username = load_property("username".to_owned()).await?;
        let password = load_property("password".to_owned()).await?;
        let _ = CLIENT.login(username, password).await?;
        println!("re login");
        lock.login = true;
        lock.last_login = chrono::Local::now().timestamp();
        save_property("cookie".to_string(), CLIENT.cookie_str().await).await?;
        drop(lock);
    }
    return Ok("".to_owned());
}

async fn favorites(params: &str) -> Result<String> {
    check_login().await?;
    let query: FavoursQuery = from_str(params)?;
    if !is_pro().await?.is_pro && query.page > NO_PRO_MAX {
        return Err(anyhow!("需要发电鸭"));
    }
    let key = format!("FAVORITES:{}:{}:{}", query.folder_id, query.page, query.o);
    use_web_cache(key, Duration::new(600, 0), || async {
        Ok(to_string(
            &CLIENT
                .favorites(query.folder_id, query.page, query.o)
                .await?,
        )?)
    })
    .await
}

async fn set_favorite(params: &str) -> Result<String> {
    check_login().await?;
    let aid = params.parse::<i64>()?;
    let data = CLIENT.set_favorite(aid).await?;
    clean_web_cache_by_patten("FAVORITES:%".to_owned()).await?;
    clean_web_cache_by_patten(format!("ALBUM:{}", aid)).await?;
    clean_web_cache_by_patten(format!("CHAPTER:{}", aid)).await?;
    Ok(to_string(&data)?)
}

async fn games(params: &str) -> Result<String> {
    check_login().await?;
    let page = params.parse::<i64>()?;
    let key = format!("GAMES:{}", page);
    use_web_cache(key, Duration::new(3600, 0), || async {
        Ok(to_string(&CLIENT.games(page).await?)?)
    })
    .await
}

async fn jm_3x4_cover(params: &str) -> Result<String> {
    let comic_id = params.parse::<i64>()?;
    if let Some(path) = jm_3x4_cover_by_id(comic_id).await {
        return Ok(path);
    }
    let key = format!("JM_3X4_COVER:{}", comic_id.clone());
    let url = CLIENT.comic_cover_url_3x4(comic_id).await;

    use_image_cache(
        key,
        Box::pin(download_image_from_url(&url, 0, String::default())),
    )
    .await
}

async fn jm_square_cover(params: &str) -> Result<String> {
    let comic_id = params.parse::<i64>()?;
    if let Some(path) = jm_square_cover_by_id(comic_id).await {
        return Ok(path);
    }
    let key = format!("JM_SQUARE_COVER:{}", comic_id.clone());
    let url = CLIENT.comic_cover_url_square(comic_id).await;
    use_image_cache(
        key,
        Box::pin(download_image_from_url(&url, 0, String::default())),
    )
    .await
}

async fn jm_page_image(params: &str) -> Result<String> {
    let query: PageImageQuery = serde_json::from_str(params)?;
    let key = page_image_key(query.id, &query.image_name);
    if let Some(path) = page_image_by_key(&key).await {
        return Ok(path);
    }
    let url = CLIENT
        .comic_page_url(query.id.clone(), query.image_name.clone())
        .await;
    use_image_cache(
        key,
        Box::pin(download_image_from_url(&url, query.id, query.image_name)),
    )
    .await
}

async fn jm_photo_image(params: &str) -> Result<String> {
    let key = format!("JM_PHOTO_IMAGE:{}", params,);
    let url = CLIENT.photo_url(params.to_string()).await;
    use_image_cache(
        key,
        Box::pin(download_image_from_url(&url, 0, String::default())),
    )
    .await
}

//let data = download_image_from_url(&url, page_image_flag, page_image_flag2).await?;

async fn image_size(params: &str) -> Result<String> {
    let img = image::load_from_memory(std::fs::read(params)?.as_slice())?;
    let w = img.width();
    let h = img.height();
    Ok(to_string(&ImageSize { w, h })?)
}

async fn clean_all_cache() -> Result<String> {
    clean_all_image_cache().await?;
    clean_all_web_cache().await?;
    Ok(String::default())
}

async fn http_get(params: &str) -> Result<String> {
    Ok(reqwest::ClientBuilder::new()
        .build()
        .unwrap()
        .get(params)
        .header("User-Agent", "jasmine")
        .send()
        .await?
        .text()
        .await?)
}

async fn last_search_histories(params: &str) -> Result<String> {
    let limit = params.parse::<i64>()?;
    let history = load_last_search_histories(limit).await?;
    Ok(to_string(&history)?)
}

pub(crate) fn page_image_key(chapter_id: i64, image_name: &str) -> String {
    format!("JM_PAGE_IMAGE:{}:{}", chapter_id, image_name,)
}

/// page_image_flag : 如果是pageImage, 传chapterId, 否则传0
/// page_image_flag2 : 如果是pageImage, 传name, 否则传""
pub(crate) async fn download_image_from_url(
    url: &str,
    page_image_flag: i64,
    page_image_flag2: String,
) -> Result<(bytes::Bytes, u32, u32)> {
    let agent = CLIENT.agent.lock().await;
    let req = agent.get(url);
    drop(agent);
    let data: bytes::Bytes = req
        .header("user-agent", crate::define::UA.clone())
        .send()
        .await?
        .error_for_status()?
        .bytes()
        .await?;
    Ok(check_page_image_flag(
        data,
        page_image_flag,
        page_image_flag2,
    )?)
}

fn check_page_image_flag(
    data: bytes::Bytes,
    page_image_flag: i64,
    page_image_flag2: String,
) -> Result<(bytes::Bytes, u32, u32)> {
    let src = image::load_from_memory(&data)?;
    if page_image_flag <= 220980 {
        return Ok((data, src.width(), src.height()));
    }
    let format = image::guess_format(&data)?.extensions_str()[0];
    if "gif".eq(format) {
        let src = image::load_from_memory(&data)?;
        return Ok((data, src.width(), src.height()));
    }
    let rows = if page_image_flag < 268850 {
        10
    } else {
        let md5 = md5::compute(format!(
            "{}{}",
            page_image_flag,
            page_image_flag2.split(".").nth(0).unwrap()
        ));
        let hex = hex::encode(md5.as_ref());
        let bytes = hex.as_bytes();
        let byte = bytes[bytes.len() - 1];
        if page_image_flag <= 421925 {
            ((byte as i64) % 10) * 2 + 2
        } else {
            ((byte as i64) % 8) * 2 + 2
        }
    } as u32;
    let height = src.height();
    let width = src.width();
    let remainder = height % rows;
    let mut dst = image::ImageBuffer::new(width, height);
    let mut copy_image = |src_start_x: u32,
                          src_start_y: u32,
                          dst_start_x: u32,
                          dst_start_y: u32,
                          width: u32,
                          height: u32| {
        for y in 0..height {
            for x in 0..width {
                let pixel = src.get_pixel(src_start_x + x, src_start_y + y);
                dst.put_pixel(dst_start_x + x, dst_start_y + y, pixel);
            }
        }
    };
    for x in 0..rows {
        let mut copy_h = height / rows;
        let mut py = copy_h * (x);
        let y = height - (copy_h * (x + 1)) - remainder;
        if x == 0 {
            copy_h += remainder
        } else {
            py += remainder
        }
        copy_image(0, y, 0, py, width, copy_h);
    }
    let pixels = dst.as_bytes();
    let mut file_buffer: Vec<u8> = vec![];
    PngEncoder::new(&mut file_buffer).write_image(pixels, width, height, ColorType::Rgba8)?;
    Ok((bytes::Bytes::from(file_buffer), width, height))
}

async fn set_proxy(params: &str) -> Result<String> {
    let agent = if params.len() > 0 {
        reqwest::ClientBuilder::new()
            .proxy(reqwest::Proxy::all(params)?)
            .timeout(Duration::new(30, 0))
            .build()
    } else {
        reqwest::ClientBuilder::new()
            .timeout(Duration::new(30, 0))
            .build()
    }?;
    save_property("proxy_url".to_string(), params.to_owned()).await?;
    CLIENT.set_agent(agent).await;
    Ok("".to_owned())
}

async fn get_proxy() -> Result<String> {
    load_property("proxy_url".to_string()).await
}

async fn ping_server(params: &str) -> Result<String> {
    let api_host = match params {
        "0" => ApiHost::Default,
        "1" => ApiHost::Branch1,
        "2" => ApiHost::Branch2,
        "3" => ApiHost::Branch3,
        _ => return Err(anyhow!("不支持的分流")),
    };
    let lock = CLIENT.agent.lock().await;
    let agent = lock.clone();
    drop(lock);
    let request = agent
        .get(format!("https://{}/", api_host.as_str()))
        .timeout(Duration::from_secs(10));
    let time1 = chrono::Local::now().timestamp_millis();
    let response = request.send().await?;
    let status = response.status();
    let time2 = chrono::Local::now().timestamp_millis();
    let _ = response.text().await?;
    if status.is_client_error() {
        Ok(format!("{}", time2 - time1))
    } else {
        Err(anyhow!("不支持的分流"))
    }
}

#[cfg(test)]
mod tests;

#[no_mangle]
pub unsafe extern "C" fn load_int_property(name: *const c_char, default_value: i32) -> i32 {
    let name = CStr::from_ptr(name).to_str().unwrap();
    let name = name.to_string();
    let result = RUNTIME.block_on(async move { load_property(name).await });
    match result {
        Ok(value) => {
            if value.is_empty() {
                default_value
            } else {
                match value.parse::<i32>() {
                    Ok(value) => value,
                    Err(err) => {
                        println!("{:?}", err);
                        default_value
                    }
                }
            }
        }
        Err(err) => {
            println!("{:?}", err);
            default_value
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn save_int_property(name: *const c_char, value: i32) {
    let name = CStr::from_ptr(name).to_str().unwrap();
    let name = name.to_string();
    let value = value.to_string();
    let _ = RUNTIME.block_on(async move { save_property(name, value).await });
}

fn get_home_dir() -> Result<String> {
    match std::env::var("HOME") {
        Ok(var) => Ok(var),
        Err(_) => Ok(String::default()),
    }
}

fn mkdirs(folder: &str) -> Result<String> {
    let dir = Path::new(folder);
    if !dir.exists() {
        std::fs::create_dir_all(dir)?;
    }
    Ok("".to_string())
}

async fn copy_picture_to_folder(query_str: &str) -> Result<String> {
    let query: SaveImage = serde_json::from_str(query_str)?;
    let folder = Path::new(query.folder.as_str());
    if !folder.exists() {
        tokio::fs::create_dir_all(folder).await?;
    }
    let buff = tokio::fs::read(query.path.as_str()).await?;
    let ext = image::guess_format(&buff)?.extensions_str()[0];
    let file = folder.join(format!(
        "{}{}.{}",
        chrono::Utc::now().timestamp_micros(),
        random::<u16>(),
        ext
    ));
    tokio::fs::write(file, buff).await?;
    Ok("".to_string())
}

async fn load_server_name() -> Result<String> {
    let sn = load_property("pro_server_name".to_string()).await?;
    if sn.is_empty() {
        let sn = "HK".to_string();
        Ok(sn)
    } else {
        Ok(sn)
    }
}

async fn set_pro_server_name(params: &str) -> Result<String> {
    save_property("pro_server_name".to_string(), params.to_owned()).await?;
    Ok("".to_string())
}

async fn get_pro_server_name() -> Result<String> {
    load_server_name().await
}
