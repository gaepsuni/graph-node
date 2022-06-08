use std::sync::Arc;

use anyhow::{format_err, Context, Error};
use graph::{
    env::env_var,
    firehose::FirehoseEndpoint,
    log::logger,
    substreams::{self, ForkStep},
};
use prost::Message;
use tonic::Streaming;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token_env = env_var("SF_API_TOKEN", "".to_string());
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
        panic!("Environment variable SUBSTREAMS_PACKAGE must set");
    }

    let package = read_package(&package_file)?;

    let logger = logger(true);
    let firehose =
        Arc::new(FirehoseEndpoint::new(logger, "substreams", &endpoint, token, false).await?);

    let cursor: Option<String> = None;

    loop {
        println!("Connecting to the stream!");
        let mut stream: Streaming<substreams::Response> = match firehose
            .clone()
            .substreams(substreams::Request {
                // FIXME: Using 0 which I would have expected to mean "use package start's block"
                // does not work, so we specify the range for now.
                start_block_num: 12287507,
                stop_block_num: 12292923,
                modules: package.modules.clone(),
                output_modules: vec!["map_transfers".to_string()],
                start_cursor: match &cursor {
                    Some(c) => c.clone(),
                    None => String::from(""),
                },
                fork_steps: vec![ForkStep::StepNew as i32, ForkStep::StepUndo as i32],
                ..Default::default()
            })
            .await
        {
            Ok(s) => s,
            Err(e) => {
                println!("Could not connect to stream! {}", e);
                continue;
            }
        };

        loop {
            let msg = match stream.message().await {
                Ok(Some(t)) => t.message.expect("always present"),
                Ok(None) => {
                    println!("Stream completed");
                    return Ok(());
                }
                Err(e) => {
                    println!("Error getting message {}", e);
                    break;
                }
            };

            match msg {
                substreams::response::Message::Progress(progress) => process_progress(&progress),
                substreams::response::Message::SnapshotData(_) => {
                    println!("Received snapshot data")
                }
                substreams::response::Message::SnapshotComplete(_) => {
                    println!("Received snapshot complete")
                }
                substreams::response::Message::Data(data) => process_data(&data),
            }
        }
    }
}

fn process_data(data: &substreams::BlockScopedData) {
    if data
        .outputs
        .iter()
        .all(|output| match output.data.as_ref().unwrap() {
            substreams::module_output::Data::MapOutput(result) => result.value.len() == 0,
            substreams::module_output::Data::StoreDeltas(deltas) => deltas.deltas.len() == 0,
        })
    {
        return;
    }

    println!("Received data for {} modules", data.outputs.len());
    data.outputs.iter().for_each(|output| {
        println!(
            "{} => {}",
            output.name,
            match output.data.as_ref().unwrap() {
                substreams::module_output::Data::MapOutput(result) =>
                    format!("{} ({} bytes)", result.type_url, result.value.len()),
                substreams::module_output::Data::StoreDeltas(deltas) => "store delta".to_string(),
            }
        )
    })
}

fn process_progress(progress: &substreams::ModulesProgress) {
    println!("Received progress {}", progress.modules.len())
}

fn read_package(file: &str) -> Result<substreams::Package, anyhow::Error> {
    let content = std::fs::read(file).context(format_err!("read package {}", file))?;

    substreams::Package::decode(content.as_ref()).context("decode command")
}
