use crate::{
    events::IssueCommentEvent,
    metadata::{Metadata, METADATA_FILE},
};
use askama::Template;
use std::collections::HashMap;

/// Template for the vote created comment.
#[derive(Debug, Clone, Template)]
#[template(path = "vote-created.md")]
pub(crate) struct VoteCreated<'a> {
    creator: &'a str,
    issue_title: &'a str,
    issue_number: u64,
    metadata_url: String,
    voters: &'a Vec<String>,
    duration: String,
    pass_threshold: f64,
}

impl<'a> VoteCreated<'a> {
    pub(crate) fn new(event: &'a IssueCommentEvent, md: &'a Metadata) -> Self {
        Self {
            creator: &event.comment.user.login,
            issue_title: &event.issue.title,
            issue_number: event.issue.number,
            metadata_url: format!(
                "https://github.com/{}/blob/HEAD/{}",
                &event.repository.full_name, METADATA_FILE
            ),
            voters: &md.voters,
            duration: humantime::format_duration(md.duration).to_string(),
            pass_threshold: md.pass_threshold,
        }
    }
}

/// Template for the vote closed comment.
#[derive(Debug, Clone, Template)]
#[template(path = "vote-closed.md")]
pub(crate) struct VoteClosed<'a> {
    passed: bool,
    in_favor_percentage: f64,
    pass_threshold: f64,
    in_favor: u64,
    against: u64,
    abstain: u64,
    not_voted: u64,
    voters: HashMap<&'a str, &'a str>,
}
