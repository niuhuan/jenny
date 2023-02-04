use anyhow::Result;

const OWNER: &str = "niuhuan";
const REPO: &str = "jenny";

#[tokio::main]
async fn main() -> Result<()> {
    let vs_code_txt = tokio::fs::read_to_string("version.code.txt").await?;
    let vs_info_txt = tokio::fs::read_to_string("version.info.txt").await?;

    let info = format!(
        "{REPO} 版本 {vs_code_txt} 发布! \n\n更新内容:\n{vs_info_txt}\n\nhttps://github.com/{OWNER}/{REPO}/releases/tag/{vs_code_txt}",
    );

    // get accounts
    let tg_token = match std::env::var("TG_BOT_TOKEN") {
        Ok(s) => s,
        Err(_) => "".to_owned(),
    };
    let tg_chat_id = match std::env::var("TG_CHAT_IDS") {
        Ok(s) => s,
        Err(_) => "".to_owned(),
    };
    let dc_token = match std::env::var("DISCORD_BOT_TOKEN") {
        Ok(s) => s,
        Err(_) => "".to_owned(),
    };
    let dc_chat_id = match std::env::var("DISCORD_CHAT_IDS") {
        Ok(s) => s,
        Err(_) => "".to_owned(),
    };

    println!("loaded");

    if !tg_token.is_empty() && !tg_chat_id.is_empty() {
        match send_to_tg(tg_token, tg_chat_id, info.clone()).await {
            Ok(_) => println!("send to tg OK"),
            Err(e) => println!("send to tg err : {}", e.to_string()),
        };
    }
    if !dc_token.is_empty() && !dc_chat_id.is_empty() {
        match send_to_dc(dc_token, dc_chat_id, info.clone()).await {
            Ok(_) => println!("send to dc OK"),
            Err(e) => println!("send to dc err : {}", e.to_string()),
        };
    }
    Ok(())
}

async fn send_to_tg(tg_token: String, tg_chat_id: String, info: String) -> Result<()> {
    use teloxide::prelude::Requester;
    use teloxide::prelude::*;
    use teloxide::Bot;
    let bot = Bot::new(tg_token);
    let _ = bot
        .send_message(ChatId(tg_chat_id.parse()?), info)
        .send()
        .await?;
    Ok(())
}

async fn send_to_dc(dc_token: String, dc_chat_id: String, info: String) -> Result<()> {
    use serenity::client::ClientBuilder;
    use serenity::prelude::GatewayIntents;
    let intents = GatewayIntents::GUILDS
        | GatewayIntents::GUILD_MEMBERS
        | GatewayIntents::GUILD_BANS
        | GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::GUILD_MESSAGE_REACTIONS;
    let client = ClientBuilder::new(dc_token, intents).await?;
    let cid = dc_chat_id.parse::<u64>()?;
    serenity::model::id::ChannelId::from(cid)
        .say(client.cache_and_http.http.as_ref(), info)
        .await?;
    Ok(())
}
