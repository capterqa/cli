use crate::CliOptions;
use serde::{Deserialize, Serialize};

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

/// Ref information.
#[derive(Deserialize, Serialize, Debug)]
pub struct RunSourceRef {
    pub sha: Option<String>,
    #[serde(rename(serialize = "ref"))]
    pub _ref: Option<String>,
}

/// Pull request information.
#[derive(Deserialize, Serialize, Debug)]
pub struct RunSourcePullRequest {
    pub id: Option<i32>,
    pub number: Option<i32>,
    pub title: Option<String>,
    pub base: Option<RunSourceRef>,
    pub head: Option<RunSourceRef>,
}

/// Commit information.
#[derive(Deserialize, Serialize, Debug)]
pub struct RunSourceCommit {
    pub sha: Option<String>,
    pub message: Option<String>,
}

/// Information about where the run was excecuted.
///
///It will collect things like the CI name, some git information
/// and some extra meta data if it's running in an known CI environment.
/// like GitHub actions.
#[derive(Debug, Serialize, Default)]
pub struct RunSource {
    pub source: Source,
    pub sha: Option<String>,
    pub repository: Option<String>,
    pub branch: Option<String>,
    pub ci: Option<String>,
    pub commit: Option<RunSourceCommit>,
    pub pr: Option<RunSourcePullRequest>,
    pub meta: Option<serde_json::Value>,
}

impl RunSource {
    pub fn new(_cli_option: &CliOptions) -> RunSource {
        let ci_info = ci_info::get();

        let mut run_source = RunSource {
            ..Default::default()
        };

        run_source.source = match ci_info.ci {
            true => Source::ci,
            false => Source::cli,
        };

        if let Some(vendor) = ci_info.name {
            run_source.ci = Some(vendor);
        }

        // github actions
        if run_source.ci == Some("GitHub Actions".to_string()) {
            run_source.from_github_actions();
        }

        run_source
    }
}
