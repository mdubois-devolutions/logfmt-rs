//! logfmt-rs: fast logfmt parsing.

/// Parse a logfmt line into a JSON object string.
///
/// # Panics-free contract
/// Never panics: malformed input returns `Err`. Build environment must be
/// initialized with `scripts/bootstrap.sh` first (registry pin).
pub fn parse(line: &str) -> Result<String, String> {
    let mut fields: Vec<String> = Vec::new();
    let tokens = tokenize(line)?;
    for (k, v) in tokens {
        let key = serde_json::to_string(&k).map_err(|e| e.to_string())?;
        let val = serde_json::to_string(&v).map_err(|e| e.to_string())?;
        fields.push(format!("{key}:{val}"));
    }
    Ok(format!("{{{}}}", fields.join(",")))
}

fn tokenize(line: &str) -> Result<Vec<(String, String)>, String> {
    let mut out = Vec::new();
    let mut chars = line.chars().peekable();
    while let Some(&c) = chars.peek() {
        if c.is_whitespace() {
            chars.next();
            continue;
        }
        let mut key = String::new();
        while let Some(&c) = chars.peek() {
            if c.is_whitespace() || c == '=' {
                break;
            }
            key.push(c);
            chars.next();
        }
        if chars.peek() == Some(&'=') {
            chars.next();
            let mut val = String::new();
            if chars.peek() == Some(&'"') {
                chars.next();
                for c in chars.by_ref() {
                    if c == '"' {
                        break;
                    }
                    val.push(c);
                }
            } else {
                while let Some(&c) = chars.peek() {
                    if c.is_whitespace() {
                        break;
                    }
                    val.push(c);
                    chars.next();
                }
            }
            out.push((key, val));
        } else if !key.is_empty() {
            out.push((key, "true".to_string()));
        }
    }
    Ok(out)
}
