use last_git_commit::LastGitCommit;
use serde::Serialize;

#[derive(Debug, Serialize)]
#[allow(non_camel_case_types)]
pub enum RunSource {
    cli,
    ci,
}

impl Default for RunSource {
    fn default() -> Self {
        RunSource::cli
    }
}

#[derive(Debug, Serialize, Default)]
pub struct Source {
    pub source: RunSource,
    pub sha: Option<String>,
    pub repository: Option<String>,
    pub branch: Option<String>,
    pub ci: Option<String>,
    pub commit_message: Option<String>,
    pub meta: Option<serde_json::Value>,
}

pub fn get_source(skip_git: bool) -> Source {
    let ci_info = ci_info::get();

    let mut source = Source {
        source: RunSource::cli,
        ..Default::default()
    };

    if skip_git == false {
        println!("skip git: {}", skip_git);
        let lgc = LastGitCommit::new().build();
        if let Ok(lgc) = lgc {
            source.branch = Some(lgc.branch().clone());
            source.sha = Some(lgc.id().long().clone());
            if let Some(commit) = lgc.message() {
                source.commit_message = Some(commit.clone());
            }
        }
    }

    if ci_info.ci == false {
        return source;
    } else {
        source.source = RunSource::ci;
    };

    if let Some(vendor) = ci_info.name {
        source.ci = Some(vendor);
    }

    source
}
