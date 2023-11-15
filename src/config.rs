use std::{
    env, fs,
    io::{self, Write},
    path::PathBuf,
};

use crate::error::{AppInitError, DynConfInitError};
use serde::{Deserialize, Serialize};

const APP_CONFIG_DIR_NAME: &str = ".guivre";

pub struct AppConfig {
    app_conf_dir_path: PathBuf,
}

impl AppConfig {
    pub fn new() -> Self {
        // Self {
            
        // }
    }
}

// アプリを管理するためのファイル群を最初に生成するための関数
// ~/.guivre
//  |   dynamic_config.toml     現在の作業ディレクトリなど、アプリケーションの実行途中に動的に変更される設定
//  |   shell_history           インタラクティブシェルのコマンド履歴ファイル
pub fn app_init() -> Result<(), AppInitError> {
    // ~/.guirveが存在するかどうか。
    let home_path = match home::home_dir() {
        Some(dir) => dir,
        None => return Err(AppInitError::HomeNotFound),
    };
    let app_conf_path = home_path.join(APP_CONFIG_DIR_NAME);
    if match is_entry_exist(&app_conf_path) {
        Ok(b) => b,
        Err(e) => {
            return Err(AppInitError::IoError(format!(
                "failed to get if {} exists: {}",
                app_conf_path.to_str().unwrap(),
                e.to_string()
            )))
        }
    } {
        // 存在しているときは、もうすでに初期化してあると判断する。
        return Err(AppInitError::InitAlreadyDone);
    }

    // 存在していないときは作成する
    let _ = match fs::create_dir_all(app_conf_path) {
        Ok(_) => (),
        Err(e) => {
            return Err(AppInitError::IoError(format!(
                "falied to create {}: {}",
                app_conf_path.to_str().unwrap(),
                e.to_string(),
            )))
        }
    };

    // dynamic_config.tomlの設定
    let _ = match init_dyn_conf(&app_conf_path) {
        Ok(_) => (),
        Err(e) => return Err(AppInitError::DynConfInitError(e.to_string())),
    };
    Ok(())
}

pub fn is_app_initialized() -> Result<bool, AppInitError> {
    let home_path = match home::home_dir() {
        Some(dir) => dir,
        None => return Err(AppInitError::HomeNotFound),
    };
    let app_conf_path = home_path.join(APP_CONFIG_DIR_NAME);
    match is_entry_exist(&app_conf_path) {
        Ok(b) => Ok(b),
        Err(e) => {
            return Err(AppInitError::IoError(format!(
                "failed to get if {} exists: {}",
                app_conf_path.to_str().unwrap(),
                e.to_string()
            )))
        }
    }
}

//
// ~/.guirve/dynamic_config.tomlの設定
//
const DYNAMIC_CONFIG_FILE_NAME: &str = "dynamic_config.toml";
#[derive(Deserialize, Serialize, Debug)]
struct DynamicConfig {
    current_workspace: String,
}

fn init_dyn_conf(app_conf_path: &PathBuf) -> Result<(), DynConfInitError> {
    // ~/.guivre/dynamic_config.tomlの作成
    let path = app_conf_path.join(DYNAMIC_CONFIG_FILE_NAME);
    let _ = match fs::File::create(&path) {
        Ok(_) => (),
        Err(e) => {
            return Err(DynConfInitError::IoError(format!(
                "failed to create {}: {}",
                path.to_str().unwrap(),
                e.to_string()
            )))
        }
    };

    // 初期のdynamic_config.tomlの書き込み
    //
    // current_workspaceには最初は何も指定しない
    // なにか現在の作業ディレクトリ(ユーザーが思っているだけ)に対してのコマンドが来ても、
    // 最初に現在の作業ディレクトリがどこなのかを指定させるようにする
    let conf = DynamicConfig {
        current_workspace: "".to_string(),
    };
    // DynamicConfigをTomlファイルに変換する
    let toml_ = match toml::to_string(&conf) {
        Ok(s) => s,
        Err(e) => return Err(DynConfInitError::TomlError(e.to_string())),
    };
    // もうこの時点では~/.guivre/dynamic_config.tomlは作成済みなので、
    // ファイルを開くときにエラーが出た場合は、即リターンする
    let file = match fs::File::create(&path) {
        Ok(f) => f,
        Err(e) => {
            return Err(DynConfInitError::IoError(format!(
                "failed to open {}: {}",
                path.to_str().unwrap(),
                e.to_string()
            )))
        }
    };
    let mut writer = io::BufWriter::new(file);
    let _ = match writer.write_all(toml_.as_bytes()) {
        Ok(_) => (),
        Err(e) => {
            return Err(DynConfInitError::IoError(format!(
                "failed to write config to {}: {}",
                path.to_str().unwrap(),
                e.to_string()
            )))
        }
    };

    Ok(())
}

//
// コマンド履歴ファイルの設定
// ~/.guivre/shell_history
const SHELL_HISTORY_FILE_NAME: &str = "shell_history";

/* pub fn get_shell_history_path() -> PathBuf {

} */

//
//
//
//
pub fn is_entry_exist(path: &PathBuf) -> Result<bool, io::Error> {
    match fs::metadata(path) {
        Ok(_) => Ok(true),
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(false)
            } else {
                Err(e)
            }
        }
    }
}
