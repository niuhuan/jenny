use std::collections::VecDeque;
use std::fs::create_dir_all;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use itertools::Itertools;
use lazy_static::lazy_static;
use once_cell::sync::OnceCell;
use sea_orm::ActiveValue::Set;
use sea_orm::DbErr;
use sea_orm::TransactionTrait;
use sea_orm::{ActiveModelTrait, ConnectionTrait};
use serde_json::{from_str, to_string};
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::database::active_db::{dl_album, dl_chapter, dl_image, ACTIVE_DB};
use crate::tools::join_paths;
use crate::{download_image_from_url, load_download_thread, page_image_key, CLIENT};
use crate::{DownloadCreate, DownloadCreateAlbum};
use crate::{DownloadCreateChapter, Result};

pub(crate) static DOWNLOAD_FOLDER: OnceCell<String> = OnceCell::new();

pub(crate) async fn init_dir() {
    let dir = join_paths(vec![crate::FOLDER.lock().await.deref(), "download"]);
    tokio::fs::create_dir_all(dir.clone()).await.unwrap();
    DOWNLOAD_FOLDER.set(dir).expect("INIT ACTIVE DB DUP");
}

lazy_static! {
    pub(crate) static ref RESTART_FLAG: Mutex<bool> = Mutex::new(false);
    pub(crate) static ref DOWNLOAD_AND_EXPORT_TO: Mutex<String> = Mutex::new("".to_owned());
}

async fn need_restart() -> bool {
    *RESTART_FLAG.lock().await.deref()
}

//
pub(crate) async fn start_download() {
    loop {
        // 检测重启flag
        let mut restart_flag = RESTART_FLAG.lock().await;
        if *restart_flag.deref() {
            *restart_flag = false;
        }
        drop(restart_flag);
        // 删除
        let mut need_delete = load_first_need_delete_album().await;
        while need_delete.is_some() {
            delete_file_and_database(need_delete.unwrap()).await;
            need_delete = load_first_need_delete_album().await;
        }
        // 下载
        match load_first_need_download_album().await {
            None => sleep(Duration::new(3, 0)).await,
            Some(album) => {
                println!("LOAD ALBUM : {}", album.id);
                let album_dir = join_paths(vec![
                    &DOWNLOAD_FOLDER.get().unwrap(),
                    &format!("{}", album.id),
                ]);
                create_dir_if_not_exists(&album_dir);
                download_cover(&album_dir, &album).await;
                if need_restart().await {
                    continue;
                }
                let chapters = load_chapters(&album).await;
                for chapter in &chapters {
                    let chapter_dir = join_paths(vec![&album_dir, &format!("{}", chapter.id)]);
                    create_dir_if_not_exists(&chapter_dir);

                    let images = Arc::new(Mutex::new(VecDeque::from(
                        load_all_need_download_image(&chapter).await,
                    )));

                    let _ = futures_util::future::join_all(
                        num_iter::range(0, load_download_thread().await.unwrap_or(1))
                            .map(|_| download_line(&chapter_dir, images.clone()))
                            .collect_vec(),
                    )
                    .await;

                    if need_restart().await {
                        break;
                    }

                    println!("PRE SUMMARY chapter : {}", chapter.id);
                    summary_chapter(chapter.id).await;
                }
                if need_restart().await {
                    continue;
                }
                println!("PRE SUMMARY album : {}", album.id);
                summary_album(album.id).await;
            }
        };
    }
}

pub(crate) async fn delete_file_and_database(album: dl_album::Model) {
    println!("DELETE ALBUM : {}", album.id);
    let album_dir = join_paths(vec![
        &DOWNLOAD_FOLDER.get().unwrap(),
        &format!("{}", album.id),
    ]);
    if Path::new(&album_dir).exists() {
        let _ = tokio::fs::remove_dir_all(&album_dir).await;
    }
    crate::database::active_db::clear_download_album(album.id).await;
}

