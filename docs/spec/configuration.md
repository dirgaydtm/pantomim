# Spec: `configuration` module

## Interface

```rust
pub fn load(data: &[u8], password: Option<&str>) -> Result<plist::Dictionary, Error>;
pub fn config_key(settings: &plist::Dictionary) -> String;
pub fn config_key_hash(config_key: &str, url: &str) -> String;
pub fn to_seb_json(settings: &plist::Dictionary) -> String;

#[derive(Debug, PartialEq)]
pub enum Error { PasswordNeeded, WrongPassword, NotSupported(String), InvalidData(String) }
// + impl Display, impl std::error::Error
```

Only the items above are public. Every hash is lowercase hex SHA-256.

## `load`

1. Data longer than 18 bytes that starts with `1F 8B 08` is gunzipped (failure: `InvalidData`). Output over 16 MiB is rejected with `InvalidData` (gzip bomb guard; SEB files are a few KB).
2. Data longer than 4 bytes that starts with `<?xm` (case-insensitive) is parsed as an XML plist.
3. Anything left that is 4 bytes or shorter: `InvalidData`.
4. The first 4 bytes are the prefix and the rest is the block:
   - `plnd`: the block is the payload.
   - `pswd`: decrypt the block with the password. No password: `PasswordNeeded`.
   - `pwcc` (client configuration), `pkhs`, `phsk` (certificate): `NotSupported`.
   - anything else: `InvalidData`.
5. The payload is gunzipped if compressed and must then be XML (rule 2). Otherwise: `InvalidData`.
6. A broken plist or a root that is not `<dict>`: `InvalidData`.

### Block decryption (RNCryptor)

Layout: `[version 1][options 1][encryption salt 8][HMAC salt 8][IV 16][ciphertext][HMAC 32]`.

- The version and options bytes are ignored. SEB writes version 2, which has the same layout as v3.
- A block shorter than 66 bytes: `InvalidData`.
- Keys: PBKDF2-HMAC-SHA1, 10,000 iterations, 32 bytes, from the UTF-8 bytes of the password, one per salt.
- HMAC-SHA256 over every byte except the last 32. Mismatch: `WrongPassword`.
- AES-256-CBC with PKCS#7 padding. Bad padding: `InvalidData`.

## `to_seb_json`

The rules on the [SEB Config Key page](https://safeexambrowser.org/developer/seb-config-key.html) apply, plus these:

- No whitespace and no escaping. Strings are written as they are, `"` and `\` included.
- Keys in every dict are sorted recursively like .NET `StringComparer.InvariantCulture`. For ASCII, compare case-insensitively with symbols < digits < letters (`a10` < `a2`); on a tie, lowercase comes first (`b` < `B`, `allowWlan` < `allowWLAN`).
- Dicts drop the `originatorVersion` key (case-insensitive, at any level) and any entry whose value is an empty dict. An empty dict inside an array is still written as `{}`.
- A comma follows every entry except the one that was last before dropping. If the last entry is dropped, the entry before it keeps its comma: `{"a":false,}`. Arrays do not do this.

Values:

| Type | Written as |
|---|---|
| bool | `true` / `false` |
| integer | decimal |
| real | .NET "G" with precision 15 (see below) |
| date (UTC) | `YYYY-MM-DDTHH:MM:SS.0000000Z`, unquoted |
| data | standard base64 with padding, quoted |
| string | quoted, as is |
| array / dict | `[a,b]` / `{"k":v}` |
| other types | `""` |

Reals are rounded to 15 significant digits with trailing zeros removed. Plain notation is used when the scientific exponent is > -5 and < 15. Otherwise the number uses `E`, a sign, and at least 2 exponent digits:

`2.0`→`2`, `0.1`→`0.1`, `0.10000000000000001`→`0.1`, `-1.5`→`-1.5`, `0.0001`→`0.0001`, `0.00001`→`1E-05`, `0.000001`→`1E-06`, `123456789012345.0`→`123456789012345`, `1e15`→`1E+15`, `1.5e300`→`1.5E+300`, `0.0`→`0`, NaN→`NaN`, +∞→`Infinity`, −∞→`-Infinity`.

## `config_key` and `config_key_hash`

- `config_key(settings)`: SHA-256 of the UTF-8 bytes of `to_seb_json(settings)`.
- `config_key_hash(key, url)`: SHA-256 of the URL without its fragment, followed by the key. The fragment starts at the first `#`.

## Test vectors

1. `tests/fixtures/config.seb` gives `config_key` = `866101fbdbcce2b3e9df1b4ca0a25e19e034f8b271f51b26bd884f162dd97d31` (accepted by Moodle).
2. `config_key_hash("key", "https://a.b/c")` = `723aa5a712872900388543270c826d6d8a3e094524d0dc411c341643c5a2caa0` (`printf 'https://a.b/ckey' | sha256sum`). `"https://a.b/c#frag"` gives the same value.
3. This XML:
   ```xml
   <?xml version="1.0"?><plist version="1.0"><dict>
     <key>b</key><string>say "hi"</string>
     <key>B</key><integer>1</integer>
     <key>a2</key><real>0.1</real>
     <key>a10</key><real>2</real>
     <key>date</key><date>2024-05-01T10:20:30Z</date>
     <key>empty</key><dict/>
     <key>list</key><array><dict/><data>AQI=</data><string/></array>
     <key>zz</key><dict><key>a</key><false/><key>originatorVersion</key><string>x</string></dict>
   </dict></plist>
   ```
   produces exactly this `to_seb_json`:
   `{"a10":2,"a2":0.1,"b":"say "hi"","B":1,"date":2024-05-01T10:20:30.0000000Z,"list":[{},"AQI=",""],"zz":{"a":false,}}`
4. The same XML gives the same `config_key` through gzip(XML), gzip(`plnd` + gzip(XML)), and gzip(`pswd` + RNCryptor(gzip(XML), "secret")) with password `"secret"`.
5. `pswd` without a password: `PasswordNeeded`. Wrong password: `WrongPassword`. `pwcc` or `pkhs` followed by any 10 bytes: `NotSupported`. A valid plist padded to 17 MB and gzipped: `InvalidData`.
6. Real-number examples at the edges: `2.0`, `0.10000000000000001`, `0.0001`, `0.00001`, `123456789012345.0`, `1e15`.
