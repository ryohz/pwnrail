use std::{
    env, fs,
    io::{self, Read, Write},
    path::PathBuf,
};

use crate::{
    error::{
        AppConfigError, AppInitError, CreateNewWorkspaceError, DynConfInitError,
        InitCurrentDirAsWorkspaceError, ReadDynConfError, ShellHistInitError,
        UpdateDynConfFileError, UseCurrentDirAsWorkspaceError,
    },
    output::error_prefix,
};
use serde::{Deserialize, Serialize};

const APP_CONFIG_DIR_NAME: &str = ".pwnrail";
const DYNAMIC_CONFIG_FILE_NAME: &str = "dynamic_config.toml";
const SHELL_HISTORY_FILE_NAME: &str = "shell_history";

const WORKSPACE_DIR_NAME: &str = ".prail";
const VARS_FILE_NAME: &str = "vars.json";

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
    // ~/.pwnrail
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
        // ~/.pwnrail
        let app_conf_path = home_path.join(APP_CONFIG_DIR_NAME);
        // ~/.pwnrail/dynamic_config.toml
        let dyn_conf_path = app_conf_path.join(DYNAMIC_CONFIG_FILE_NAME);
        // ~/.pwnrail/shell_history
        let shell_hist_path = app_conf_path.join(SHELL_HISTORY_FILE_NAME);

        let dyn_conf = match app_init(&app_conf_path, &dyn_conf_path, &shell_hist_path) {
            Ok(dc) => dc,
            Err(e) => match e {
                AppInitError::InitAlreadyDone => match read_dyn_conf(&dyn_conf_path) {
                    Ok(conf) => conf,
                    Err(e) => return Err(AppConfigError::ReadDynConfError(e)),
                },
                _ => return Err(AppConfigError::AppInitError(e)),
            },
        };

        Ok(Self {
            app_conf_path,
            dyn_conf_path,
            shell_hist_path,
            dyn_conf,
        })
    }

    // カレントディレクトリをワークスペースとして初期化する関数
    pub fn init_current_directory_as_workspace(
        &mut self,
    ) -> Result<(), InitCurrentDirAsWorkspaceError> {
        // カレントディレクトリを取得
        let current_dir_path = match env::current_dir() {
            Ok(p) => p,
            Err(e) => return Err(InitCurrentDirAsWorkspaceError::GetCurrentDirError(e)),
        };
        // 作成するワークスペースディレクトリの構造を取得
        let workspace_struct = Workspace::assemble_struct(&current_dir_path);
        // ワークスペース管理ディレクトリ・ファイル群を作成
        let _ = match workspace_struct.create() {
            Ok(_) => (),
            Err(e) => return Err(InitCurrentDirAsWorkspaceError::CreateNewWorkspaceError(e)),
        };
        // 初期化したら、自動で初期化したディレクトリをワークスペースに設定するようにする
        let _ = match self.use_current_dir_as_workspace() {
            Ok(_) => (),
            Err(e) => return Err(InitCurrentDirAsWorkspaceError::UseCurrentDirAsWorkspaceError(e)),
        };
        Ok(())
    }

    // カレントディレクトリをワークスペースとして使うように設定する関数
    pub fn use_current_dir_as_workspace(&mut self) -> Result<(), UseCurrentDirAsWorkspaceError> {
        // カレントディレクトリを取得
        let current_dir_path = match env::current_dir() {
            Ok(p) => p,
            Err(e) => return Err(UseCurrentDirAsWorkspaceError::GetCurrentDirError(e)),
        };
        // カレントディレクトリ配下に管理ディレクトリがあるかどうか確認
        let workspace = Workspace::assemble_struct(&current_dir_path);
        if !match is_entry_exist(&workspace.mgr_path) {
            Ok(b) => b,
            Err(e) => return Err(UseCurrentDirAsWorkspaceError::CheckMgrPresenceError(e)),
        } {
            // ない場合はエラーを返す
            return Err(UseCurrentDirAsWorkspaceError::BeforeInitError);
        }
        // app configのdyanamic configの現在の作業ディレクトリを取得したカレントディレクトリに変更
        self.dyn_conf.current_workspace = current_dir_path.to_str().unwrap().to_string();
        // app configの設定ファイルを更新する
        let _ = match self.update_dyn_conf_file() {
            Ok(_) => (),
            Err(e) => return Err(UseCurrentDirAsWorkspaceError::UpdateDynConfFileError(e)),
        };
        Ok(())
    }

    pub fn update_dyn_conf_file(&self) -> Result<(), UpdateDynConfFileError> {
        let toml_ = match toml::to_string(&self.dyn_conf) {
            Ok(t) => t,
            Err(e) => {
                return Err(UpdateDynConfFileError::ParseError(e));
            }
        };
        let file = match fs::File::create(&self.dyn_conf_path) {
            Ok(f) => f,
            Err(e) => return Err(UpdateDynConfFileError::OpenError(e)),
        };
        let mut writer = io::BufWriter::new(file);
        let _ = match writer.write_all(toml_.as_bytes()) {
            Ok(_) => (),
            Err(e) => return Err(UpdateDynConfFileError::WriteError(e)),
        };
        Ok(())
    }
}

