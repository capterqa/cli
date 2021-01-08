mod assert;
mod compile;
mod workflow;

use chrono::{DateTime, Utc};
use glob::glob;
use reqwest::Method;
use serde::Serialize;
use serde_json::json;
use std::fs;
use std::time::Instant;
use workflow::{get_source, parse_yaml, run_workflow, RequestData, WorkflowConfig};

#[derive(Debug, Serialize)]
pub struct WorkflowRun {
    pub workflow: WorkflowConfig,
    pub created_at: DateTime<Utc>,
    pub run_time: i64,
    pub passed: bool,
    pub requests: Vec<RequestData>,
}

#[tokio::main]
async fn main() {
    let mut workflow_runs: Vec<WorkflowRun> = vec![];
    let source = get_source();

    for entry in glob(".capter/**/*.yml").expect("Failed to read glob pattern") {
        let path = entry.expect("Failed to read path");
        println!("{}", path.display());

        let content = fs::read_to_string(path).unwrap();

        let workflow = parse_yaml(content).expect("Failed to parse config.");

        if workflow.skip.is_some() {
            println!("skipping {}", workflow.name);
            continue;
        }

        // track time
        let timer = Instant::now();

        let requests = run_workflow(workflow.clone()).await.unwrap();

        // save time
        let run_time = timer.elapsed().as_millis() as i64;

        let workflow_run = WorkflowRun {
            workflow,
            created_at: Utc::now(),
            requests,
            run_time,
            passed: true,
        };

        workflow_runs.push(workflow_run);
    }

    let client = reqwest::Client::new();
    let _request = client
        .request(Method::POST, "http://localhost:3002/api/webhooks/runs")
        .query(&[("token", "111bd686-f54b-4268-b3f7-0d7f31cc9394")])
        .json(&json!({
            "source": json!(source),
            "data": json!(workflow_runs)
        }))
        .send()
        .await;

    println!("done");
}
