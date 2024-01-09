use aes::Aes256;
use block_modes::block_padding::Pkcs7;
use block_modes::{BlockMode, Ecb};

type AesEcb = Ecb<Aes256, Pkcs7>;

pub(crate) fn decrypt_jm(data: &str, key: &[u8]) -> anyhow::Result<String> {
    let data = base64::decode(data)?;
    let data = aes_decrypt_ecb(data, key)?;
    let data = std::str::from_utf8(data.as_slice())?;
    Ok(data.to_string())
}

fn aes_decrypt_ecb(data: Vec<u8>, key: &[u8]) -> anyhow::Result<Vec<u8>> {
    Ok(AesEcb::new_from_slices(key, "".as_bytes())?.decrypt_vec(data.as_slice())?)
}

/// FROM STRING 并打印出错的位置
pub fn from_str<T: for<'de> serde::Deserialize<'de>>(json: &str) -> anyhow::Result<T> {
    Ok(serde_path_to_error::deserialize(
        &mut serde_json::Deserializer::from_str(json),
    )?)
}
