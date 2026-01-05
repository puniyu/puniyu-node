use crate::impl_from_trait;
use napi_derive::napi;
use puniyu_types::version::Version as puniyu_version;

#[napi(object)]
#[derive(Default)]
pub struct Version {
    /// 主版本号
    pub major: u16,
    /// 次版本号
    pub minor: u16,
    /// 补丁版本号
    pub patch: u16,
}


impl From<Version> for String {
    fn from(v: Version) -> Self {
        format!("{}.{}.{}", v.major, v.minor, v.patch)
    }
}

impl From<String> for Version {
    fn from(v: String) -> Self {
        let mut parts = v.split('.');
        Self {
            major: parts.next().unwrap_or("0").parse().unwrap_or(0),
            minor: parts.next().unwrap_or("0").parse().unwrap_or(0),
            patch: parts.next().unwrap_or("0").parse().unwrap_or(0),
        }
    }
}

impl_from_trait!(Version, puniyu_version, {
    major => major,
    minor => minor,
    patch => patch,
});
