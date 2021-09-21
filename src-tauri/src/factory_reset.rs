use std::fs;

use tauri::api::process::kill_children;

use crate::{
  config::{holochain_config_path, holochain_data_path},
  launch::launch_children_processes,
};

pub async fn factory_reset() -> Result<(), String> {
  log::warn!("A factory reset has been requested, initiating...");

  // Kill all the children processes to avoid messing up with the filesystem
  kill_children();
  log::info!("Stopped children processes");

  fs::remove_dir_all(holochain_data_path())
    .or(Err(String::from("Could not remove holochain data path")))?;
  fs::remove_dir_all(holochain_config_path())
    .or(Err(String::from("Could not remove holochain config path")))?;
  log::info!("Cleaned up the file system");

  launch_children_processes().await?;

  log::info!("Started children processes again, factory reset completed");

  Ok(())
}
