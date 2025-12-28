use crate::impl_from_trait;
use napi_derive::napi;
use puniyu_types::version::Version as puniyu_version;

#[napi(object)]
pub struct Version {
    /// 主版本号
    pub major: u16,
    /// 次版本号
    pub minor: u16,
    /// 补丁版本号
    pub patch: u16,
}

impl_from_trait!(Version, puniyu_version, {
    major => major,
    minor => minor,
    patch => patch,
});
