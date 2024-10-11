use tokio::process::Command;
use tracing::{info, error};

use crate::error::Error;

pub async fn shutdown() {
    tokio::signal::ctrl_c().await.unwrap();
    info!("Shutting down");
}

pub async fn build_css() {
    let res = Command::new("tailwind")
        .args([
            "-i",
            "./src/styles.css",
            "-o",
            "./static/styles.css",
            "--minify",
        ])
        .status()
        .await;

    match res {
        Ok(status) => {
            if !status.success() {
                error!("CSS build failed");
            } else {
                println!("CSS build successful");
            }
        }
        Err(e) => {
            error!("CSS build failed: {}", e);
        }
    }
}