use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit},
    Aes256Gcm, Key,
};
use base64::{self, Engine};
use handlebars::Handlebars;
use hmac::Hmac;
use pbkdf2::pbkdf2;
use rand::{rngs::OsRng, RngCore};
use scraper::{Html, Selector};
use serde_json::json;
use sha2::Sha256;
use std::fs;
use std::path::Path;
use std::str;
use url::Url;

fn embede_html_content(src: &str) -> String {
    let mut src_content = String::from(src);
    let document = Html::parse_document(&src_content);
    src_content = document.html();

    {
        let selector = Selector::parse(r#"img[src]"#).unwrap();
        for element in document.select(&selector) {
            if let Some(src) = element.value().attr("src") {
                if Url::parse(src).is_ok() {
                    eprintln!("Don't embbed remote image: {src}");
                    continue;
                }
                eprintln!("Embbed local image: {src}");
                let element_data = std::fs::read(src).unwrap();
                let mime_type =
                    infer::get(&element_data).map_or("image/png", |info| info.mime_type());
                let base64_img = base64::engine::general_purpose::STANDARD.encode(&element_data);
                let new_tag = format!(r#"<img src="data:{};base64,{}">"#, mime_type, base64_img);
                src_content = src_content.replacen(&element.html(), &new_tag, 1);
            }
        }
    }

    {
        let selector = Selector::parse(r#"script[src]"#).unwrap();
        for _ in document.select(&selector) {
            eprintln!("Script found. Note that scripts are not embedded.");
            break;
        }
    }

    {
        let selector = Selector::parse(r#"link[rel="stylesheet"]"#).unwrap();
        for element in document.select(&selector) {
            if let Some(href) = element.value().attr("href") {
                if Url::parse(href).is_ok() {
                    eprintln!("Don't embbed remote stylesheet: {href}");
                    continue;
                }
                eprintln!("Embbed local stylesheet: {href}");
                let element_data = std::fs::read_to_string(href).unwrap();
                let new_tag = format!("<style>{}</style>", element_data);
                src_content = src_content.replacen(&element.html(), &new_tag, 1);
            }
        }
    }

    return src_content;
}

pub fn encrypt_html(
    src: &Path,
    password: &str,
    dst: &Path,
    embed: bool,
    encrypt: bool,
    message: &str,
    title: &str,
) {
    // Load content
    let mut html_content = fs::read_to_string(src).unwrap();

    if embed {
        html_content = embede_html_content(&html_content);
    }

    if encrypt {
        let (salt, iv, crypted) = encrypt_aes_256_gcm(password, &html_content);

        let template = include_str!("template.html");
        let reg = Handlebars::new();
        html_content = reg
            .render_template(
                template,
                &json!({
                    "salt":hex::encode(&salt),
                    "iv":hex::encode(&iv),
                    "crypted":hex::encode(&crypted),
                    "message":message,
                    "title":title,
                }),
            )
            .unwrap();
    }
    fs::write(dst, html_content).unwrap();
}

fn encrypt_aes_256_gcm(password: &str, plaintext: &str) -> (Vec<u8>, Vec<u8>, Vec<u8>) {
    const PBKDF2_ITERATIONS: u32 = 100_000;
    const SALT_LEN: usize = 16;
    const KEY_LEN: usize = 32;
    const NONCE_LEN: usize = 12;
    let mut salt = [0u8; SALT_LEN];
    OsRng.fill_bytes(&mut salt);

    let mut nonce = [0u8; NONCE_LEN];
    OsRng.fill_bytes(&mut nonce);

    // Derive the key from the password
    let mut key = [0u8; KEY_LEN];
    pbkdf2::<Hmac<Sha256>>(password.as_bytes(), &salt, PBKDF2_ITERATIONS, &mut key).unwrap();

    // Create the cipher
    let key = Key::<Aes256Gcm>::from_slice(&key);
    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_bytes()).unwrap();

    (salt.to_vec(), nonce.to_vec(), ciphertext)
}

#[cfg(test)]
mod lib_test;