async fn download_cover(album_dir: &str, album: &dl_album::Model) {
    if album.dl_3x4_cover_status == 0 {
        let url = CLIENT.comic_cover_url_3x4(album.id).await;
        let data = download_image_from_url(&url, 0, String::new()).await;
        match data {
            Err(_) => {
                dl_album::set_3x4_cover_status(
                    ACTIVE_DB.get().unwrap().lock().await.deref(),
                    album.id,
                    2,
                )
                .await
            }
            Ok((data, _, _)) => {
                tokio::fs::write(&join_paths(vec![album_dir, "cover_3x4"]), data)
                    .await
                    .unwrap();
                dl_album::set_3x4_cover_status(
                    ACTIVE_DB.get().unwrap().lock().await.deref(),
                    album.id,
                    1,
                )
                .await;
            }
        }
    }
    if album.dl_square_cover_status == 0 {
        let url = CLIENT.comic_cover_url_square(album.id).await;
        let data = download_image_from_url(&url, 0, String::new()).await;
        match data {
            Err(_) => {
                dl_album::set_square_cover_status(
                    ACTIVE_DB.get().unwrap().lock().await.deref(),
                    album.id,
                    2,
                )
                .await
            }
            Ok((data, _, _)) => {
                tokio::fs::write(&join_paths(vec![album_dir, "cover_square"]), data)
                    .await
                    .unwrap();
                dl_album::set_square_cover_status(
                    ACTIVE_DB.get().unwrap().lock().await.deref(),
                    album.id,
                    1,
                )
                .await;
            }
        }
    }
}

async fn summary_chapter(chapter_id: i64) {
    let lock = ACTIVE_DB.get().unwrap().lock().await;
    let chapter = dl_chapter::find_by_id(lock.deref(), chapter_id)
        .await
        .unwrap();
    match chapter.load_images == 1
        && dl_image::has_not_success_images(lock.deref(), chapter_id).await
    {
        true => {
            println!("SUMMARY CHAPTER : {} : FAIL", chapter_id);
            dl_chapter::set_dl_status(lock.deref(), chapter_id, 2).await
        }
        false => {
            println!("SUMMARY CHAPTER : {} : SUCCESS", chapter_id);
            dl_chapter::set_dl_status(lock.deref(), chapter_id, 1).await
        }
    };
}

async fn summary_album(album_id: i64) {
    // todo check album cover
    let lock = ACTIVE_DB.get().unwrap().lock().await;
    match dl_chapter::has_not_success_chapter(lock.deref(), album_id).await {
        true => {
            println!("SUMMARY ALBUM : {} : FAIL", album_id);
            dl_album::set_dl_status(lock.deref(), album_id, 2).await
        }
        false => {
            println!("SUMMARY ALBUM : {} : SUCCESS", album_id);
            dl_album::set_dl_status(lock.deref(), album_id, 1).await
        }
    };
}

async fn download_line(
    chapter_dir: &str,
    deque: Arc<Mutex<VecDeque<dl_image::Model>>>,
) -> Result<()> {
    loop {
        if need_restart().await {
            break;
        }
        let mut model_stream = deque.lock().await;
        let model = model_stream.pop_back();
        drop(model_stream);
        if let Some(image) = model {
            let _ = download_image(&chapter_dir, &image).await;
        } else {
            break;
        }
    }
    Ok(())
}

async fn download_image(chapter_dir: &str, image: &dl_image::Model) {
    let image = image.clone();
    let url = CLIENT
        .comic_page_url(image.chapter_id, image.name.clone())
        .await;
    let result = download_image_from_url(&url, image.chapter_id, image.name.clone()).await;
    match result {
        Err(err) => {
            println!("ERR : {}", err.to_string());
            dl_image::set_dl_status(
                ACTIVE_DB.get().unwrap().lock().await.deref(),
                image.chapter_id,
                image.image_index,
                0,
                0,
                0,
            )
            .await
        }
        Ok((buff, width, height)) => {
            {
                let exp = DOWNLOAD_AND_EXPORT_TO.lock().await;
                if !exp.is_empty() {
                    let dir = join_paths(vec![
                        exp.as_str(),
                        image.album_id.to_string().as_str(),
                        image.chapter_id.to_string().as_str(),
                    ]);
                    if !Path::new(&dir).exists() {
                        let _ = tokio::fs::create_dir_all(&dir).await;
                    }
                    drop(exp);
                    let path = join_paths(vec![&dir, &image.name]);
                    let _ = tokio::fs::write(path, buff.clone()).await;
                }
            }
            std::fs::write(
                join_paths(vec![chapter_dir, format!("{}", image.image_index).as_str()]),
                buff.clone(),
            )
            .unwrap();
            ACTIVE_DB
                .get()
                .unwrap()
                .lock()
                .await
                .transaction::<_, (), DbErr>(|db| {
                    Box::pin(async move {
                        dl_image::set_dl_status(
                            db,
                            image.chapter_id,
                            image.image_index,
                            1,
                            width.try_into().unwrap(),
                            height.try_into().unwrap(),
                        )
                        .await;
                        dl_chapter::download_one_image(db, image.chapter_id).await;
                        dl_album::download_one_image(db, image.album_id).await;
                        Ok(())
                    })
                })
                .await
                .unwrap();
        }
    }
}

