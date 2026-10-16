use std::io::Write;

use aes::cipher::{BlockModeEncrypt, KeyIvInit, block_padding::Pkcs7};
use flate2::{Compression, write::GzEncoder};
use hmac::{KeyInit, Mac};
use pantomim_lib::configuration::{Error, config_key, config_key_hash, load, to_seb_json};
use sha2::Sha256;

const SAMPLE_XML: &str = r#"<?xml version="1.0"?><plist version="1.0"><dict>
  <key>b</key><string>say "hi"</string>
  <key>B</key><integer>1</integer>
  <key>a2</key><real>0.1</real>
  <key>a10</key><real>2</real>
  <key>date</key><date>2024-05-01T10:20:30Z</date>
  <key>empty</key><dict/>
  <key>list</key><array><dict/><data>AQI=</data><string/></array>
  <key>zz</key><dict><key>a</key><false/><key>originatorVersion</key><string>x</string></dict>
</dict></plist>"#;

const SAMPLE_JSON: &str = r#"{"a10":2,"a2":0.1,"b":"say "hi"","B":1,"date":2024-05-01T10:20:30.0000000Z,"list":[{},"AQI=",""],"zz":{"a":false,}}"#;

fn gzip(data: &[u8]) -> Vec<u8> {
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}

/// RNCryptor v3 password-based encryption with fixed salts and IV.
fn rncryptor_encrypt(password: &str, plaintext: &[u8]) -> Vec<u8> {
    let encryption_salt = [1u8; 8];
    let hmac_salt = [2u8; 8];
    let iv = [3u8; 16];
    let derive = |salt: &[u8]| {
        pbkdf2::pbkdf2_hmac_array::<sha1::Sha1, 32>(password.as_bytes(), salt, 10_000)
    };

    let mut message = vec![3, 1];
    message.extend_from_slice(&encryption_salt);
    message.extend_from_slice(&hmac_salt);
    message.extend_from_slice(&iv);
    let ciphertext = cbc::Encryptor::<aes::Aes256>::new_from_slices(&derive(&encryption_salt), &iv)
        .unwrap()
        .encrypt_padded_vec::<Pkcs7>(plaintext);
    message.extend_from_slice(&ciphertext);

    let mut mac = hmac::Hmac::<Sha256>::new_from_slice(&derive(&hmac_salt)).unwrap();
    mac.update(&message);
    message.extend_from_slice(&mac.finalize().into_bytes());
    message
}

fn sample_key() -> String {
    config_key(&load(SAMPLE_XML.as_bytes(), None).unwrap())
}

#[test]
fn fixture_config_key_matches_moodle() {
    let data = std::fs::read(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/fixtures/config.seb"
    ))
    .unwrap();
    assert_eq!(
        config_key(&load(&data, None).unwrap()),
        "866101fbdbcce2b3e9df1b4ca0a25e19e034f8b271f51b26bd884f162dd97d31"
    );
}

#[test]
fn config_key_hash_is_sha256_of_url_without_fragment_then_key() {
    // printf 'https://a.b/ckey' | sha256sum
    let expected = "723aa5a712872900388543270c826d6d8a3e094524d0dc411c341643c5a2caa0";
    assert_eq!(config_key_hash("key", "https://a.b/c"), expected);
    assert_eq!(config_key_hash("key", "https://a.b/c#frag"), expected);
}

#[test]
fn seb_json_reproduces_quirks() {
    let settings = load(SAMPLE_XML.as_bytes(), None).unwrap();
    assert_eq!(to_seb_json(&settings), SAMPLE_JSON);
}

#[test]
fn loads_every_container() {
    let xml = gzip(SAMPLE_XML.as_bytes());
    let pswd = gzip(&[b"pswd".as_slice(), &rncryptor_encrypt("secret", &xml)].concat());
    let plnd = gzip(&[b"plnd".as_slice(), &xml].concat());

    for (data, password) in [(&xml, None), (&plnd, None), (&pswd, Some("secret"))] {
        assert_eq!(config_key(&load(data, password).unwrap()), sample_key());
    }
}

#[test]
fn rejects_bad_input() {
    let pswd = gzip(
        &[
            b"pswd".as_slice(),
            &rncryptor_encrypt("secret", &gzip(SAMPLE_XML.as_bytes())),
        ]
        .concat(),
    );
    assert_eq!(load(&pswd, None), Err(Error::PasswordNeeded));
    assert_eq!(load(&pswd, Some("nope")), Err(Error::WrongPassword));
    for prefix in ["pwcc", "pkhs"] {
        let data = format!("{prefix}0123456789");
        assert!(matches!(
            load(data.as_bytes(), None),
            Err(Error::NotSupported(_))
        ));
    }

    // Gzip bomb: a valid plist padded to 17 MB compresses to ~17 KB.
    let padding = " ".repeat(17 * 1024 * 1024);
    let xml =
        format!(r#"<?xml version="1.0"?><plist version="1.0"><dict>{padding}</dict></plist>"#);
    assert!(matches!(
        load(&gzip(xml.as_bytes()), None),
        Err(Error::InvalidData(_))
    ));
}

#[test]
fn real_number_formatting() {
    let cases = [
        ("2.0", "2"),
        ("0.10000000000000001", "0.1"),
        ("0.0001", "0.0001"),
        ("0.00001", "1E-05"),
        ("123456789012345.0", "123456789012345"),
        ("1e15", "1E+15"),
    ];
    for (input, expected) in cases {
        let xml = format!(
            r#"<?xml version="1.0"?><plist version="1.0"><dict><key>r</key><real>{input}</real></dict></plist>"#
        );
        let settings = load(xml.as_bytes(), None).unwrap();
        assert_eq!(
            to_seb_json(&settings),
            format!(r#"{{"r":{expected}}}"#),
            "input {input}"
        );
    }
}
