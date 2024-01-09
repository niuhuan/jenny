use std::ops::Deref;
use std::path::Path;

use anyhow::{anyhow, Context};
use async_zip::{Compression, ZipEntryBuilder, ZipEntryBuilderExt};
use image::EncodableLayout;
use itertools::Itertools;
use sea_orm::{ConnectionTrait, DbErr, IntoActiveModel};
use sea_orm::{EntityTrait, TransactionTrait};
use serde_json::{from_str, to_string};
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufWriter};

use crate::database::active_db::{dl_album, dl_chapter, dl_image, ACTIVE_DB};
use crate::download::{delete_download_no_lock, DOWNLOAD_FOLDER};
use crate::tools::join_paths;
use crate::{is_pro, ExportQuery, ExportSingleQuery, Result};

use serde_derive::Deserialize;
use serde_derive::Serialize;

#[async_trait::async_trait]
trait ArchiveWriter {
    async fn write_to(&mut self, path: String, data: &mut [u8]) -> Result<()>;
    async fn finish(mut self) -> Result<()>;
}

struct JmiWriter<'a>(async_zip::write::ZipFileWriter<&'a mut BufWriter<File>>);
struct ZipWriter<'a>(async_zip::write::ZipFileWriter<&'a mut BufWriter<File>>);
struct JpegsWriter<'a>(&'a String);

const K: u8 = 170;

#[async_trait::async_trait]
impl ArchiveWriter for JmiWriter<'_> {
    async fn write_to(&mut self, path: String, data: &'_ mut [u8]) -> Result<()> {
        for i in 0..data.len() {
            data[i] ^= K;
        }
        let builder = ZipEntryBuilder::new(path, Compression::Deflate);
        let builder = builder.unix_permissions(644);
        self.0.write_entry_whole(builder, data).await?;
        Ok(())
    }
    async fn finish(self) -> Result<()> {
        self.0.close().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl ArchiveWriter for ZipWriter<'_> {
    async fn write_to(&mut self, path: String, data: &mut [u8]) -> Result<()> {
        let builder = ZipEntryBuilder::new(path, Compression::Deflate);
        let builder = builder.unix_permissions(644);
        self.0.write_entry_whole(builder, data).await?;
        Ok(())
    }
    async fn finish(self) -> Result<()> {
        self.0.close().await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl ArchiveWriter for JpegsWriter<'_> {
    async fn write_to(&mut self, path: String, data: &mut [u8]) -> Result<()> {
        let path = join_paths(vec![self.0.as_str(), path.as_str()]);
        tokio::fs::write(path, data).await?;
        Ok(())
    }
    async fn finish(mut self) -> Result<()> {
        Ok(())
    }
}

#[async_trait::async_trait]
trait ArchiveReader {
    async fn read_path(
        &self,
        reader: &mut async_zip::read::fs::ZipFileReader,
        path: &str,
    ) -> Result<Vec<u8>>;
}

struct JmiReader;
struct ZipReader;

#[async_trait::async_trait]
impl ArchiveReader for JmiReader {
    async fn read_path(
        &self,
        reader: &mut async_zip::read::fs::ZipFileReader,
        path: &str,
    ) -> Result<Vec<u8>> {
        let entry = reader
            .entry(path)
            .with_context(|| format!("not found {}", path))?;
        let mut e = reader.entry_reader(entry.0).await?;
        let mut data = vec![];
        e.read_to_end(&mut data).await?;
        for i in 0..data.len() {
            data.as_mut_slice()[i] ^= K;
        }
        Ok(data)
    }
}

#[async_trait::async_trait]
impl ArchiveReader for ZipReader {
    async fn read_path(
        &self,
        reader: &mut async_zip::read::fs::ZipFileReader,
        path: &str,
    ) -> Result<Vec<u8>> {
        let entry = reader
            .entry(path)
            .with_context(|| format!("not found {}", path))?;
        let mut e = reader.entry_reader(entry.0).await?;
        let mut data = vec![];
        e.read_to_end(&mut data).await?;
        Ok(data)
    }
}

fn local_name(name: &str) -> String {
    name.replace("\\", "_")
        .replace("/", "_")
        .replace("*", "_")
        .replace("%", "_")
        .replace("&", "_")
        .replace("$", "_")
        .replace(" ", "_")
        .replace("(", "_")
        .replace(")", "_")
        .replace("[", "_")
        .replace("]", "_")
        .replace("?", "_")
        .replace("<", "_")
        .replace(">", "_")
        .replace("|", "_")
        .replace("\"", "_")
        .replace("'", "_")
}

pub(crate) async fn export_jm_jpegs(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }
    let mut paths: Vec<String> = vec![];
    let query: ExportQuery = from_str(params)?;
    let db = ACTIVE_DB.get().unwrap().lock().await;
    for comic_id in query.comic_id {
        let ab = dl_album::find_by_id(db.deref(), comic_id)
            .await
            .with_context(|| "not found")?;
        let archive_path = join_paths(vec![
            query.dir.as_str(),
            format!(
                "{}-{}",
                local_name(ab.name.as_str()),
                chrono::Local::now().timestamp()
            )
            .as_str(),
        ]);
        tokio::fs::create_dir_all(archive_path.as_str()).await?;
        put_comic_to_zip(Box::new(JpegsWriter(&archive_path)), db.deref(), ab, true).await?;
        paths.push(archive_path);
        if query.delete_exported {
            let _ = delete_download_no_lock(db.deref(), comic_id).await;
        }
    }
    Ok(to_string(&paths)?)
}

