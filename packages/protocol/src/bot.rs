use crate::account::AccountInfo;
use crate::adapter::AdapterInfo;
use crate::impl_from_trait;
use napi_derive::napi;
use puniyu_protocol::bot::BotInfo as puniyu_protocol_bot;
use puniyu_types::bot::BotInfo as puniyu_bot;

#[napi(object)]
pub struct BotInfo {
    /// 适配器信息
    pub adapter: AdapterInfo,
    /// 账户信息
    pub account: AccountInfo,
}

impl_from_trait!(BotInfo, puniyu_bot, {
    adapter => adapter,
    account => account,
});

impl From<BotInfo> for puniyu_protocol_bot {
    fn from(bot: BotInfo) -> Self {
        Self {
            adapter: Some(bot.adapter.into()),
            account: Some(bot.account.into()),
        }
    }
}

impl From<puniyu_protocol_bot> for BotInfo {
    fn from(bot: puniyu_protocol_bot) -> Self {
        Self {
            adapter: bot.adapter.unwrap().into(),
            account: bot.account.unwrap().into(),
        }
    }
}
