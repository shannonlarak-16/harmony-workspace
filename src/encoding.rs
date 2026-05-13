use crate::{
    chat::{Author, Content, Message, ReasoningEffort, Role, SystemContent, TextContent},
    tiktoken::{CoreBPE, Rank},
};
use anyhow::Context as _;
use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    vec,
};

const REPLACEMENT: &str = "\u{FFFD}";

// Parsed representation of a message header.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ParsedHeader {
    author: Author,
    recipient: Option<String>,
    channel: Option<String>,
    content_type: Option<String>,
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum RenderFormattingTokenError {
    #[error("tried to render unmapped formatting token {0}")]
    UnmappedToken(FormattingToken),

    #[error(
        "Expected encoding of formatting token {token} to be a single token, but got {encoding:?}"
    )]
    InvalidEncoding {
        token: FormattingToken,
        encoding: Vec<Rank>,
    },
}

/// These are formatting tokens that the renderer can use to generically
/// format the output of the model, but at formatting time, they are replaced
/// by actual tokens from the tokenizers vocabulary.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum FormattingToken {
    Start,
    Message,
    EndMessage,
    EndMessageDoneSampling,
    EndMessageAssistantToTool,
    Refusal,
    ConstrainedFormat,
    Channel,
    BeginUntrusted,
    EndUntrusted,
    MetaSep,
    MetaEnd,
}

impl FormattingToken {
    fn as_str(&self) -> &str {
        match self {
            FormattingToken::Start => "<|start|>",
            FormattingToken::Message => "<|message|>",
            FormattingToken::EndMessage => "<|end|>",
            FormattingToken::EndMessageDoneSampling => "<|return|>",
            FormattingToken::EndMessageAssistantToTool => "<|call|>",
            FormattingToken::Refusal => "<|refusal|>",
            FormattingToken::ConstrainedFormat => "<|constrain|>",
            FormattingToken::Channel => "<|channel|>",
            FormattingToken::BeginUntrusted => "<|untrusted|>",
            FormattingToken::EndUntrusted => "<|end_untrusted|>",
            FormattingToken::MetaSep => "<|meta_sep|>",
            FormattingToken::MetaEnd => "<|meta_end|>",
        }
    }
}

impl std::fmt::Display for FormattingToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
