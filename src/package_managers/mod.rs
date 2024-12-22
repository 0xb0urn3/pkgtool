
use async_trait::async_trait;
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

mod apt;
mod pacman;
mod dnf;
mod common;

pub use apt::AptManager;
pub use pacman::PacmanManager;
pub use dnf::DnfManager;

#[async_trait]
pub trait PackageManager: Send + Sync {
    async fn initialize(&self) -> anyhow::Result<()>;
    async fn search(&self, query: &str) -> anyhow::Result<Vec<PackageInfo>>;
    async fn install(&self, packages: &[String]) -> anyhow::Result<()>;
    async fn remove(&self, packages: &[String]) -> anyhow::Result<()>;
    async fn update_system(&self) -> anyhow::Result<()>;
    async fn get_updates(&self) -> anyhow::Result<Vec<PackageUpdate>>;
}
