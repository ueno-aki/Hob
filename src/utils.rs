use anyhow::{Context, Result};
use base64::{
    prelude::{BASE64_STANDARD, BASE64_URL_SAFE_NO_PAD},
    Engine,
};
use std::fs;
use yaml_rust::YamlLoader;

pub fn get_option(option_name: &str) -> Result<String> {
    let file = fs::read_to_string("./server.properties")
        .context("Not Found 'server.properties'")
        .unwrap();
    let options = &YamlLoader::load_from_str(&file)?[0];
    Ok(options[option_name]
        .as_str()
        .context(format!("Failed to get option:{}", option_name))?
        .to_owned())
}

#[inline]
pub fn decode_base64<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    let decoded = BASE64_STANDARD.decode(input)?;
    Ok(decoded)
}
#[inline]
pub fn encode_base64<T: AsRef<[u8]>>(input: T) -> String {
    BASE64_STANDARD.encode(input)
}
#[inline]
pub fn decode_nopad_base64<T: AsRef<[u8]>>(input: T) -> Result<Vec<u8>> {
    let decoded = BASE64_URL_SAFE_NO_PAD.decode(input)?;
    Ok(decoded)
}
#[inline]
pub fn encode_nopad_base64<T: AsRef<[u8]>>(input: T) -> String {
    BASE64_URL_SAFE_NO_PAD.encode(input)
}
