use std::fs::File;
use std::fs;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize)]
pub struct UncodeConfig {
  pub path: String,
}

impl Default for UncodeConfig {
  fn default() -> Self {
    UncodeConfig {
      path: "".to_string()
    }
  }
}

impl UncodeConfig {
  pub fn save_config (&self){
    if self.path == ""{
      error!("workspace path is empty");
      return;
    }

    let ws = serde_json::to_string_pretty(self.clone()).expect("error");
    let path = UncodeConfig::config_path().expect("lost home issue");
    if !path.exists() {
      let _file = File::create(&path).unwrap();
    }

    let result = fs::write(&path, ws);
    match result {
      Ok(_) => log::info!("save config: {:?}", path),
      Err(e) => log::info!("failed to write data: {}", { e }),
    }
  }
  pub fn json(&self) -> String {
    serde_json::to_string(self.clone()).expect("error")
  }

  pub fn read_config() -> UncodeConfig {
    let mut app_state = UncodeConfig::default();
    let path = UncodeConfig::config_path().expect("lost home issue");
    let content;
    match fs::read_to_string(&path) {
      Ok(str) => {
        content = str;
      }
      Err(_) => {
        return app_state;
      }
    }

    match serde_json::from_str(&content) {
      Ok(state) => {
        app_state = state;
      }
      Err(_err) => {
        log::error!("error config: {}", content);
      }
    };
    return app_state;
  }

  pub fn config_path() -> Option<PathBuf> {
    let home = dirs::home_dir()?;
    let base = home.join(".uncode");
    if !&base.exists() {
      let _ = fs::create_dir_all(&base);
    }
    let config_path = base.join("uncode.json");
    Some(config_path)
  }

}