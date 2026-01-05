use super::{Role, Sex};
use napi_derive::napi;
use puniyu_protocol::sender::GroupSender as puniyu_sender;

#[napi(object)]
pub struct GroupSender {
    /// 发送者id
    pub user_id: String,
    /// 用户昵称
    pub nick: Option<String>,
    /// 性别
    pub sex: Sex,
    /// 年龄
    pub age: Option<u8>,
    /// 角色
    pub role: Role,
    /// 群名片
    pub card: Option<String>,
    /// 等级
    pub level: Option<u8>,
    /// 专属头衔
    pub title: Option<String>,
}



impl From<GroupSender> for puniyu_sender {
    fn from(sender: GroupSender) -> Self {
        let sex = puniyu_protocol::sender::Sex::from(sender.sex);
        let role = puniyu_protocol::sender::Role::from(sender.role);
        Self {
            user_id: sender.user_id,
            nick: sender.nick,
            sex: sex.into(),
            age: sender.age.map(|age| age as u32),
            role: role.into(),
            card: sender.card,
            level: sender.level.map(|level| level as u32),
            title: sender.title,
        }
    }
}

impl From<puniyu_sender> for GroupSender {
    fn from(sender: puniyu_sender) -> Self {
        let sex = puniyu_protocol::sender::Sex::try_from(sender.sex).unwrap();
        let role = puniyu_protocol::sender::Role::try_from(sender.role).unwrap();
        Self {
            user_id: sender.user_id,
            nick: sender.nick,
            sex: sex.into(),
            age: sender.age.map(|age| age as u8),
            role: role.into(),
            card: sender.card,
            level: sender.level.map(|level| level as u8),
            title: sender.title,
        }
     }
}
