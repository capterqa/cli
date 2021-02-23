use crate::CliOptions;
use last_git_commit::LastGitCommit;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{env, fs};

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum Source {
    cli,
    ci,
}

impl Default for Source {
    fn default() -> Self {
        Source::cli
    }
}

/// Partial of the event payload on GitHub Actions
///
/// There's a lot of extra information in `workflow/event.json` that
/// we can use to display useful information to the user. We read it
/// through the `GITHUB_EVENT_PATH` environment variable.
#[derive(Deserialize, Serialize, Debug)]
struct GitHubActionsEventPayload {
    action: Option<String>,
    after: Option<String>,
    before: Option<String>,
    number: Option<i32>,
    pull_request: Option<GitHubActionsPullReqeustPayload>,
}

#[derive(Deserialize, Serialize, Debug)]
struct GitHubActionsPullReqeustPayload {
    id: Option<i32>,
    number: Option<i32>,
    title: Option<String>,
    base: Option<GitHubActionsPullReqeustRefPayload>,
    head: Option<GitHubActionsPullReqeustRefPayload>,
}

#[derive(Deserialize, Serialize, Debug)]
struct GitHubActionsPullReqeustRefPayload {
    sha: Option<String>,
}

/// Information about where the run was excecuted. It will
/// collect things like the CI name, some git information
/// and some extra meta data if it's running in an known CI environment.
/// like GitHub actions.
#[derive(Debug, Serialize, Default)]
pub struct RunSource {
    pub source: Source,
    pub sha: Option<String>,
    pub repository: Option<String>,
    pub branch: Option<String>,
    pub ci: Option<String>,
    pub commit_message: Option<String>,
    pub meta: Option<serde_json::Value>,
}

impl RunSource {
    pub fn new(cli_option: &CliOptions) -> RunSource {
        let ci_info = ci_info::get();

        let mut source = RunSource {
            ..Default::default()
        };

        // only collect git information if we're allowed to
        if cli_option.skip_git == false {
            let lgc = LastGitCommit::new().build();
            if let Ok(lgc) = lgc {
                source.branch = Some(lgc.branch().clone());
                source.sha = Some(lgc.id().long().clone());
                if let Some(commit) = lgc.message() {
                    source.commit_message = Some(commit.clone());
                }
            }
        }

        source.source = match ci_info.ci {
            true => Source::ci,
            false => Source::cli,
        };

        if let Some(vendor) = ci_info.name {
            source.ci = Some(vendor);
        }

        // github actions
        if source.ci == Some("GitHub Actions".to_string()) {
            source.sha = Some(env::var("GITHUB_SHA").unwrap());
            source.repository = Some(env::var("GITHUB_REPOSITORY").unwrap());
            let mut meta = json!({
                "env": {
                    "GITHUB_HEAD_REF": env::var("GITHUB_HEAD_REF").unwrap(),
                    "GITHUB_BASE_REF": env::var("GITHUB_BASE_REF").unwrap(),
                    "GITHUB_WORKFLOW": env::var("GITHUB_WORKFLOW").unwrap(),
                    "GITHUB_RUN_ID": env::var("GITHUB_RUN_ID").unwrap(),
                    "GITHUB_ACTOR": env::var("GITHUB_ACTOR").unwrap(),
                }
            });

            // try to read event file
            if let Some(event) = read_github_workflow_event() {
                meta["event"] = json!(event);
            }

            source.meta = Some(meta);
        }

        source
    }
}

/// Reads the event.json file in GitHub Actions.
///
/// This file mimics the payload you would get in a webhook from GitHub
/// and what we are looking for here is mainly information about PRs,
/// like number and title.
fn read_github_workflow_event() -> Option<GitHubActionsEventPayload> {
    let path = env::var("GITHUB_EVENT_PATH").unwrap();

    return match fs::read_to_string(&path) {
        Ok(s) => {
            if let Ok(payload) = serde_json::from_str(&s) {
                return Some(payload);
            }

            None
        }
        _ => None,
    };
}