fn create_dir_if_not_exists<P: AsRef<Path>>(path: P) {
    if !path.as_ref().exists() {
        create_dir_all(path).unwrap();
    }
}

async fn load_chapters(album: &dl_album::Model) -> Vec<dl_chapter::Model> {
    let album = album.clone();
    let chapters = load_all_need_download_chapter(&album).await;
    for chapter in &chapters {
        if chapter.load_images == 0 {
            let chapter = chapter.clone();
            match CLIENT.chapter(chapter.id).await {
                Err(_) => {
                    dl_chapter::set_dl_status(
                        ACTIVE_DB.get().unwrap().lock().await.deref(),
                        chapter.id,
                        1,
                    )
                    .await
                }
                Ok(load) => {
                    // 设置已经下载图片和图片个数
                    ACTIVE_DB
                        .get()
                        .unwrap()
                        .lock()
                        .await
                        .transaction::<_, (), DbErr>(|db| {
                            Box::pin(async move {
                                let images = &load.images;
                                for idx in 0..images.len() {
                                    let image = &images[idx];
                                    dl_image::ActiveModel {
                                        album_id: Set(chapter.album_id),
                                        chapter_id: Set(chapter.id),
                                        image_index: Set(idx.try_into().unwrap()),
                                        name: Set(image.to_string()),
                                        key: Set(page_image_key(chapter.id, image)),
                                        dl_status: Set(0),
                                        width: Set(0),
                                        height: Set(0),
                                        ..Default::default()
                                    }
                                    .insert(db)
                                    .await
                                    .unwrap();
                                }
                                dl_chapter::save_image_count(
                                    db,
                                    chapter.id,
                                    images.len().try_into().unwrap(),
                                )
                                .await;
                                dl_album::inc_image_count(
                                    db,
                                    album.id,
                                    images.len().try_into().unwrap(),
                                )
                                .await;
                                Ok(())
                            })
                        })
                        .await
                        .unwrap();
                }
            };
        }
    }
    load_all_need_download_chapter(&album).await
}

async fn load_first_need_download_album() -> Option<dl_album::Model> {
    return dl_album::load_first_need_download_album(ACTIVE_DB.get().unwrap().lock().await.deref())
        .await;
}

async fn load_first_need_delete_album() -> Option<dl_album::Model> {
    return dl_album::load_first_need_delete_album(ACTIVE_DB.get().unwrap().lock().await.deref())
        .await;
}

async fn load_all_need_download_chapter(album: &dl_album::Model) -> Vec<dl_chapter::Model> {
    dl_chapter::load_all_need_download_chapter(
        ACTIVE_DB.get().unwrap().lock().await.deref(),
        &album,
    )
    .await
}

async fn load_all_need_download_image(chapter: &dl_chapter::Model) -> Vec<dl_image::Model> {
    dl_image::load_all_need_download_image(ACTIVE_DB.get().unwrap().lock().await.deref(), &chapter)
        .await
}

pub(crate) async fn create_download(params: &str) -> Result<String> {
    let create: DownloadCreate = from_str(params)?;
    // todo upddate
    ACTIVE_DB
        .get()
        .unwrap()
        .lock()
        .await
        .transaction::<_, (), DbErr>(|db| {
            Box::pin(async move {
                let album = match dl_album::find_by_id(db, create.album.id).await {
                    None => {
                        dl_album::ActiveModel {
                            id: Set(create.album.id),
                            name: Set(create.album.name),
                            author: Set(to_string(&create.album.author).unwrap()),
                            tags: Set(to_string(&create.album.tags).unwrap()),
                            works: Set(to_string(&create.album.tags).unwrap()),
                            description: Set(create.album.description),
                            dl_square_cover_status: Set(0),
                            dl_3x4_cover_status: Set(0),
                            dl_status: Set(0),
                            image_count: Set(0),
                            dled_image_count: Set(0),
                            ..Default::default()
                        }
                        .insert(db)
                        .await?
                    }
                    Some(model) => {
                        dl_album::set_dl_status(db, create.album.id, 0).await;
                        model
                    }
                };
                for chapter in &create.chapters {
                    match dl_chapter::find_by_id(db, chapter.id).await {
                        None => {
                            dl_chapter::ActiveModel {
                                album_id: Set(album.id),
                                id: Set(chapter.id),
                                name: Set(chapter.name.to_string()),
                                sort: Set(chapter.sort.to_string()),
                                load_images: Set(0),
                                image_count: Set(0),
                                dled_image_count: Set(0),
                                dl_status: Set(0),
                                ..Default::default()
                            }
                            .insert(db)
                            .await?;
                            ()
                        }
                        Some(_) => (),
                    }
                }
                Ok(())
            })
        })
        .await
        .unwrap();
    Ok(String::new())
}

