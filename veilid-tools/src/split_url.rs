// Loose subset interpretation of the URL standard
// Not using full Url crate here for no_std compatibility
//
// Caveats:
//   No support for query string parsing
//   No support for paths with ';' parameters
//   URLs must convert to UTF8
//   Only IP address and DNS hostname host fields are supported

use super::*;

fn is_alphanum(c: u8) -> bool {
    matches!(c,
        b'A'..=b'Z'
        | b'a'..=b'z'
        | b'0'..=b'9'
    )
}
fn is_mark(c: u8) -> bool {
    matches!(
        c,
        b'-' | b'_' | b'.' | b'!' | b'~' | b'*' | b'\'' | b'(' | b')'
    )
}
fn is_unreserved(c: u8) -> bool {
    is_alphanum(c) || is_mark(c)
}

fn must_encode_userinfo(c: u8) -> bool {
    !(is_unreserved(c) || matches!(c, b'%' | b':' | b';' | b'&' | b'=' | b'+' | b'$' | b','))
}

fn must_encode_path(c: u8) -> bool {
    !(is_unreserved(c)
        || matches!(
            c,
            b'%' | b'/' | b':' | b'@' | b'&' | b'=' | b'+' | b'$' | b','
        ))
}

fn is_valid_scheme<H: AsRef<str>>(host: H) -> bool {
    let mut chars = host.as_ref().chars();
    if let Some(ch) = chars.next() {
        if !ch.is_ascii_alphabetic() {
            return false;
        }
    } else {
        return false;
    }
    for ch in chars {
        if !matches!(ch,
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '+' | '.' )
        {
            return false;
        }
    }
    true
}

fn hex_decode(h: u8) -> Result<u8, SplitUrlError> {
    match h {
        b'0'..=b'9' => Ok(h - b'0'),
        b'A'..=b'F' => Ok(h - b'A' + 10),
        b'a'..=b'f' => Ok(h - b'a' + 10),
        _ => Err(SplitUrlError::new(
            "Unexpected character in percent encoding",
        )),
    }
}

fn hex_encode(c: u8) -> (char, char) {
    let c0 = c >> 4;
    let c1 = c & 15;
    (
        if c0 < 10 {
            char::from_u32((b'0' + c0) as u32).unwrap()
        } else {
            char::from_u32((b'A' + c0 - 10) as u32).unwrap()
        },
        if c1 < 10 {
            char::from_u32((b'0' + c1) as u32).unwrap()
        } else {
            char::from_u32((b'A' + c1 - 10) as u32).unwrap()
        },
    )
}

fn url_decode<S: AsRef<str>>(s: S) -> Result<String, SplitUrlError> {
    let url = s.as_ref().to_owned();
    if !url.is_ascii() {
        return Err(SplitUrlError::new("URL is not in ASCII encoding"));
    }
    let url_bytes = url.as_bytes();
    let mut dec_bytes: Vec<u8> = Vec::with_capacity(url_bytes.len());
    let mut i = 0;
    let end = url_bytes.len();
    while i < end {
        let mut b = url_bytes[i];
        i += 1;
        if b == b'%' {
            if (i + 1) >= end {
                return Err(SplitUrlError::new("Invalid URL encoding"));
            }
            b = hex_decode(url_bytes[i])? << 4 | hex_decode(url_bytes[i + 1])?;
            i += 2;
        }
        dec_bytes.push(b);
    }
    String::from_utf8(dec_bytes)
        .map_err(|e| SplitUrlError::new(format!("Decoded URL is not valid UTF-8: {}", e)))
}

fn url_encode<S: AsRef<str>>(s: S, must_encode: impl Fn(u8) -> bool) -> String {
    let bytes = s.as_ref().as_bytes();
    let mut out = String::new();
    for b in bytes {
        if must_encode(*b) {
            let (c0, c1) = hex_encode(*b);
            out.push('%');
            out.push(c0);
            out.push(c1);
        } else {
            out.push(char::from_u32(*b as u32).unwrap())
        }
    }
    out
}

fn convert_port<N>(port_str: N) -> Result<u16, SplitUrlError>
where
    N: AsRef<str>,
{
    port_str
        .as_ref()
        .parse::<u16>()
        .map_err(|e| SplitUrlError::new(format!("Invalid port: {}", e)))
}

///////////////////////////////////////////////////////////////////////////////
#[derive(ThisError, Debug, Clone, Eq, PartialEq)]
#[error("SplitUrlError: {0}")]
pub struct SplitUrlError(String);

impl SplitUrlError {
    pub fn new<T: ToString>(message: T) -> Self {
        SplitUrlError(message.to_string())
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SplitUrlPath {
    pub path: String,
    pub fragment: Option<String>,
    pub query: Option<String>,
}

impl SplitUrlPath {
    pub fn new<P, F, Q>(path: P, fragment: Option<F>, query: Option<Q>) -> Self
    where
        P: AsRef<str>,
        F: AsRef<str>,
        Q: AsRef<str>,
    {
        Self {
            path: path.as_ref().to_owned(),
            fragment: fragment.map(|f| f.as_ref().to_owned()),
            query: query.map(|f| f.as_ref().to_owned()),
        }
    }
}

impl FromStr for SplitUrlPath {
    type Err = SplitUrlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(if let Some((p, q)) = s.split_once('?') {
            if let Some((p, f)) = p.split_once('#') {
                SplitUrlPath::new(url_decode(p)?, Some(url_decode(f)?), Some(q))
            } else {
                SplitUrlPath::new(url_decode(p)?, Option::<String>::None, Some(q))
            }
        } else if let Some((p, f)) = s.split_once('#') {
            SplitUrlPath::new(url_decode(p)?, Some(url_decode(f)?), Option::<String>::None)
        } else {
            SplitUrlPath::new(
                url_decode(s)?,
                Option::<String>::None,
                Option::<String>::None,
            )
        })
    }
}

impl fmt::Display for SplitUrlPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(fragment) = &self.fragment {
            if let Some(query) = &self.query {
                write!(
                    f,
                    "{}#{}?{}",
                    url_encode(&self.path, must_encode_path),
                    url_encode(fragment, must_encode_path),
                    query
                )
            } else {
                write!(f, "{}#{}", self.path, fragment)
            }
        } else if let Some(query) = &self.query {
            write!(f, "{}?{}", url_encode(&self.path, must_encode_path), query)
        } else {
            write!(f, "{}", url_encode(&self.path, must_encode_path))
        }
    }
}

