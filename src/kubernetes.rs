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

    pub async fn watch_events<F>(&self, callback: F) -> anyhow::Result<()>
    where
        F: 'static + Fn(Event),
    {
        let api = Api::<Event>::all(self.client.clone());

        let ctx = Context::new(MainContext {
            client: self.client.clone(),
            f: Box::new(callback),
        });

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

fn error_policy(_error: &Error, _ctx: Context<MainContext>) -> ReconcilerAction {
    ReconcilerAction {
        requeue_after: Some(Duration::from_secs(1)),
    }
}

async fn reconciler(event: Event, ctx: Context<MainContext>) -> Result<ReconcilerAction, Error> {
    info!("Event : {:?}", event);
    let f = &ctx.get_ref().f;
    f(event);

    Ok(ReconcilerAction {
        requeue_after: Some(Duration::from_secs(300)),
    })
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

struct MainContext {
    client: Client,
    f: Box<dyn Fn(Event)>,
}