pub(crate) async fn export_jm_zip(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }
    let mut paths: Vec<String> = vec![];
    let query: ExportQuery = from_str(params)?;
    let db = ACTIVE_DB.get().unwrap().lock().await;
    for comic_id in query.comic_id {
        let ab = dl_album::find_by_id(db.deref(), comic_id)
            .await
            .with_context(|| "not found")?;
        let archive_path = join_paths(vec![
            query.dir.as_str(),
            format!(
                "{}-{}.jm.zip",
                local_name(ab.name.as_str()),
                chrono::Local::now().timestamp()
            )
            .as_str(),
        ]);

        let writer_file = tokio::fs::File::create(archive_path.as_str()).await;
        if writer_file.is_ok() {
            let writer_file = writer_file.unwrap();
            let mut buff_writer = tokio::io::BufWriter::new(writer_file);
            let writer = async_zip::write::ZipFileWriter::new(&mut buff_writer);
            let write_result1 =
                put_comic_to_zip(Box::new(ZipWriter(writer)), db.deref(), ab, false).await;
            let write_result3 = buff_writer.flush().await;
            if write_result1.is_err() {
                // todo delete file
                return Err(anyhow!("{}", write_result1.err().unwrap().to_string()));
            }
            if write_result3.is_err() {
                // todo delete file
                return Err(anyhow!(
                    "{} : {}",
                    write_result3.err().unwrap().to_string(),
                    archive_path
                ));
            }
        } else {
            // todo
            return Err(anyhow!("{}", writer_file.err().unwrap().to_string()));
        }
        paths.push(archive_path);
        if query.delete_exported {
            let _ = delete_download_no_lock(db.deref(), comic_id).await;
        }
    }
    Ok(to_string(&paths)?)
}

