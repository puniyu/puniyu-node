use napi_derive::napi;
use crate::impl_from_trait;
use puniyu_types::account::AccountInfo as puniyu_account;
use puniyu_protocol::account::AccountInfo as puniyu_protocol_account;

/// 账户信息
#[napi(object)]
pub struct AccountInfo {
    /// Bot账号的uin
    pub uin: String,
    /// Bot账号的昵称
    pub name: String,
    /// Bot账号的头像URL地址
    pub avatar: String,
}

impl_from_trait!(AccountInfo, puniyu_account, {
    uin => uin,
    name => name,
    avatar => avatar,
});
impl_from_trait!(AccountInfo, puniyu_protocol_account, {
    uin => uin,
    name => name,
    avatar => avatar,
});


