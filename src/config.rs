use std::{
    env, fs,
    io::{self, Read, Write},
    path::PathBuf,
};

use crate::error::{
    AppConfigError, AppInitError, DynConfInitError, ReadDynConfError, ShellHistInitError,
};
use serde::{Deserialize, Serialize};

const APP_CONFIG_DIR_NAME: &str = ".guivre";
const DYNAMIC_CONFIG_FILE_NAME: &str = "dynamic_config.toml";
const SHELL_HISTORY_FILE_NAME: &str = "shell_history";

pub struct AppConfig {
    pub app_conf_path: PathBuf,
    pub dyn_conf_path: PathBuf,
    pub shell_hist_path: PathBuf,
    pub dyn_conf: DynamicConfig,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DynamicConfig {
    pub current_workspace: String,
}

impl AppConfig {
    // ~/.guivre
    //  |   dynamic_config.toml     現在の作業ディレクトリなど、アプリケーションの実行途中に動的に変更される設定
    //  |                           初期化では初期設定が書き込まれる。アプリケーションが起動したタイミングでAppConfigに読みこまれる
    //  |                           起動時の読み込みに失敗すると、シェルは起動せずに終了する。
    //  |                           このとき、ユーザーはこのファイルを編集して、起動可能な状態にするか、再度初期化をする。
    //  |   shell_history           インタラクティブシェルのコマンド履歴ファイル。初期状態では空で、あとからshellによって使用される
    pub fn new() -> Result<Self, AppConfigError> {
        let home_path = match home::home_dir() {
            Some(dir) => dir,
            None => panic!("home directory is not found"),
        };
        // ~/.guivre
        let app_conf_path = home_path.join(APP_CONFIG_DIR_NAME);
        // ~/.guivre/dynamic_config.toml
        let dyn_conf_path = app_conf_path.join(DYNAMIC_CONFIG_FILE_NAME);
        // ~/.guivre/shell_history
        let shell_hist_path = app_conf_path.join(SHELL_HISTORY_FILE_NAME);

        let dyn_conf = match app_init(&app_conf_path, &dyn_conf_path, &shell_hist_path) {
            Ok(dc) => dc,
            Err(e) => match e {
                AppInitError::InitAlreadyDone => match read_dyn_conf(&dyn_conf_path) {
                    Ok(conf) => conf,
                    Err(e) => return Err(AppConfigError::ReadDynConfError(e.to_string())),
                },
                _ => return Err(AppConfigError::AppInitError(e.to_string())),
            },
        };

        Ok(Self {
            app_conf_path,
            dyn_conf_path,
            shell_hist_path,
            dyn_conf,
        })
    }
}

// アプリケーションを初期化する関数。
// この関数では~/.guivreが存在しないときに~/.guivreを作成し、その配下に初期設定の設定ファイルたちを配置する。
// ~/.guivreが存在するときは存在することを示すエラーを吐く
fn app_init(
    app_conf_path: &PathBuf,
    dyn_conf_path: &PathBuf,
    shell_hist_path: &PathBuf,
) -> Result<DynamicConfig, AppInitError> {
    if match is_entry_exist(app_conf_path) {
        Ok(b) => b,
        Err(e) => {
            return Err(AppInitError::IoError(format!(
                "failed to get if {} exists: {}",
                app_conf_path.to_str().unwrap(),
                e.to_string()
            )))
        }
    } {
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
    let dyn_conf = match init_dyn_conf(dyn_conf_path) {
        Ok(c) => c,
        Err(e) => return Err(AppInitError::DynConfInitError(e.to_string())),
    };
    Ok(dyn_conf)
}
//
// dynamic_config を初期化する関数
// app_init関数から呼び出される
// やることはdynamic_cnfig.tomlの作成とそのファイルへの初期設定の書き込み
fn init_dyn_conf(dyn_conf_path: &PathBuf) -> Result<DynamicConfig, DynConfInitError> {
    // ~/.guivre/dynamic_config.tomlの作成
    let path = dyn_conf_path;
    let file = match fs::File::create(&path) {
        Ok(f) => f,
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
    // dybamic_config.tomlに設定を書き込む
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

    Ok(conf)
}
// dynamic configを読み込む関数
fn read_dyn_conf(dyn_conf_path: &PathBuf) -> Result<DynamicConfig, ReadDynConfError> {
    let file = match fs::File::open(dyn_conf_path) {
        Ok(f) => f,
        Err(e) => {
            return Err(ReadDynConfError::OpenError(format!(
                "failed to open {}: {}",
                dyn_conf_path.to_str().unwrap(),
                e.to_string()
            )))
        }
    };
    let mut reader = io::BufReader::new(&file);
    let mut toml = String::new();
    reader.read_to_string(&mut toml);
    let conf: DynamicConfig = match toml::from_str(&toml) {
        Ok(c) => c,
        Err(e) => return Err(ReadDynConfError::ParseError(e.to_string())),
    };
    Ok(conf)
}

// shell_historyを初期化する関数
// app_init関数から呼び出される
// やることはshell_historyの作成
fn init_shell_hist(shell_hist_path: &PathBuf) -> Result<(), ShellHistInitError> {
    // ~/.guivre/shell_historyの作成
    let path = shell_hist_path;
    let _ = match fs::File::create(&path) {
        Ok(_) => (),
        Err(e) => {
            return Err(ShellHistInitError::IoError(format!(
                "failed to create {}: {}",
                path.to_str().unwrap(),
                e.to_string()
            )))
        }
    };
    Ok(())
}

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