pub(crate) async fn export_jm_zip_single(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }

    let query: ExportSingleQuery = from_str(params)?;
    let db = ACTIVE_DB.get().unwrap().lock().await;
    let ab = dl_album::find_by_id(db.deref(), query.id)
        .await
        .with_context(|| "not found")?;

    let archive_path = join_paths(vec![
        query.folder.as_str(),
        format!(
            "{}-{}.jm.zip",
            local_name(
                if let Some(rename) = query.rename {
                    rename
                } else {
                    ab.name.clone()
                }
                .as_str()
            ),
            chrono::Local::now().timestamp()
        )
        .as_str(),
    ]);
    let writer_file = tokio::fs::File::create(archive_path.as_str()).await;
    if writer_file.is_ok() {
        let writer_file = writer_file.unwrap();
        let mut buff_writer = tokio::io::BufWriter::new(writer_file);
        let writer = async_zip::write::ZipFileWriter::new(&mut buff_writer);
        let write_result1 =
            put_comic_to_zip(Box::new(ZipWriter(writer)), db.deref(), ab, false).await;
        let write_result3 = buff_writer.flush().await;
        if write_result1.is_err() {
            // todo delete file
            return Err(anyhow!("{}", write_result1.err().unwrap().to_string()));
        }
        if write_result3.is_err() {
            // todo delete file
            return Err(anyhow!(
                "{} : {}",
                write_result3.err().unwrap().to_string(),
                archive_path
            ));
        }
    } else {
        // todo
        return Err(anyhow!("{}", writer_file.err().unwrap().to_string()));
    }
    if query.delete_exported {
        let _ = delete_download_no_lock(db.deref(), query.id).await;
    }
    Ok(archive_path)
}

pub(crate) async fn export_jm_jpegs_zip_single(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }

    let query: ExportSingleQuery = from_str(params)?;
    let db = ACTIVE_DB.get().unwrap().lock().await;
    let ab = dl_album::find_by_id(db.deref(), query.id)
        .await
        .with_context(|| "not found")?;

    let archive_path = join_paths(vec![
        query.folder.as_str(),
        format!(
            "{}-{}.jm.jpegs.zip",
            local_name(
                if let Some(rename) = query.rename {
                    rename
                } else {
                    ab.name.clone()
                }
                .as_str()
            ),
            chrono::Local::now().timestamp()
        )
        .as_str(),
    ]);
    let writer_file = tokio::fs::File::create(archive_path.as_str()).await;
    if writer_file.is_ok() {
        let writer_file = writer_file.unwrap();
        let mut buff_writer = tokio::io::BufWriter::new(writer_file);
        let writer = async_zip::write::ZipFileWriter::new(&mut buff_writer);
        let write_result1 =
            put_comic_to_zip(Box::new(ZipWriter(writer)), db.deref(), ab, true).await;
        let write_result3 = buff_writer.flush().await;
        if write_result1.is_err() {
            // todo delete file
            return Err(anyhow!("{}", write_result1.err().unwrap().to_string()));
        }
        if write_result3.is_err() {
            // todo delete file
            return Err(anyhow!(
                "{} : {}",
                write_result3.err().unwrap().to_string(),
                archive_path
            ));
        }
    } else {
        // todo
        return Err(anyhow!("{}", writer_file.err().unwrap().to_string()));
    }
    if query.delete_exported {
        let _ = delete_download_no_lock(db.deref(), query.id).await;
    }
    Ok(archive_path)
}

pub(crate) async fn export_jm_jmi(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }
    let mut paths: Vec<String> = vec![];
    let query: ExportQuery = from_str(params)?;
    let db = ACTIVE_DB.get().unwrap().lock().await;
    for comic_id in query.comic_id {
        let ab = dl_album::find_by_id(db.deref(), comic_id)
            .await
            .with_context(|| "not found")?;
        let archive_path = join_paths(vec![
            query.dir.as_str(),
            format!(
                "{}-{}.jmi",
                local_name(ab.name.as_str()),
                chrono::Local::now().timestamp()
            )
            .as_str(),
        ]);

        let writer_file = tokio::fs::File::create(archive_path.as_str()).await;
        if writer_file.is_ok() {
            let writer_file = writer_file.unwrap();
            let mut buff_writer = tokio::io::BufWriter::new(writer_file);
            let writer = async_zip::write::ZipFileWriter::new(&mut buff_writer);
            let write_result1 =
                put_comic_to_zip(Box::new(JmiWriter(writer)), db.deref(), ab, false).await;
            let write_result3 = buff_writer.flush().await;
            if write_result1.is_err() {
                // todo delete file
                return Err(anyhow!(
                    "{} : {}",
                    write_result1.err().unwrap().to_string(),
                    archive_path
                ));
            }
            if write_result3.is_err() {
                // todo delete file
                return Err(anyhow!(
                    "{} : {}",
                    write_result3.err().unwrap().to_string(),
                    archive_path
                ));
            }
        } else {
            // todo
            return Err(anyhow!(
                "{} : {}",
                writer_file.err().unwrap().to_string(),
                archive_path
            ));
        }
        paths.push(archive_path);
        if query.delete_exported {
            let _ = delete_download_no_lock(db.deref(), comic_id).await;
        }
    }
    Ok(to_string(&paths)?)
}

