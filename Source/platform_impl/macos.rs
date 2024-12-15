#![cfg(target_os = "macos")]

use tauri::{
	Manager,
	Runtime,
	plugin::{self, TauriPlugin},
};

use crate::SingleInstanceCallback;
pub fn init<R:Runtime>(f:Box<SingleInstanceCallback<R>>) -> TauriPlugin<R> {
	plugin::Builder::new("single-instance").build()
}

pub fn destroy<R:Runtime, M:Manager<R>>(_manager:&M) {}
