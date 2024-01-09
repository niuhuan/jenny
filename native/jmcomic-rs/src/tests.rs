use serde_json::json;

use crate::{Client, FavoritesOrder, SortBy};
use crate::{Method, Result};

/// 打印结果
fn print<T: serde::Serialize>(result: Result<T>) {
    match result {
        Ok(data) => println!("{}", serde_json::to_string(&data).unwrap()),
        Err(err) => panic!("{}", err),
    }
}

async fn client() -> Client {
    let client = Client::new();
    client.set_user_agent(Client::rand_user_agent()).await;
    client
}

#[tokio::test]
async fn request_data() {
    let rsp = client().await
        .request_data(Method::GET, "setting", json!({}))
        .await;
    match rsp {
        Ok(text) => println!("{}", text),
        Err(err) => panic!("{}", err),
    }
}

async fn login_client() -> Client {
    let client = client().await;
    client
        .login("username".to_owned(), "password".to_owned())
        .await
        .unwrap();
    client
}

#[tokio::test]
async fn login_request_data() {
    match login_client()
        .await
        .request_data(Method::GET, "favorite", json!({"page":1}))
        .await
    {
        Ok(text) => println!("{}", text),
        Err(err) => panic!("{}", err),
    }
}

#[tokio::test]
async fn categories() {
    print(client().await.categories().await)
}

#[tokio::test]
async fn latest() {
    print(client().await.latest().await)
}

#[tokio::test]
async fn comics() {
    print(client().await.comics("".to_string(), SortBy::Default, 1).await)
}

#[tokio::test]
async fn album() {
    print(client().await.album(215435).await)
}

#[tokio::test]
async fn chapter() {
    print(client().await.chapter(215435).await)
}

#[tokio::test]
async fn videos() {
    print(client().await.videos(SortBy::View, 2).await)
}

#[tokio::test]
async fn forum() {
    print(client().await.forum(None, None, 100).await)
}

#[tokio::test]
async fn set_favorite() {
    print(login_client().await.set_favorite(302608).await);
}

#[tokio::test]
async fn favorites() {
    print(
        login_client()
            .await
            .favorites(0, 1, FavoritesOrder::Mp)
            .await,
    );
}

#[tokio::test]
async fn favorites_intro() {
    print(login_client().await.favorites_intro().await);
}

#[tokio::test]
async fn create_favorite_folder() {
    print(
        login_client()
            .await
            .create_favorite_folder("MY_FOLDER".to_string())
            .await,
    );
}

#[tokio::test]
async fn games() {
    print(client().await.games(1).await);
}

#[tokio::test]
async fn comics_search() {
    print(
        client().await
            .comics_search("ABC".to_owned(), SortBy::Default, 1)
            .await,
    );
}

#[tokio::test]
async fn test() {}
