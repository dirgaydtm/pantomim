use std::io::Read;

use flate2::read::GzDecoder;

use super::Error;
use super::rncryptor;

pub fn load(data: &[u8], password: Option<&str>) -> Result<plist::Dictionary, Error> {
    let data = gunzip_if_compressed(data)?;
    if is_xml(&data) {
        return parse_xml(&data);
    }
    if data.len() <= 4 {
        return Err(Error::InvalidData("file too short".into()));
    }

    let (prefix, block) = data.split_at(4);
    let payload = match prefix {
        b"plnd" => block.to_vec(),
        b"pswd" => rncryptor::decrypt(block, password.ok_or(Error::PasswordNeeded)?)?,
        // Client-configuration and certificate-encrypted files never start an exam.
        b"pwcc" | b"pkhs" | b"phsk" => {
            return Err(Error::NotSupported(format!(
                "{} configuration",
                String::from_utf8_lossy(prefix)
            )));
        }
        _ => return Err(Error::InvalidData("unknown prefix".into())),
    };

    let payload = gunzip_if_compressed(&payload)?;
    if !is_xml(&payload) {
        return Err(Error::InvalidData("payload is not an XML plist".into()));
    }
    parse_xml(&payload)
}

/// Guards against gzip bombs from downloaded files; real SEB files are a few KB.
const MAX_UNCOMPRESSED: u64 = 16 * 1024 * 1024;

fn gunzip_if_compressed(data: &[u8]) -> Result<Vec<u8>, Error> {
    if data.len() <= 18 || !data.starts_with(&[0x1f, 0x8b, 0x08]) {
        return Ok(data.to_vec());
    }
    let mut out = Vec::new();
    GzDecoder::new(data)
        .take(MAX_UNCOMPRESSED + 1)
        .read_to_end(&mut out)
        .map_err(|e| Error::InvalidData(format!("gzip: {e}")))?;
    if out.len() as u64 > MAX_UNCOMPRESSED {
        return Err(Error::InvalidData("decompressed data too large".into()));
    }
    Ok(out)
}

fn is_xml(data: &[u8]) -> bool {
    data.len() > 4 && data[..4].eq_ignore_ascii_case(b"<?xm")
}

fn parse_xml(data: &[u8]) -> Result<plist::Dictionary, Error> {
    plist::Value::from_reader_xml(data)
        .map_err(|e| Error::InvalidData(format!("plist: {e}")))?
        .into_dictionary()
        .ok_or_else(|| Error::InvalidData("plist root is not a dict".into()))
}
