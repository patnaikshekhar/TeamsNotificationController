mod kubernetes;

use crate::kubernetes::KubeClient;
use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let kube_client = KubeClient::new().await?;

    kube_client.watch_events(|event| {}).await?;

    Ok(())
}