// .prail      管理ディレクトリという呼称にする
//  | vars.json     ipアドレスなどの変数を気軽に収納するためのファイル  varsファイルという呼称にする
struct Workspace {
    pub mgr_path: PathBuf,
    pub vars_path: PathBuf,
}

impl Workspace {
    fn assemble_struct(path: &PathBuf) -> Self {
        let mgr_path = path.join(WORKSPACE_DIR_NAME);
        let vars_path = path.join(VARS_FILE_NAME);
        Self {
            mgr_path,
            vars_path,
        }
    }

    fn create(&self) -> Result<(), CreateNewWorkspaceError> {
        // ワークスペースの管理ディレクトリの存在確認
        if match is_entry_exist(&self.mgr_path) {
            Ok(b) => b,
            Err(e) => return Err(CreateNewWorkspaceError::CheckMgrPresenceError(e)),
        } {
            // 存在したらエラーを返す
            return Err(CreateNewWorkspaceError::MgrAlreadyExists);
        }
        // 管理ディレクトリを作成
        let _ = match fs::create_dir_all(&self.mgr_path) {
            Ok(_) => (),
            Err(e) => return Err(CreateNewWorkspaceError::CreateMgrError(e)),
        };
        // varsファイルを作成
        let _ = match fs::File::create(&self.vars_path) {
            Ok(_) => (),
            Err(e) => return Err(CreateNewWorkspaceError::CreateVarsFileError(e)),
        };

        Ok(())
    }
}

// アプリケーションを初期化する関数。
// この関数では~/.pwnrailが存在しないときに~/.pwnrailを作成し、その配下に初期設定の設定ファイルたちを配置する。
// ~/.pwnrailが存在するときは存在することを示すエラーを吐く
fn app_init(
    app_conf_path: &PathBuf,
    dyn_conf_path: &PathBuf,
    shell_hist_path: &PathBuf,
) -> Result<DynamicConfig, AppInitError> {
    // app confディレクトリの存在確認
    if match is_entry_exist(app_conf_path) {
        Ok(b) => b,
        Err(e) => return Err(AppInitError::CheckAppConfPresenceError(e)),
    } {
        return Err(AppInitError::InitAlreadyDone);
    }

    // app confディレクトリの作成
    let _ = match fs::create_dir_all(app_conf_path) {
        Ok(_) => (),
        Err(e) => return Err(AppInitError::AppConfDirCreateError(e)),
    };

    // dynamic configの初期化
    let dyn_conf = match init_dyn_conf(dyn_conf_path) {
        Ok(c) => c,
        Err(e) => return Err(AppInitError::DynConfInitError(e)),
    };

    // shell historyの初期化
    let _ = match init_shell_hist(shell_hist_path) {
        Ok(_) => (),
        Err(e) => return Err(AppInitError::ShellHistInitError(e)),
    };

    Ok(dyn_conf)
}
//
// dynamic_config を初期化する関数
// app_init関数から呼び出される
// やることはdynamic_cnfig.tomlの作成とそのファイルへの初期設定の書き込み
fn init_dyn_conf(dyn_conf_path: &PathBuf) -> Result<DynamicConfig, DynConfInitError> {
    // dynamic configファイルの作成
    let path = dyn_conf_path;
    let file = match fs::File::create(&path) {
        Ok(f) => f,
        Err(e) => return Err(DynConfInitError::CreateError(e)),
    };
    // 初期のdynamic_config.tomlの書き込み
    //
    // current_workspaceには最初は何も指定しない
    let conf = DynamicConfig {
        current_workspace: "".to_string(),
    };
    // DynamicConfigをTomlファイルに変換する
    let toml_ = match toml::to_string(&conf) {
        Ok(s) => s,
        Err(e) => return Err(DynConfInitError::ParseError(e)),
    };
    // dybamic_config.tomlに設定を書き込む
    let mut writer = io::BufWriter::new(file);
    let _ = match writer.write_all(toml_.as_bytes()) {
        Ok(_) => (),
        Err(e) => return Err(DynConfInitError::WriteError(e)),
    };

    Ok(conf)
}
// dynamic configを読み込む関数
fn read_dyn_conf(dyn_conf_path: &PathBuf) -> Result<DynamicConfig, ReadDynConfError> {
    // dynamic configファイルを開く
    let file = match fs::File::open(dyn_conf_path) {
        Ok(f) => f,
        Err(e) => return Err(ReadDynConfError::OpenError(e)),
    };
    let mut reader = io::BufReader::new(&file);
    let mut toml = String::new();
    let _ = match reader.read_to_string(&mut toml) {
        Ok(_) => (),
        Err(e) => return Err(ReadDynConfError::ReadError(e)),
    };
    let conf: DynamicConfig = match toml::from_str(&toml) {
        Ok(c) => c,
        Err(e) => return Err(ReadDynConfError::ParseError(e)),
    };
    Ok(conf)
}

// shell_historyを初期化する関数
// app_init関数から呼び出される
// やることはshell_historyの作成
fn init_shell_hist(shell_hist_path: &PathBuf) -> Result<(), ShellHistInitError> {
    // ~/.pwnrail/shell_historyの作成
    let path = shell_hist_path;
    let _ = match fs::File::create(&path) {
        Ok(_) => (),
        Err(e) => return Err(ShellHistInitError::CreateError(e)),
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
