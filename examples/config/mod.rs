#![allow(unused)]
use e_log::_log::TauriBuilder;
use e_log::{init_layer_log, Level};
use e_utils::parse::{AutoJson as _, AutoPath as _, MyParseFormat as _};
use e_utils::system::{get_original_dir, init_original_dir};
use e_utils::ui::UiConfig;
use e_utils::{Error, Result};
use hikvision::mvs_sdk::types::{MvSaveIamgeMethodValue, MvSaveIamgeType};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::env::current_dir;
use std::fs::OpenOptions;
use std::path::{Path, PathBuf};
pub mod env;
use self::env::MyEnv;
pub mod tag;

#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
pub struct CpuConf {
  // Read from environment variable first in multi-threaded mode.
  // Default to lazy auto-detection (one thread per CPU core)
  pub core: usize,
}

/// 窗口主题
#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum WindowThemeType {
  Mask,
  Line,
  Cat,
  Auto,
}
impl Default for WindowThemeType {
  fn default() -> Self {
    Self::Auto
  }
}
/// 主程序窗口配置
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
pub struct WindowConf {
  /// 窗口主题
  pub theme: WindowThemeType,
  /// 窗口UI配置
  pub ui: UiConfig,
}
/// 主程序包信息
#[derive(Deserialize, Debug, Clone, Serialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct PackageConf {
  /// 名称
  pub product_name: String,
  /// 版本
  pub version: String,
}

#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct Conf {
  pub package: PackageConf,
  pub window: WindowConf,
  pub log: LogConf,
}
#[derive(Deserialize, Debug, Clone, Default, Serialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct LogConf {
  pub level: Level,
  pub folder: String,
  pub fname: String,
  pub format: String,
  pub output_list: Vec<String>,
}

/// 全局配置
#[derive(Clone, Debug, Serialize, Deserialize, e_utils::Json)]
#[serde(rename_all = "camelCase")]
pub struct Config {
  pub conf: Conf,
  pub env: MyEnv,
}
impl Default for Config {
  fn default() -> Self {
    let home = current_dir().unwrap();
    let conf: Conf = home
      .join(ConfigType::Main.to_string())
      .auto_read_json()
      .unwrap();
    // 解析Env.json环境变量
    let myenv = MyEnv::new(home.join(ConfigType::Env.to_string())).unwrap();
    println!("{:?}", myenv);
    Self { conf, env: myenv }
  }
}
/// 配置类型
pub enum ConfigType {
  /// 默认Conf.json
  Main,
  /// 默认 Env.json
  Env,
}
impl ToString for ConfigType {
  fn to_string(&self) -> String {
    match self {
      ConfigType::Main => "Conf.json",
      ConfigType::Env => "Env.json",
    }
    .to_string()
  }
}

impl Config {
  /// 新
  pub fn new() -> Self {
    Self::default()
  }
  /// 筛选
  pub fn parse_conf<S: AsRef<str>>(&self, value: S) -> Result<String> {
    let match_fn = |k: String| -> String {
      match &*k {
        "version" => self.conf.package.version.clone(),
        "name" => self.get_product_name().unwrap_or_default(),
        _ => String::new(),
      }
    };
    value.as_ref().parse_replace('#', '#', match_fn)
  }
  /// 获取日志文件
  pub fn get_product_name(&self) -> Result<String> {
    Ok(
      self
        .parse_conf(&self.conf.package.product_name)?
        .parse_all()?
        .to_uppercase(),
    )
  }
  /// 获取日志文件
  pub fn get_window_title(&self) -> Result<String> {
    Ok(
      self
        .parse_conf(&self.conf.window.ui.title)?
        .parse_all()?
        .to_uppercase(),
    )
  }
  /// 获取日志目录
  pub fn get_log_folder(&self) -> Result<String> {
    self.parse_conf(&self.conf.log.folder)?.parse_all()
  }
  /// 获取日志文件
  pub fn get_log_fname(&self) -> Result<String> {
    self.parse_conf(&self.conf.log.fname)?.parse_all()
  }

  /// 清空日志
  pub fn clean_log(&self) -> Result<PathBuf> {
    let folder = self.get_log_folder()?;
    let fname = self.get_log_fname()?;
    let target = Path::new(&folder).join(fname);
    if Path::new(&folder).exists() {
      OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(&target)?
        .set_len(0)?;
      Ok(target)
    } else {
      Err(Error::NotFound(folder.into()))
    }
  }
  /// 写入配置
  #[allow(unused)]
  pub fn dump(&self, ctype: ConfigType) -> Result<()> {
    let home = get_original_dir()?;
    let ctype_str = ctype.to_string();
    match ctype {
      ConfigType::Main => self.conf.auto_write_json(home, ctype_str)?,
      ConfigType::Env => self.env.auto_write_json(home, ctype_str)?,
    }
    Ok(())
  }
  /// 读取并同步配置
  #[allow(unused)]
  pub fn sync(&mut self, ctype: ConfigType) -> Result<()> {
    let target = get_original_dir()?.join(ctype.to_string());
    match ctype {
      ConfigType::Main => self.conf = Conf::auto_read_json(target)?,
      ConfigType::Env => self.env = MyEnv::auto_read_json(target)?,
    }
    Ok(())
  }
  /// 读取配置
  #[allow(unused)]
  pub fn read<R>(&self, ctype: ConfigType) -> Result<R>
  where
    R: DeserializeOwned,
  {
    let home = get_original_dir()?;
    let value = home.join(ctype.to_string()).auto_read_json::<R>()?;
    Ok(value)
  }
  /// 添加日志
  #[inline]
  pub fn init_layer_log(&self) -> Result<TauriBuilder> {
    let folder = self.get_log_folder()?;
    let fname = self.get_log_fname()?;
    folder.auto_create_dir()?;
    let log = init_layer_log(
      Some(fname),
      folder,
      self.conf.log.level,
      self.conf.log.format.clone(),
      self.conf.log.output_list.clone(),
    )?;
    Ok(log)
  }
  /// 初始化环境
  pub fn init_env(&self) -> Result<()> {
    self.env.init()
  }
}
