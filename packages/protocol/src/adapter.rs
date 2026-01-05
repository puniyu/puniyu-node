use crate::impl_enum_from_trait;
use crate::version::Version;
use chrono::{DateTime, Utc};
use napi_derive::napi;
use puniyu_protocol::adapter as puniyu_adapter;

/// 适配器平台
///
/// 用于标识适配器的平台，用于在不同平台之间进行消息传递。
/// - QQ：QQ 平台
/// - Wechat： 微信平台
/// - Telegram: Telegram 平台
/// - Discord: Discord 平台
/// - Kook: 开黑吧 平台
/// - Other: 其他平台
#[napi]
pub enum AdapterPlatform {
    QQ,
    Wechat,
    Telegram,
    Discord,
    Kook,
    Other,
}

impl_enum_from_trait!(AdapterPlatform, puniyu_adapter::AdapterPlatform, {
    QQ => Qq,
    Wechat => WeChat,
    Telegram => Telegram,
    Discord => Discord,
    Kook => Kook,
    Other => PlatformOther,
});

/// 适配器所使用的标准接口协议
///
/// - OneBotV11: onebot v11 标准
/// - OneBotV12: onebot v12 标准
/// - OICQ: OICQ 标准
/// - ICQQ: OICQ fork 标准
/// - Other: 其他标准
#[napi]
pub enum AdapterStandard {
    OneBotV11,
    OneBotV12,
    Oicq,
    Icqq,
    Other,
}

impl_enum_from_trait!(AdapterStandard, puniyu_adapter::AdapterStandard, {
    OneBotV11 => OneBotV11,
    OneBotV12 => OneBotV12,
    Oicq => Oicq,
    Icqq => Icqq,
    Other => StandardOther,
});

/// 适配器协议实现
///
/// 用于标识适配器所使用的协议实现，用于在不同平台之间进行消息传递。
///
/// - QQBOT: [QQ 平台协议实现](https://bot.q.qq.com/wiki)
/// - ICQQ: [Icqq 平台协议实现](https://github.com/icqqjs/icqq)
/// - GoCqHttp: [go-cqhttp 协议实现](https://docs.go-cqhttp.org/)
/// - NapCat: [NapCat 协议实现](https://napneko.github.io/zh-CN/)
/// - Oicq: [Oicq 平台协议实现](https://github.com/takayama-lily/oicq)
/// - LLOneBot: [LLOneBot 协议实现](https://llonebot.github.io/zh-CN/)
/// - Lagrange: [Lagrange 协议实现](ttps://lagrangedev.github.io/Lagrange.Doc/Lagrange.OneBot/)
/// - Console: 控制台协议实现
/// - Other: 其他协议实现
#[napi]
pub enum AdapterProtocol {
    QQBot,
    Oicq,
    Icqq,
    GoCqHttp,
    NapCat,
    LLOneBot,
    Conwechat,
    Lagrange,
    Console,
    Other,
}

impl_enum_from_trait!(AdapterProtocol, puniyu_adapter::AdapterProtocol, {
    QQBot => QqBot,
    Oicq => Oicq,
    Icqq => Icqq,
    GoCqHttp => GoCqHttp,
    NapCat => NapCat,
    LLOneBot => LlOneBot,
    Conwechat => Conwechat,
    Lagrange => Lagrange,
    Console => Console,
    Other => Other,
});

/// 适配器通信方式
///
/// 用于标识适配器所使用的通信方式，用于在不同平台之间进行消息传递。
///
/// - Http: Http 通信方式
/// - WebSocketServer: WebSocket 服务器通信方式
/// - WebSocketClient: WebSocket 客户端通信方式
/// - Grpc: Grpc 通信方式
/// - Other: 其他通信方式
#[napi]
pub enum AdapterCommunication {
    Http,
    WebSocketServer,
    WebSocketClient,
    Grpc,
    Other,
}

impl_enum_from_trait!(AdapterCommunication, puniyu_adapter::AdapterCommunication, {
    Http => Http,
    WebSocketServer => WebSocketServer,
    WebSocketClient => WebSocketClient,
    Grpc => Grpc,
    Other => Other,
});

/// 适配器信息
#[napi(object)]
pub struct AdapterInfo {
    /// 适配器名称 如lagrange-onebot
    pub name: String,
    /// 适配器版本
    pub version: Version,
    /// 适配器平台
    pub platform: AdapterPlatform,
    /// 适配器使用的协议标准 如onebot11
    pub standard: AdapterStandard,
    /// 适配器协议实现 如gocq、napcat
    pub protocol: AdapterProtocol,
    /// 适配器通信方式
    pub communication: AdapterCommunication,
    /// 适配器通信地址
    ///
    /// # 示例
    /// `127.0.0.1:7000`
    /// `127.0.0.1:7000/ws`
    /// `127.0.0.1:7001`
    pub address: Option<String>,
    /// 连接时间
    pub connect_time: DateTime<Utc>,
    /// 鉴权密钥
    pub secret: Option<String>
}

impl From<puniyu_adapter::AdapterInfo> for AdapterInfo {
    fn from(adapter: puniyu_adapter::AdapterInfo) -> Self {
        let platform = puniyu_adapter::AdapterPlatform::try_from(adapter.platform).unwrap_or_default();
        let standard = puniyu_adapter::AdapterStandard::try_from(adapter.standard).unwrap_or_default();
        let protocol = puniyu_adapter::AdapterProtocol::try_from(adapter.protocol).unwrap_or_default();
        let communication = puniyu_adapter::AdapterCommunication::try_from(adapter.communication).unwrap_or_default();
        Self {
            name: adapter.name,
            version: adapter.version.into(),
            platform: platform.into(),
            standard: standard.into(),
            protocol: protocol.into(),
            communication: communication.into(),
            address: adapter.address,
            connect_time: DateTime::from_timestamp_secs(adapter.connect_time as i64).unwrap(),
            secret: adapter.secret,
        }
    }
}

impl From<AdapterInfo> for puniyu_adapter::AdapterInfo {
    fn from(adapter: AdapterInfo) -> Self {
        let platform = puniyu_adapter::AdapterPlatform::from(adapter.platform);
        let standard = puniyu_adapter::AdapterStandard::from(adapter.standard);
        let protocol = puniyu_adapter::AdapterProtocol::from(adapter.protocol);
        let communication = puniyu_adapter::AdapterCommunication::from(adapter.communication);
        Self {
            name: adapter.name,
            version: adapter.version.into(),
            platform: platform.into(),
            standard: standard.into(),
            protocol: protocol.into(),
            communication: communication.into(),
            address: adapter.address,
            connect_time: adapter.connect_time.timestamp() as u64,
            secret: adapter.secret,
        }
    }
}