///////////////////////////////////////////////////////////////////////////////
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SplitUrlHost {
    Hostname(String),
    IpAddr(IpAddr),
}

impl SplitUrlHost {
    pub fn new<S: AsRef<str>>(s: S) -> Result<Self, SplitUrlError> {
        Self::from_str(s.as_ref())
    }
}

impl FromStr for SplitUrlHost {
    type Err = SplitUrlError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            return Err(SplitUrlError::new("Host is empty"));
        }
        if let Ok(v4) = Ipv4Addr::from_str(s) {
            return Ok(SplitUrlHost::IpAddr(IpAddr::V4(v4)));
        }
        if &s[0..1] == "[" && &s[s.len() - 1..] == "]" {
            if let Ok(v6) = Ipv6Addr::from_str(&s[1..s.len() - 1]) {
                return Ok(SplitUrlHost::IpAddr(IpAddr::V6(v6)));
            }
            return Err(SplitUrlError::new("Invalid ipv6 address"));
        }
        for ch in s.chars() {
            if !matches!(ch,
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '.' )
            {
                return Err(SplitUrlError::new("Invalid hostname"));
            }
        }
        Ok(SplitUrlHost::Hostname(s.to_owned()))
    }
}
impl fmt::Display for SplitUrlHost {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Hostname(h) => {
                write!(f, "{}", h)
            }
            Self::IpAddr(IpAddr::V4(v4)) => {
                write!(f, "{}", v4)
            }
            Self::IpAddr(IpAddr::V6(v6)) => {
                write!(f, "[{}]", v6)
            }
        }
    }
}

///////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SplitUrl {
    pub scheme: String,
    pub userinfo: Option<String>,
    pub host: SplitUrlHost,
    pub port: Option<u16>,
    pub path: Option<SplitUrlPath>,
}

impl SplitUrl {
    pub fn new<S>(
        scheme: S,
        userinfo: Option<String>,
        host: SplitUrlHost,
        port: Option<u16>,
        path: Option<SplitUrlPath>,
    ) -> Self
    where
        S: AsRef<str>,
    {
        Self {
            scheme: scheme.as_ref().to_owned(),
            userinfo,
            host,
            port,
            path,
        }
    }

    pub fn host_port(&self, default_port: u16) -> String {
        format!("{}:{}", self.host, self.port.unwrap_or(default_port))
    }
}

fn split_host_with_port(s: &str) -> Option<(&str, &str)> {
    // special case for ipv6 colons
    if s.len() > 2 && s[0..1] == *"[" {
        if let Some(end) = s.find(']') {
            if end < (s.len() - 2) && s[end + 1..end + 2] == *":" {
                return Some((&s[0..end + 1], &s[end + 2..]));
            }
        }
        None
    } else {
        s.split_once(':')
    }
}

impl FromStr for SplitUrl {
    type Err = SplitUrlError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((scheme, mut rest)) = s.split_once("://") {
            if !is_valid_scheme(scheme) {
                return Err(SplitUrlError::new("Invalid scheme specified"));
            }
            let userinfo = {
                if let Some((userinfo_str, after)) = rest.split_once('@') {
                    rest = after;
                    Some(url_decode(userinfo_str)?)
                } else {
                    None
                }
            };
            if let Some((host, rest)) = split_host_with_port(rest) {
                let host = SplitUrlHost::from_str(host)?;
                if let Some((portstr, path)) = rest.split_once('/') {
                    let port = convert_port(portstr)?;
                    let path = SplitUrlPath::from_str(path)?;
                    Ok(SplitUrl::new(
                        scheme,
                        userinfo,
                        host,
                        Some(port),
                        Some(path),
                    ))
                } else {
                    let port = convert_port(rest)?;
                    Ok(SplitUrl::new(scheme, userinfo, host, Some(port), None))
                }
            } else if let Some((host, path)) = rest.split_once('/') {
                let host = SplitUrlHost::from_str(host)?;
                let path = SplitUrlPath::from_str(path)?;
                Ok(SplitUrl::new(scheme, userinfo, host, None, Some(path)))
            } else {
                let host = SplitUrlHost::from_str(rest)?;
                Ok(SplitUrl::new(scheme, userinfo, host, None, None))
            }
        } else {
            Err(SplitUrlError::new("No scheme specified"))
        }
    }
}

impl fmt::Display for SplitUrl {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hostname = {
            if let Some(userinfo) = &self.userinfo {
                let userinfo = url_encode(userinfo, must_encode_userinfo);
                if let Some(port) = self.port {
                    format!("{}@{}:{}", userinfo, self.host, port)
                } else {
                    format!("{}@{}", userinfo, self.host)
                }
            } else if let Some(port) = self.port {
                format!("{}:{}", self.host, port)
            } else {
                format!("{}", self.host)
            }
        };
        if let Some(path) = &self.path {
            write!(f, "{}://{}/{}", self.scheme, hostname, path)
        } else {
            write!(f, "{}://{}", self.scheme, hostname)
        }
    }
}