pub(crate) async fn download_by_id(id: i64) -> Result<String> {
    let lock = ACTIVE_DB.get().unwrap().lock().await;
    match dl_album::find_by_id(lock.deref(), id).await {
        None => Ok("null".to_owned()),
        Some(album) => {
            let chapters = dl_chapter::list_by_album_id(lock.deref(), id).await;
            Ok(to_string(&DownloadCreate {
                album: DownloadCreateAlbum {
                    id: album.id,
                    name: album.name,
                    author: from_str(&album.author).unwrap(),
                    tags: from_str(&album.tags).unwrap(),
                    works: from_str(&album.works).unwrap(),
                    description: album.description,
                },
                chapters: chapters
                    .iter()
                    .map(|chapter| DownloadCreateChapter {
                        id: chapter.id,
                        name: chapter.name.clone(),
                        sort: chapter.sort.clone(),
                    })
                    .collect_vec(),
            })
            .unwrap())
        }
    }
}

pub(crate) async fn all_downloads() -> Result<String> {
    Ok(to_string(
        &dl_album::all(ACTIVE_DB.get().unwrap().lock().await.deref()).await,
    )?)
}

pub(crate) async fn page_image_by_key(key: &str) -> Option<String> {
    match dl_image::find_by_key(ACTIVE_DB.get().unwrap().lock().await.deref(), key).await {
        None => None,
        Some(model) => Some(join_paths(vec![
            &DOWNLOAD_FOLDER.get().unwrap(),
            &model.album_id.to_string(),
            &model.chapter_id.to_string(),
            &model.image_index.to_string(),
        ])),
    }
}

pub(crate) async fn jm_3x4_cover_by_id(id: i64) -> Option<String> {
    match dl_album::find_by_id(ACTIVE_DB.get().unwrap().lock().await.deref(), id).await {
        None => None,
        Some(model) => match model.dl_3x4_cover_status {
            1 => Some(join_paths(vec![
                &DOWNLOAD_FOLDER.get().unwrap(),
                &model.id.to_string(),
                "cover_3x4",
            ])),
            _ => None,
        },
    }
}

pub(crate) async fn jm_square_cover_by_id(id: i64) -> Option<String> {
    match dl_album::find_by_id(ACTIVE_DB.get().unwrap().lock().await.deref(), id).await {
        None => None,
        Some(model) => match model.dl_square_cover_status {
            1 => Some(join_paths(vec![
                &DOWNLOAD_FOLDER.get().unwrap(),
                &model.id.to_string(),
                "cover_square",
            ])),
            _ => None,
        },
    }
}

pub(crate) async fn dl_image_by_chapter_id(chapter_id: i64) -> Result<String> {
    Ok(to_string(
        &dl_image::find_by_chapter_id(ACTIVE_DB.get().unwrap().lock().await.deref(), chapter_id)
            .await,
    )?)
}

pub(crate) async fn delete_download(id: i64) -> Result<String> {
    let lock = ACTIVE_DB.get().unwrap().lock().await;
    let mut restart_flag = RESTART_FLAG.lock().await;
    if *restart_flag.deref() {
        *restart_flag = true;
    }
    // delete_flag
    dl_album::set_dl_status(lock.deref(), id, 3).await;
    drop(restart_flag);
    Ok(String::default())
}

pub(crate) async fn delete_download_no_lock(db: &impl ConnectionTrait, id: i64) -> Result<String> {
    let mut restart_flag = RESTART_FLAG.lock().await;
    if *restart_flag.deref() {
        *restart_flag = true;
    }
    // delete_flag
    dl_album::set_dl_status(db, id, 3).await;
    drop(restart_flag);
    Ok(String::default())
}

pub(crate) async fn renew_all_downloads() -> Result<String> {
    let lock = ACTIVE_DB.get().unwrap().lock().await;
    let mut restart_flag = RESTART_FLAG.lock().await;
    if *restart_flag.deref() {
        *restart_flag = true;
    }
    lock.transaction::<_, (), DbErr>(|db| {
        Box::pin(async move {
            dl_album::renew_failed(db).await;
            dl_chapter::renew_failed(db).await;
            dl_image::renew_failed(db).await;
            Ok(())
        })
    })
    .await?;
    Ok(String::default())
}
