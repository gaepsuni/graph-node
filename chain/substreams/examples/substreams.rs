use std::sync::Arc;
use std::env;
use std::task::Poll;

use anyhow::{format_err, Context, Error};
use graph::{
    env::env_var,
    firehose::FirehoseEndpoint,
    log::logger,
    substreams::{self, ForkStep},
};
use prost::{DecodeError, Message};
use graph::blockchain::block_stream::BlockStreamEvent;
use graph::blockchain::substreams_block_stream;
use graph::blockchain::substreams_block_stream::SubstreamsBlockStream;
use graph::prelude::{DeploymentHash, info, Registry, tokio, tonic::Streaming};
use graph::slog::Logger;
use graph::substreams::module_output::Data;
use graph::substreams::module_output::Data::{MapOutput, StoreDeltas};
use graph::substreams::ModuleOutput;
use graph::tokio_stream::{Stream, StreamExt};
use graph_chain_substreams::{Chain, SubstreamBlock};
use graph_core::MetricsRegistry;
use graph_chain_substreams::codec::EntitiesChanges;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let module_name = env::args().nth(1).unwrap();

    let token_env = env_var("SUBSTREAMS_API_TOKEN", "".to_string());
    let mut token: Option<String> = None;
    if token_env.len() > 0 {
        token = Some(token_env);
    }

    let endpoint = env_var(
        "SUBSTREAMS_ENDPOINT",
        "https://api-dev.streamingfast.io".to_string(),
    );

    let package_file = env_var("SUBSTREAMS_PACKAGE", "".to_string());
    if package_file == "" {
        panic!("Environment variable SUBSTREAMS_PACKAGE must be set");
    }

    let package = read_package(&package_file)?;

    let logger = logger(true);
    // Set up Prometheus registry
    let prometheus_registry = Arc::new(Registry::new());
    let metrics_registry = Arc::new(MetricsRegistry::new(
        logger.clone(),
        prometheus_registry.clone(),
    ));

    let firehose =
        Arc::new(FirehoseEndpoint::new(logger.clone(), "substreams", &endpoint, token, false).await?);

    let mut sbs: SubstreamsBlockStream<graph_chain_substreams::Chain> = SubstreamsBlockStream::new(
        DeploymentHash::new("substreams".to_string()).unwrap(),
        firehose.clone(),
        None,
        None,
        package.modules.clone(),
        module_name.to_string(),
        vec![12369621],
        vec![12370000],
        logger.clone(),
        metrics_registry
    );

    loop {
        match sbs.next().await {
            None => {
                break;
            }
            Some(event) => {
                match event {
                    Err(_) => {}
                    Ok(block_stream_event) => {
                        match block_stream_event {
                            BlockStreamEvent::ProcessSubstreamsBlock(msg, _) => {
                                if msg.outputs.len() == 0 {
                                    continue;
                                }
                                if msg.outputs.len() > 1 {
                                    panic!("expected only 1 module output")
                                }
                                if msg.outputs[0].name != "graph_out" {
                                    panic!("expected module name graph_out, got: {}", msg.outputs[0].name);
                                }
                                match msg.outputs[0].data.as_ref().unwrap() {
                                    MapOutput(map_outputs) => {
                                        let entities_changes: EntitiesChanges = Message::decode(map_outputs.value.as_slice()).unwrap();
                                        for changes in entities_changes.entity_changes {
                                            info!(&logger, "----- Entity -----");
                                            info!(&logger, "name: {} operation: {}", changes.entity, changes.operation);
                                            for field in changes.fields {
                                                info!(&logger, "field: {}, type: {}", field.name, field.value_type);
                                                info!(&logger, "new value: {}", hex::encode(field.new_value));
                                                info!(&logger, "old value: {}", hex::encode(field.old_value));
                                            }
                                        }
                                    }
                                    StoreDeltas(_) => {
                                        panic!("expecting map output for graph_out module")
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }
        }
    }

    Ok(())
}


fn read_package(file: &str) -> Result<substreams::Package, anyhow::Error> {
    let content = std::fs::read(file).context(format_err!("read package {}", file))?;

    substreams::Package::decode(content.as_ref()).context("decode command")
}
