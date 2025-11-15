use napi_derive::napi;
use puniyu_core::AppBuilder;
use std::sync::{LazyLock, Mutex, RwLock};

use napi::tokio::runtime::Runtime;

static APP: LazyLock<RwLock<AppBuilder>> = LazyLock::new(|| RwLock::new(AppBuilder::new()));

pub(crate) static RT_RUNTIME: LazyLock<Mutex<Runtime>> =
    LazyLock::new(|| Mutex::new(Runtime::new().unwrap()));

#[napi(constructor)]
pub struct Bot;

#[napi]
impl Bot {
    #[napi]
    /// 设置当前应用名称
    pub fn set_name(&self, name: String) {
        let mut client = APP.write().unwrap();
        client.with_name(name.as_str());
    }
    #[napi]
    /// 设置工作目录
    pub fn set_working_dir(&self, dir: String) {
        let mut client = APP.write().unwrap();
        client.with_working_dir(dir.as_ref());
    }
    #[napi]
    /// 运行应用
    pub fn run(&self) {
        let mut client = APP.write().unwrap();
        let rt = RT_RUNTIME.lock().unwrap();
        rt.block_on(async {
            client.with_adapter(&puniyu_adapter_console::Adapter).build().run().await;
        })
    }
}
