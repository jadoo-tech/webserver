use std::path::PathBuf;

use tokio::process::Command;
use tracing::{info, error};

use crate::error::Error;

pub async fn shutdown() {
    tokio::signal::ctrl_c().await.unwrap();
    info!("Shutting down");
}

#[derive(Debug, thiserror::Error)]
pub enum CssBuildError {
    #[error("CSS build failed")]
    Failed,
}

pub async fn build_css() -> Result<(), CssBuildError> {
    let res = Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./src/styles.css",
            "-o",
           "./static/styles.css",
            "--minify",
        ])
        .spawn();

    match res {
        Ok(child) => {
            let output = child.wait_with_output().await.unwrap();

            match output.status.success() {
                true => {
                    println!("CSS build successful");
                }
                false => {
                    println!("CSS build failed");
                    dbg!(&output);
                    return Err(CssBuildError::Failed);
                }
            }

            return Ok(());
        }
        Err(e) => {
            println!("CSS build failed: {}", e);
            return Err(CssBuildError::Failed);
        }
    }
}