use crate::account::AccountInfo;
use crate::adapter::AdapterInfo;
use napi_derive::napi;
use puniyu_protocol::bot::BotInfo as puniyu_bot;

#[napi(object)]
pub struct BotInfo {
    /// 适配器信息
    pub adapter: AdapterInfo,
    /// 账户信息
    pub account: AccountInfo,
}


impl From<BotInfo> for puniyu_bot {
    fn from(bot: BotInfo) -> Self {
        Self {
            adapter: Some(bot.adapter.into()),
            account: Some(bot.account.into()),
        }
    }
}

impl From<puniyu_bot> for BotInfo {
    fn from(bot: puniyu_bot) -> Self {
        Self {
            adapter: bot.adapter.unwrap_or_default().into(),
            account: bot.account.unwrap_or_default().into(),
        }
    }
}