pub(crate) async fn export_jm_jmi_single(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }

    let query: ExportSingleQuery = from_str(params)?;
    let db = ACTIVE_DB.get().unwrap().lock().await;
    let ab = dl_album::find_by_id(db.deref(), query.id)
        .await
        .with_context(|| "not found")?;

    let archive_path = join_paths(vec![
        query.folder.as_str(),
        format!(
            "{}-{}.jmi",
            local_name(
                if let Some(rename) = query.rename {
                    rename
                } else {
                    ab.name.clone()
                }
                .as_str()
            ),
            chrono::Local::now().timestamp()
        )
        .as_str(),
    ]);
    let writer_file = tokio::fs::File::create(archive_path.as_str()).await;
    if writer_file.is_ok() {
        let writer_file = writer_file.unwrap();
        let mut buff_writer = tokio::io::BufWriter::new(writer_file);
        let writer = async_zip::write::ZipFileWriter::new(&mut buff_writer);
        let write_result1 =
            put_comic_to_zip(Box::new(JmiWriter(writer)), db.deref(), ab, false).await;
        let write_result3 = buff_writer.flush().await;
        if write_result1.is_err() {
            // todo delete file
            return Err(anyhow!(
                "{} : {}",
                write_result1.err().unwrap().to_string(),
                archive_path
            ));
        }
        if write_result3.is_err() {
            // todo delete file
            return Err(anyhow!(
                "{} : {}",
                write_result3.err().unwrap().to_string(),
                archive_path
            ));
        }
    } else {
        // todo
        return Err(anyhow!("{}", writer_file.err().unwrap().to_string()));
    }
    if query.delete_exported {
        let _ = delete_download_no_lock(db.deref(), query.id).await;
    }
    Ok(archive_path)
}

pub(crate) async fn import_jm_zip(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }
    import_archive(params, ZipReader).await
}

pub(crate) async fn import_jm_jmi(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }
    import_archive(params, JmiReader).await
}

pub(crate) async fn import_jm_dir(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }

    let paths = std::fs::read_dir(params).unwrap();
    for path in paths {
        let entry = path?;
        let path = entry
            .path()
            .to_str()
            .with_context(|| "What's up")?
            .to_owned();
        if path.ends_with(".jm.zip") {
            let _ = import_jm_zip(entry.path().to_str().with_context(|| "What's up")?).await;
        } else if path.ends_with(".jmi") {
            let _ = import_jm_jmi(entry.path().to_str().with_context(|| "What's up")?).await;
        }
    }
    Ok("".to_string())
}

