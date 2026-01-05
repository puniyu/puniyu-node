use napi_derive::napi;
use super::Sex;
use puniyu_protocol::sender::FriendSender as puniyu_sender;

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
    pub age: Option<u8>,
}



impl From<FriendSender> for puniyu_sender {
    fn from(sender: FriendSender) -> Self {
        let sex = puniyu_protocol::sender::Sex::try_from(sender.sex).unwrap();
        Self {
            user_id: sender.user_id,
            nick: sender.nick,
            sex: sex.into(),
            age: sender.age.map(|age| age as u32),
        }
    }
}

impl From<puniyu_sender> for FriendSender {
    fn from(sender: puniyu_sender) -> Self {
        let sex = puniyu_protocol::sender::Sex::try_from(sender.sex).unwrap();
        Self {
            user_id: sender.user_id,
            nick: sender.nick,
            sex: sex.into(),
            age: sender.age.map(|age| age as u8),
        }
    }
}
