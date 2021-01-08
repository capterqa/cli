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

pub fn get_source() -> Source {
    let info = ci_info::get();

    let mut source = Source {
        source: RunSource::cli,
        ..Default::default()
    };

    if info.ci == false {
        return source;
    } else {
        source.source = RunSource::ci;
    };

    if let Some(branch_name) = info.branch_name {
        source.branch = Some(branch_name);
    }

    if let Some(vendor) = info.name {
        source.ci = Some(vendor);
    }

    source
}
