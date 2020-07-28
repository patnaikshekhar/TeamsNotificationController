mod kubernetes;

use crate::kubernetes::KubeClient;
use anyhow::Result;
use log::info;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let kube_client = KubeClient::new().await?;

    kube_client.watch_events(|event| {
        info!("Main - {:?}", event);
    }).await?;

    Ok(())
}
