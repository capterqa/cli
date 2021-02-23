use crate::workflow::{
    run_source::{RunSourceCommit, RunSourcePullRequest, RunSourceRef},
    RunSource,
};
use serde::{Deserialize, Serialize};
use std::{env, fs};

/// Partial of the event payload on GitHub Actions.
///
/// There's a lot of information in `workflow/event.json` that
/// we can use to display useful information to the user. We read it
/// through the `GITHUB_EVENT_PATH` environment variable.
#[derive(Deserialize, Serialize, Debug)]
struct GitHubActionsEventPayload {
    action: Option<String>,
    after: Option<String>,
    before: Option<String>,
    number: Option<i32>,
    pull_request: Option<GitHubActionsPullRequestPayload>,
    head_commit: Option<GitHubActionsCommitPayload>,
    #[serde(alias = "ref")]
    _ref: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct GitHubActionsPullRequestPayload {
    id: Option<i32>,
    number: Option<i32>,
    title: Option<String>,
    base: Option<GitHubActionsPullRequestRefPayload>,
    head: Option<GitHubActionsPullRequestRefPayload>,
}

impl From<GitHubActionsPullRequestPayload> for RunSourcePullRequest {
    fn from(item: GitHubActionsPullRequestPayload) -> Self {
        RunSourcePullRequest {
            id: item.id,
            title: item.title,
            number: item.number,
            head: match item.head {
                Some(val) => Some(val.into()),
                _ => None,
            },
            base: match item.base {
                Some(val) => Some(val.into()),
                _ => None,
            },
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct GitHubActionsCommitPayload {
    id: Option<String>,
    message: Option<String>,
    author: Option<GitHubActionsUserPayload>,
    committer: Option<GitHubActionsUserPayload>,
}

impl From<GitHubActionsCommitPayload> for RunSourceCommit {
    fn from(item: GitHubActionsCommitPayload) -> Self {
        RunSourceCommit {
            message: item.message,
            sha: item.id,
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
struct GitHubActionsUserPayload {
    email: Option<String>,
    name: Option<String>,
    username: Option<String>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
struct GitHubActionsPullRequestRefPayload {
    sha: Option<String>,
    #[serde(alias = "ref")]
    _ref: Option<String>,
}

impl From<GitHubActionsPullRequestRefPayload> for RunSourceRef {
    fn from(item: GitHubActionsPullRequestRefPayload) -> Self {
        RunSourceRef {
            _ref: item._ref,
            sha: item.sha,
        }
    }
}

impl RunSource {
    pub fn from_github_actions(&mut self) {
        // by default we use the environment variables
        // these doesn't work with all events, so we'll try to
        // use the event.json file later to update these
        self.sha = Some(env::var("GITHUB_SHA").unwrap());
        self.repository = Some(env::var("GITHUB_REPOSITORY").unwrap());

        if let Some(event) = read_github_workflow_event() {
            self.add_github_event_to_run_source(event);
        };
    }

    /// Adds event properties to self
    fn add_github_event_to_run_source(&mut self, event: GitHubActionsEventPayload) {
        // if event has a commit
        if let Some(commit) = event.head_commit {
            self.sha = commit.id.clone();
            self.branch = event._ref.clone();
            self.commit = Some(commit.into());
        }
        // if event has pr
        if let Some(pr) = &event.pull_request {
            self.pr = Some(pr.clone().into());
            if let Some(head) = &pr.head {
                self.sha = head.sha.clone();
                self.branch = head._ref.clone();
            }
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_github_actions() {
        let mut run_source = RunSource {
            ..Default::default()
        };

        let event = json!({
            "head_commit": {
                "id": "commit-sha",
                "message": "commit-message",
            },
            "pull_request": {
                "title": "pr-title",
                "number": 5
            },
            "ref": "event-ref"
        })
        .to_string();

        let event: GitHubActionsEventPayload = serde_json::from_str(&event).unwrap();
        run_source.add_github_event_to_run_source(event);

        assert_eq!(run_source.branch, Some("event-ref".to_string()));
        let commit = run_source.commit.unwrap();
        let pr = run_source.pr.unwrap();

        assert_eq!(commit.sha, Some("commit-sha".to_string()));
        assert_eq!(commit.message, Some("commit-message".to_string()));

        assert_eq!(pr.title, Some("pr-title".to_string()));
        assert_eq!(pr.number, Some(5));
    }
}
