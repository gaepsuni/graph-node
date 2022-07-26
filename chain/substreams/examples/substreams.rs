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
use graph::prelude::{DeploymentHash, Registry, tokio, tonic::Streaming};
use graph::substreams::module_output::Data::{MapOutput, StoreDeltas};
use graph::tokio_stream::{Stream, StreamExt};
use graph_chain_substreams::{Chain, SubstreamBlock};
use graph_core::MetricsRegistry;

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

    let cursor: Option<String> = None;

    match sbs.next().await {
        None => {}
        Some(event) => {
            match event {
                Err(_) => {}
                Ok(block_stream_event) => {

                }
            }
        }
    }

    Ok(())

        // let mut stream: Streaming<substreams::Response> = match firehose
        //     .clone()
        //     .substreams(substreams::Request {
        //         // FIXME: Using 0 which I would have expected to mean "use package start's block"
        //         // does not work, so we specify the range for now.
        //         start_block_num: 12369621,
        //         stop_block_num: 12369821,
        //         modules: package.modules.clone(),
        //         output_modules: vec![module_name.to_string()],
        //         start_cursor: match &cursor {
        //             Some(c) => c.clone(),
        //             None => String::from(""),
        //         },
        //         fork_steps: vec![ForkStep::StepNew as i32, ForkStep::StepUndo as i32],
        //         ..Default::default()
        //     })
        //     .await
        // {
        //     Ok(s) => s,
        //     Err(e) => {
        //         println!("Could not connect to stream! {}", e);
        //         continue;
        //     }
        // };
        //

    // }
}


fn read_package(file: &str) -> Result<substreams::Package, anyhow::Error> {
    let content = std::fs::read(file).context(format_err!("read package {}", file))?;

    substreams::Package::decode(content.as_ref()).context("decode command")
}