async fn import_archive(params: &str, x: impl ArchiveReader) -> Result<String> {
    let mut zip = async_zip::read::fs::ZipFileReader::new(params).await?;
    // 读取基本信息
    let comic: dl_album::Model =
        from_str(String::from_utf8(x.read_path(&mut zip, "comic.json").await?)?.as_str())?;
    let chapters: Vec<dl_chapter::Model> =
        from_str(String::from_utf8(x.read_path(&mut zip, "chapters.json").await?)?.as_str())?;
    let images: Vec<dl_image::Model> =
        from_str(String::from_utf8(x.read_path(&mut zip, "images.json").await?)?.as_str())?;

    // 删除数据库
    let db = ACTIVE_DB.get().unwrap().lock().await;
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            dl_album::delete_by_album_id(txn, comic.id).await?;
            dl_chapter::delete_by_album_id(txn, comic.id).await?;
            dl_image::delete_by_album_id(txn, comic.id).await?;
            Ok(())
        })
    })
    .await?;
    // 删除文件夹
    let album_dir = join_paths(vec![
        &DOWNLOAD_FOLDER.get().unwrap(),
        &format!("{}", comic.id),
    ]);
    if Path::new(&album_dir).exists() {
        let _ = tokio::fs::remove_dir_all(&album_dir).await;
    }
    // 导入数据库
    let images1 = images.clone();
    db.transaction::<_, (), DbErr>(|txn| {
        Box::pin(async move {
            dl_album::Entity::insert(comic.clone().into_active_model())
                .exec(txn)
                .await?;
            dl_chapter::Entity::insert_many(
                chapters
                    .iter()
                    .map(|x| x.clone().into_active_model())
                    .collect_vec(),
            )
            .exec(txn)
            .await?;
            dl_image::Entity::insert_many(
                images1
                    .iter()
                    .map(|x| x.clone().into_active_model())
                    .collect_vec(),
            )
            .exec(txn)
            .await?;
            Ok(())
        })
    })
    .await?;

    // 导入logo
    if !Path::new(&album_dir).exists() {
        tokio::fs::create_dir_all(&album_dir).await?;
    }
    let path_3x4_cover = join_paths(vec![album_dir.as_str(), "cover_3x4"]);
    let path_cover_square = join_paths(vec![album_dir.as_str(), "cover_square"]);
    {
        let image_data = x.read_path(&mut zip, "cover_3x4").await?;
        tokio::fs::write(&path_3x4_cover, image_data).await?;
    }
    {
        let image_data = x.read_path(&mut zip, "cover_square").await?;
        tokio::fs::write(&path_cover_square, image_data).await?;
    }

    // 导入图片
    for image in images {
        let chapter_dir = join_paths(vec![album_dir.as_str(), &format!("{}", image.chapter_id)]);
        if !Path::new(&chapter_dir).exists() {
            tokio::fs::create_dir_all(&chapter_dir).await?;
        }
        let image_path = join_paths(vec![
            chapter_dir.as_str(),
            format!("{}", image.image_index).as_str(),
        ]);
        let in_zip = format!(
            "{}_{}_{}",
            image.album_id, image.chapter_id, image.image_index
        );
        let buff = x.read_path(&mut zip, in_zip.as_str()).await?;
        tokio::fs::write(image_path.as_str(), buff).await?;
    }
    Ok("".to_owned())
}

async fn put_comic_to_zip<A: ArchiveWriter + Sync + Send>(
    mut x: Box<A>,
    db: &impl ConnectionTrait,
    ab: dl_album::Model,
    ext: bool,
) -> Result<()> {
    let chapters = dl_chapter::list_by_album_id(db, ab.id.clone()).await;
    let images = dl_image::lisst_by_album_id(db, ab.id.clone()).await?;

    let comic_json_str = to_string(&ab)?;
    let chapters_json_str = to_string(&chapters)?;
    let images_json_str = to_string(&images)?;

    x.write_to(
        "comic.json".to_string(),
        &mut comic_json_str.as_bytes().to_vec(),
    )
    .await?;

    x.write_to(
        "comic.js".to_string(),
        &mut format!("comic = {}", comic_json_str).as_bytes().to_vec(),
    )
    .await?;

    x.write_to(
        "chapters.json".to_string(),
        &mut chapters_json_str.as_bytes().to_vec(),
    )
    .await?;

    x.write_to(
        "chapters.js".to_string(),
        &mut format!("chapters = {}", chapters_json_str)
            .as_bytes()
            .to_vec(),
    )
    .await?;

    x.write_to(
        "images.json".to_string(),
        &mut images_json_str.as_bytes().to_vec(),
    )
    .await?;

    x.write_to(
        "images.js".to_string(),
        &mut format!("images = {}", images_json_str).as_bytes().to_vec(),
    )
    .await?;

    let album_dir = join_paths(vec![&DOWNLOAD_FOLDER.get().unwrap(), &format!("{}", ab.id)]);

    // 写logo
    let path_3x4_cover = join_paths(vec![album_dir.as_str(), "cover_3x4"]);
    let path_cover_square = join_paths(vec![album_dir.as_str(), "cover_square"]);

    if Path::new(&path_3x4_cover).exists() {
        let mut image_data = tokio::fs::read(&path_3x4_cover).await?;
        x.write_to("cover_3x4".to_owned(), &mut image_data).await?;
    }
    if Path::new(&path_cover_square).exists() {
        let mut image_data = tokio::fs::read(&path_cover_square).await?;
        x.write_to("cover_square".to_owned(), &mut image_data)
            .await?;
    }

    // 写入图片
    for image in images {
        let chapter_dir = join_paths(vec![album_dir.as_str(), &format!("{}", image.chapter_id)]);
        let image_path = join_paths(vec![
            chapter_dir.as_str(),
            format!("{}", image.image_index).as_str(),
        ]);
        let mut in_zip = format!(
            "{}_{}_{}",
            image.album_id, image.chapter_id, image.image_index
        );
        if ext {
            in_zip = format!("{}.jpg", in_zip)
        }
        if image.dl_status == 1 {
            let mut image_data = tokio::fs::read(&image_path).await?;
            x.write_to(in_zip, &mut image_data).await?;
        }
    }

    //写html
    x.write_to("index.html".to_string(), &mut HTML.as_bytes().to_vec())
        .await?;

    x.finish().await?;

    Ok(())
}

