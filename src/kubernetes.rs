use futures::StreamExt;
use k8s_openapi::api::core::v1::Event;
use kube::{
    api::{Api, ListParams},
    Client, Config,
};
use kube_runtime::controller::{Context, Controller, ReconcilerAction};
use log::{info, warn};
use tokio::time::Duration;

pub struct KubeClient {
    client: Client,
}

impl KubeClient {
    pub async fn new() -> anyhow::Result<Self> {
        let config = Config::infer().await?;
        let client = Client::new(config);
        Ok(Self { client: client })
    }

    pub async fn watch_events(&self) -> anyhow::Result<()> {
        let api = Api::<Event>::all(self.client.clone());

        let ctx = Context::new(self.client.clone());

        async fn reconciler(
            event: Event,
            _ctx: Context<Client>,
        ) -> Result<ReconcilerAction, Error> {
            info!("Event : {:?}", event);
            Ok(ReconcilerAction {
                requeue_after: Some(Duration::from_secs(300)),
            })
        }

        Controller::new(api, ListParams::default())
            .run(reconciler, error_policy, ctx)
            .for_each(|res| async move {
                match res {
                    Ok(o) => info!("reconciled {:?}", o),
                    Err(e) => warn!("reconcile failed: {}", e),
                }
            })
            .await;
        Ok(())
    }
}

fn error_policy(_error: &Error, _ctx: Context<Client>) -> ReconcilerAction {
    ReconcilerAction {
        requeue_after: Some(Duration::from_secs(1)),
    }
}

#[derive(Debug, Clone, Copy)]
enum Error {
    ProcessingError,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, _: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        todo!()
    }
}
