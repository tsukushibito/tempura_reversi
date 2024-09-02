use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{ml::EarlyStoppingConfig, ResultBoxErr};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub train_data_file: String,
    pub valid_data_file: String,
    pub epochs: usize,
    pub batch_size: usize,
    pub early_stopping: EarlyStoppingConfig,
    pub models_file: String,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            train_data_file: "train.bin".to_string(),
            valid_data_file: "valid.bin".to_string(),
            epochs: 100,
            batch_size: 32,
            early_stopping: EarlyStoppingConfig {
                patience: 10,
                min_delta: 0.001,
            },
            models_file: "models.bin".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GenDataConfig {
    pub num_games_for_train: usize,
    pub num_games_for_valid: usize,
    pub train_file: String,
    pub valid_file: String,
}

impl Default for GenDataConfig {
    fn default() -> Self {
        Self {
            num_games_for_train: 1000,
            num_games_for_valid: 300,
            train_file: "train.bin".to_string(),
            valid_file: "valid.bin".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub base_path: String,
    pub training: TrainingConfig,
    pub gen_data: GenDataConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_path: "data".to_string(),
            training: Default::default(),
            gen_data: Default::default(),
        }
    }
}

impl Config {
    pub fn from_file<P: AsRef<Path>>(path: P) -> ResultBoxErr<Self> {
        if !path.as_ref().exists() {
            let default_config = Config::default();
            default_config.save_to_file(&path)?;
            println!(
                "設定ファイルが存在しなかったため、デフォルト設定で {} を作成しました。",
                path.as_ref().display()
            );
            return Ok(default_config);
        }

        let file = File::open(&path)?;
        let reader = BufReader::new(file);
        let config = serde_json::from_reader(reader)?;
        Ok(config)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> ResultBoxErr<()> {
        let json = serde_json::to_string_pretty(&self)?;
        let mut file = File::create(path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }

    pub fn training_train_data_file_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.training.train_data_file)
    }

    pub fn training_valid_data_file_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.training.valid_data_file)
    }

    pub fn training_models_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.training.models_file)
    }

    pub fn gen_data_train_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.gen_data.train_file)
    }

    pub fn gen_data_valid_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.gen_data.valid_file)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.training.epochs == 0 {
            return Err("エポック数は0より大きくなければなりません。".to_string());
        }
        if self.training.batch_size == 0 {
            return Err("バッチサイズは0より大きくなければなりません。".to_string());
        }
        if self.gen_data.num_games_for_train == 0 {
            return Err("対局数は0より大きくなければなりません。".to_string());
        }
        if self.gen_data.num_games_for_valid == 0 {
            return Err("対局数は0より大きくなければなりません。".to_string());
        }
        if !Path::new(&self.base_path).exists() {
            return Err(format!("base_path が存在しません: {}", self.base_path));
        }

        if let Some(parent) = self.gen_data_train_path().parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("出力ディレクトリの作成に失敗しました: {}", e))?;
            }
        }

        if let Some(parent) = self.gen_data_valid_path().parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("出力ディレクトリの作成に失敗しました: {}", e))?;
            }
        }

        Ok(())
    }
}
