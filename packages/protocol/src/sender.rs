mod friend;
pub use friend::FriendSender;
mod group;
pub use group::GroupSender;

use napi_derive::napi;
use puniyu_protocol::sender as puniyu_sender;
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

impl_enum_from_trait!(Sex, puniyu_sender::Sex, {
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

impl_enum_from_trait!(Role, puniyu_sender::Role, {
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

impl From<puniyu_sender::sender_type::SenderType> for SenderType {
    fn from(sender: puniyu_sender::sender_type::SenderType) -> Self {
        match sender {
            puniyu_sender::sender_type::SenderType::FriendSender(sender) => SenderType::Friend(sender.into()),
            puniyu_sender::sender_type::SenderType::GroupSender(sender) => SenderType::Group(sender.into()),
        }
    }
}

impl From<SenderType> for puniyu_sender::sender_type::SenderType {
    fn from(sender: SenderType) -> Self {
        match sender {
            SenderType::Friend(sender) => puniyu_sender::sender_type::SenderType::FriendSender(sender.into()),
            SenderType::Group(sender) => puniyu_sender::sender_type::SenderType::GroupSender(sender.into()),
        }
    }
}


impl From<puniyu_sender::SenderType> for SenderType {
    fn from(sender: puniyu_sender::SenderType) -> Self {
        sender.sender_type.expect("SenderType: is None").into()
    }
}

impl From<SenderType> for puniyu_sender::SenderType {
    fn from(sender: SenderType) -> Self {
        Self {
            sender_type: Some(sender.into()),
        }
    }
}