mod friend;

use napi_derive::napi;
pub use friend::FriendSender;
mod group;
pub use group::GroupSender;

use puniyu_types::sender::Role as puniyu_role;
use puniyu_types::sender::Sex as puniyu_sex;
use puniyu_protocol::sender as puniyu_protocol_sender;
use crate::impl_enum_from_trait;


#[derive(Debug, Clone)]
#[napi]
/// 性别
pub enum Sex {
    /// 男性
    Male,
    /// 女性
    Female,
    /// 未知
    Unknown,
}
impl_enum_from_trait!(Sex, puniyu_sex, {
    Male => Male,
    Female => Female,
    Unknown => Unknown,
});
impl_enum_from_trait!(Sex, puniyu_protocol_sender::Sex, {
    Male => Male,
    Female => Female,
    Unknown => Unknown,
});





#[derive(Debug, Clone)]
#[napi]
/// 事件发送者角色
pub enum Role {
    /// 群主
    Owner,
    /// 管理员
    Admin,
    /// 成员
    Member,
    /// 未知
    Unknown,
}

impl_enum_from_trait!(Role, puniyu_role, {
    Owner => Owner,
    Admin => Admin,
    Member => Member,
    Unknown => Unknown
});
impl_enum_from_trait!(Role, puniyu_protocol_sender::Role, {
    Owner => Owner,
    Admin => Admin,
    Member => Member,
    Unknown => Unknown
});

#[napi]
pub enum SenderType {
    Friend(FriendSender),
    Group(GroupSender),
}

