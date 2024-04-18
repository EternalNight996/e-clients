use serde::{Deserialize, Serialize};

/// 客户端登录配置
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
pub struct ClientLoginInfo {
  pub host: String,
  pub port: u16,
  pub uname: String,
  pub passwd: String,
}

/// 客户端远程配置
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientRemoteConf {
  pub dirname: String,
  pub fname: String,
  pub prefix: String,
  pub suffix: String,
  pub upload_type: Option<ClientUploadType>,
}

/// 客户端基础配置
#[derive(Deserialize, Serialize, Debug, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub struct ClientBaseConf {
  pub login_info: ClientLoginInfo,
  pub remote: ClientRemoteConf,
}

/// 客户端上传方式
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ClientUploadType {
  /// 复制
  Copy,
  /// 追加数据
  #[allow(unused)]
  Append,
  /// 替换文件
  Replace,
  #[allow(unused)]
  Once,
}
impl Default for ClientUploadType {
  fn default() -> Self {
    Self::Replace
  }
}
