use napi_derive::napi;
use super::Sex;
use puniyu_types::sender::FriendSender as puniyu_sender;
use puniyu_protocol::sender::FriendSender as puniyu_protocol_sender;
use crate::impl_from_trait;

#[derive(Debug, Clone)]
#[napi(object)]
pub struct FriendSender {
    /// 发送者id
    pub user_id: String,
    /// 用户昵称
    pub nick: Option<String>,
    /// 性别
    pub sex: Sex,
    /// 年龄
    pub age: u8,
}

impl_from_trait!(FriendSender, puniyu_sender, {
    user_id => user_id,
    nick => nick,
    sex => sex,
    age => age,
});

impl From<FriendSender> for puniyu_protocol_sender {
    fn from(sender: FriendSender) -> Self {
        let sex = puniyu_protocol::sender::Sex::try_from(sender.sex).unwrap();
        Self {
            user_id: sender.user_id,
            nick: sender.nick,
            sex: sex.into(),
            age: sender.age as u32,
        }
    }
}

impl From<puniyu_protocol_sender> for FriendSender {
    fn from(sender: puniyu_protocol_sender) -> Self {
        let sex = puniyu_protocol::sender::Sex::try_from(sender.sex).unwrap();
        Self {
            user_id: sender.user_id,
            nick: sender.nick,
            sex: sex.into(),
            age: sender.age as u8,
        }
    }
}
