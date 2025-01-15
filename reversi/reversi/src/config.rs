use std::{
    fs::{self, File},
    io::{BufReader, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

use crate::{ml::EarlyStoppingConfig, ResultBoxErr};

#[derive(Debug, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub data_for_training: String,
    pub data_for_validation: String,
    pub epochs: usize,
    pub batch_size: usize,
    pub early_stopping: EarlyStoppingConfig,
    pub output_file: String,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            data_for_training: "train.bin".to_string(),
            data_for_validation: "valid.bin".to_string(),
            epochs: 100,
            batch_size: 32,
            early_stopping: EarlyStoppingConfig {
                patience: 10,
                min_delta: 0.001,
            },
            output_file: "model.bin".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SelfPlayConfig {
    pub num_games: usize,
    pub output_file: String,
}

impl Default for SelfPlayConfig {
    fn default() -> Self {
        Self {
            num_games: 1000,
            output_file: "records.bin".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub base_path: String,
    pub training: TrainingConfig,
    pub gen_data_for_training: SelfPlayConfig,
    pub gen_data_for_validation: SelfPlayConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            base_path: "data".to_string(),
            training: Default::default(),
            gen_data_for_training: SelfPlayConfig {
                num_games: 1000,
                output_file: "train.bin".to_string(),
            },
            gen_data_for_validation: SelfPlayConfig {
                num_games: 300,
                output_file: "valid.bin".to_string(),
            },
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

    pub fn training_data_for_training_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.training.data_for_training)
    }

    pub fn training_data_for_validation_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.training.data_for_validation)
    }

    pub fn training_output_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.training.output_file)
    }

    pub fn gen_data_for_training_output_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.gen_data_for_training.output_file)
    }

    pub fn gen_data_for_validation_output_path(&self) -> PathBuf {
        Path::new(&self.base_path).join(&self.gen_data_for_validation.output_file)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.training.epochs == 0 {
            return Err("エポック数は0より大きくなければなりません。".to_string());
        }
        if self.training.batch_size == 0 {
            return Err("バッチサイズは0より大きくなければなりません。".to_string());
        }
        if self.gen_data_for_validation.num_games == 0 {
            return Err("対局数は0より大きくなければなりません。".to_string());
        }
        if !Path::new(&self.base_path).exists() {
            return Err(format!("base_path が存在しません: {}", self.base_path));
        }

        if let Some(parent) = self.gen_data_for_training_output_path().parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("出力ディレクトリの作成に失敗しました: {}", e))?;
            }
        }

        if let Some(parent) = self.gen_data_for_validation_output_path().parent() {
            if !parent.exists() {
                fs::create_dir_all(parent)
                    .map_err(|e| format!("出力ディレクトリの作成に失敗しました: {}", e))?;
            }
        }

        Ok(())
    }
}