const HTML: &str = "
<!DOCTYPE html>
<html>
<head>
    <meta charset=\"UTF-8\">
    <style>
        * {
            margin: 0;
            padding: 0;
        }
        html {
            color: white;
        }
        html, body {
            height: 100%;
        }
        #leftNav {
            position: fixed;
            width: 350px;
            height: 100%;
        }
        #leftNav > * {
            display: inline-block;
            vertical-align: top;
        }
        #leftNav > ul {
            background: #333;
            height: 100%;
            overflow-y: auto;
            overflow-x: hidden;
            width: 300px;
        }
        #leftNav > #slider {
            margin-top: 1em;
            float: right;
            width: 40px;
            border: none;
            background: rgba(0, 0, 0, .4);
            color: white;
        }
        #title > img {
            display: block;
            width: 80px;
            margin: 30px auto;
        }
        #title > p {
            text-align: center;
        }
        #title {
            margin-bottom: 30px;
        }
        #leftNav > ul a {
            margin: auto;
            display: block;
            color: white;
            height: 40px;
            line-height: 40px;
            text-align: center;
            width: 280px;
            border-top: #666 solid 1px;
            text-decoration: none;
        }
        #leftNav > ul a:hover,#leftNav > ul a.active {
            background: rgba(255, 255, 255, .1);
        }
        #content {
            width: 100%;
            height: 100%;
            background: black;
        }
        #content img {
            width: 100%;
        }
    </style>   
    <script src=\"comic.js\"></script>
    <script src=\"chapters.js\"></script>
    <script src=\"images.js\"></script>
    <script>
        function changeLeftNav() {
            var doc = document.getElementById(\"leftNav\")
            if (doc.style.left) {
                doc.style.left = \"\"
            } else {
                doc.style.left = \"-300px\"
            }
        }
        function changeEp(epIndex) {
            var epId = chapters[epIndex].id;
            var ps = [];
            for (var i = 0; i < images.length; i++) {
                if (images[i].chapter_id = epId) {
                    ps.push(images[i]);
                }
            }
            document.getElementById('content').innerHTML = \"\";
            var d = document.createElement('div');
            d.id = 'd';
            document.getElementById('content').append(d);
            for (var i = 0; i < ps.length; i++) {
                var img = document.createElement('img');
                img.src = comic.id + '_' + epId + '_' + ps[i].image_index;
                document.getElementById('content').append(img);
            }
            document.getElementById('d').scrollIntoView();
            changeLeftNav();
            var as = document.getElementById('leftNav').getElementsByTagName('a');
            for (var i = 0; i < ps.length; i++) {
                if(epIndex == i){
                    as[i].classList = [\"active\"];
                }else{
                    as[i].className = \"\";
                }
            }
        }
    </script>
</head>
<body>
<div id=\"leftNav\">
    <ul>
        <li id=\"title\">
            <script>
                document.write('<img src=\"cover_3x4\" /> <br/>')
                document.write('<p>' + comic.name + '</p>');
            </script>
        </li>
        <script>
            for (var i = 0; i < chapters.length; i++) {
                document.write('<li><a href=\"javascript:changeEp(' + i + ')\">' +chapters[i].name + '</a></li>');
            }
        </script>
    </ul>
    <button id=\"slider\" onclick=\"changeLeftNav();\">切换</button>
</div>
<div id=\"content\">
</div>
</body>
</html>
";

pub(crate) async fn export_cbzs_zip_single(params: &str) -> Result<String> {
    if !is_pro().await?.is_pro {
        return Err(anyhow!("请先发电鸭"));
    }
    let query: ExportSingleQuery = from_str(params)?;
    let db = ACTIVE_DB.get().unwrap().lock().await;
    // 找到漫画
    let ab = dl_album::find_by_id(db.deref(), query.id)
        .await
        .with_context(|| "not found")?;
    // 创建文件并创建zip输出流
    let archive_path = join_paths(vec![
        query.folder.as_str(),
        format!(
            "{}-{}.cbzs.zip",
            local_name(
                if let Some(rename) = query.rename {
                    rename
                } else {
                    ab.name.clone()
                }
                .as_str()
            ),
            chrono::Local::now().timestamp()
        )
        .as_str(),
    ]);
    let writer_file = tokio::fs::File::create(archive_path.as_str()).await;
    if writer_file.is_ok() {
        let mut writer_file = writer_file.unwrap();
        let writer = async_zip::write::ZipFileWriter::new(&mut writer_file);
        // 导出内容
        let write_result1 = export_cbzs_zip_single_export(writer, db.deref(), ab).await;
        // 关闭文件
        drop(writer_file);
        if write_result1.is_err() {
            // todo delete file
            return Err(anyhow!(
                "{} : {}",
                write_result1.err().unwrap().to_string(),
                archive_path
            ));
        }
    } else {
        // todo
        return Err(anyhow!("{}", writer_file.err().unwrap().to_string()));
    }
    if query.delete_exported {
        let _ = delete_download_no_lock(db.deref(), query.id).await;
    }
    Ok(archive_path)
}

pub(crate) async fn export_cbzs_zip_single_export(
    mut x: async_zip::write::ZipFileWriter<&mut File>,
    db: &impl ConnectionTrait,
    ab: dl_album::Model,
) -> Result<()> {
    // chpaters
    let chapters = dl_chapter::list_by_album_id(db, ab.id.clone()).await;
    let mut chapters = chapters
        .into_iter()
        .map(|c| CbzChapter {
            album_id: c.album_id,
            id: c.id,
            name: c.name,
            sort: c.sort,
            // 0:未加载图片 1:已加载图片
            load_images: c.load_images,
            // 图片总数
            image_count: c.image_count,
            // 下载了的图片总数
            dled_image_count: c.dled_image_count,
            // image(图片的下载状态)
            // 0:未下载, 1:全部下载成功 2:任何一个下载失败
            // "JM_PAGE_IMAGE:{}:{}"
            dl_status: c.dl_status,
            images: vec![],
            cbz_name: String::default(),
        })
        .collect::<Vec<CbzChapter>>();
    chapters.sort_by_key(|c| c.sort.clone());
    let mut map = std::collections::HashMap::<i64, &mut CbzChapter>::new();
    for chapter in &mut chapters {
        map.insert(chapter.id.clone(), chapter);
    }
    // images push to cbzChpater
    let images = dl_image::lisst_by_album_id(db, ab.id.clone()).await?;
    for image in images {
        if let Some(c) = map.get_mut(&image.chapter_id) {
            (*c).images.push(CbzImage {
                album_id: image.album_id,
                chapter_id: image.chapter_id,
                image_index: image.image_index,
                name: image.name,
                key: image.key,
                dl_status: image.dl_status,
                width: image.width,
                height: image.height,
                file_name: "".to_string(),
            });
        }
    }
    drop(map);
    let mut i = 1;
    for chapter in &mut chapters {
        chapter.cbz_name = format!("{:04}.cbz", {
            let tmp = i;
            i += 1;
            tmp
        });
        for image in &mut chapter.images {
            image.file_name = format!("{:04}.jpg", image.image_index);
        }
    }
    // album
    let ab = CbzAlbum {
        id: ab.id,
        name: ab.name,
        author: ab.author,
        tags: ab.tags,
        works: ab.works,
        description: ab.description,
        dl_square_cover_status: ab.dl_square_cover_status,
        dl_3x4_cover_status: ab.dl_3x4_cover_status,
        dl_status: ab.dl_status,
        image_count: ab.image_count,
        dled_image_count: ab.dled_image_count,
        chapters,
    };
    // export
    let album_dir = join_paths(vec![&DOWNLOAD_FOLDER.get().unwrap(), &format!("{}", ab.id)]);
    for chapter in &ab.chapters {
        let chapter_dir = join_paths(vec![album_dir.as_str(), &format!("{}", chapter.id)]);
        let builder = ZipEntryBuilder::new(chapter.cbz_name.clone(), Compression::Deflate);
        let builder = builder.unix_permissions(644);
        let mut cbz_entry = x.write_entry_stream(builder).await?;
        let mut cbz_writer = async_zip::write::ZipFileWriter::new(&mut cbz_entry);
        for image in &chapter.images {
            let image_path = join_paths(vec![
                chapter_dir.as_str(),
                format!("{}", image.image_index).as_str(),
            ]);
            let image_data = tokio::fs::read(&image_path).await?;
            let builder = ZipEntryBuilder::new(image.file_name.clone(), Compression::Deflate);
            let builder = builder.unix_permissions(644);
            cbz_writer
                .write_entry_whole(builder, image_data.as_bytes())
                .await?;
        }
        cbz_writer.close().await?;
        cbz_entry.close().await?;
    }
    // 写logo
    let path_3x4_cover = join_paths(vec![album_dir.as_str(), "cover_3x4"]);
    let path_cover_square = join_paths(vec![album_dir.as_str(), "cover_square"]);
    if Path::new(&path_3x4_cover).exists() {
        let image_data = tokio::fs::read(&path_3x4_cover).await?;
        let builder = ZipEntryBuilder::new("cover_3x4.jpg".to_owned(), Compression::Deflate);
        let builder = builder.unix_permissions(644);
        x.write_entry_whole(builder, image_data.as_bytes()).await?;
    }
    if Path::new(&path_cover_square).exists() {
        let image_data = tokio::fs::read(&path_cover_square).await?;
        let builder = ZipEntryBuilder::new("cover_square.jpg".to_owned(), Compression::Deflate);
        let builder = builder.unix_permissions(644);
        x.write_entry_whole(builder, image_data.as_bytes()).await?;
    }
    // 写数据
    let json = to_string(&ab)?;
    let builder = ZipEntryBuilder::new("z-jm-cbzs-info.json".to_owned(), Compression::Deflate);
    let builder = builder.unix_permissions(644);
    x.write_entry_whole(builder, json.as_bytes()).await?;
    //
    x.close().await?;
    Ok(())
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct CbzAlbum {
    pub id: i64,
    pub name: String,
    pub author: String,
    pub tags: String,
    pub works: String,
    pub description: String,
    pub dl_square_cover_status: i32,
    pub dl_3x4_cover_status: i32,
    pub dl_status: i32,
    pub image_count: i32,
    pub dled_image_count: i32,
    pub chapters: Vec<CbzChapter>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct CbzChapter {
    pub album_id: i64,
    pub id: i64,
    pub name: String,
    pub sort: String,
    pub load_images: i32,
    pub image_count: i32,
    pub dled_image_count: i32,
    pub dl_status: i32,
    pub images: Vec<CbzImage>,
    pub cbz_name: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, Eq, Hash, PartialEq)]
pub struct CbzImage {
    pub album_id: i64,
    pub chapter_id: i64,
    pub image_index: i64,
    pub name: String,
    pub key: String,
    pub dl_status: i32,
    pub width: u32,
    pub height: u32,
    pub file_name: String,
}
