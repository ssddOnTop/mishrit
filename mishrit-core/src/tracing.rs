// inspired from https://github.com/tailcallhq/tailcall/blob/main/src/core/tracing.rs

use reqwest::{Client, Error, Response};
use std::str::FromStr;
use std::sync::Arc;
use std::{env, fmt};
use tracing::instrument::WithSubscriber;
use tracing::{Event, Subscriber};
use tracing_subscriber::fmt::FormatFields;
use tracing_subscriber::registry::LookupSpan;
use tracing_subscriber::Layer;

use colored::Colorize;
use tracing::level_filters::LevelFilter;
use tracing::{Level, Metadata};
use tracing_subscriber::filter::{filter_fn, FilterFn};
use tracing_subscriber::fmt::format::Writer;
use tracing_subscriber::fmt::{FmtContext, FormatEvent};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::registry;

#[derive(Debug, Clone)]
pub struct WebhookLayer {
    webhook_url: String,
    client: Arc<Client>,
}

impl WebhookLayer {
    pub fn new<T: AsRef<str>>(url: T) -> Self {
        Self {
            webhook_url: url.as_ref().to_string(),
            client: Arc::new(Client::new()),
        }
    }
}

impl<S> Layer<S> for WebhookLayer
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(&self, event: &Event, _ctx: tracing_subscriber::layer::Context<S>) {
        let mut log = "alo";
        let client = self.client.clone();
        let webhook_url = self.webhook_url.clone();
        tokio::spawn(async move {
            let payload = serde_json::json!({ "content": log });
            let _v = client.post(&webhook_url).json(&payload).send().await;
        });
    }
}

struct FmtLevel<'a> {
    level: &'a Level,
    ansi: bool,
}

impl<'a> FmtLevel<'a> {
    pub(crate) fn new(level: &'a Level, ansi: bool) -> Self {
        Self { level, ansi }
    }
}

const TRACE_STR: &str = "TRACE";
const DEBUG_STR: &str = "DEBUG";
const INFO_STR: &str = "INFO";
const WARN_STR: &str = "WARN";
const ERROR_STR: &str = "ERROR";

impl<'a> fmt::Display for FmtLevel<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.ansi {
            match *self.level {
                Level::TRACE => write!(f, "{:>5} ", TRACE_STR.magenta()),
                Level::DEBUG => write!(f, "{:>5} ", DEBUG_STR.blue()),
                Level::INFO => write!(f, "{:>5} ", INFO_STR.green()),
                Level::WARN => write!(f, "{:>5} ", WARN_STR.yellow()),
                Level::ERROR => write!(f, "{:>5} ", ERROR_STR.red()),
            }
        } else {
            match *self.level {
                Level::TRACE => f.pad(TRACE_STR),
                Level::DEBUG => f.pad(DEBUG_STR),
                Level::INFO => f.pad(INFO_STR),
                Level::WARN => f.pad(WARN_STR),
                Level::ERROR => f.pad(ERROR_STR),
            }
        }
    }
}

struct CliFmt;

impl<S, N> FormatEvent<S, N> for CliFmt
where
    S: Subscriber + for<'a> LookupSpan<'a>,
    N: for<'a> FormatFields<'a> + 'static,
{
    fn format_event(
        &self,
        ctx: &FmtContext<'_, S, N>,
        mut writer: Writer<'_>,
        event: &Event<'_>,
    ) -> fmt::Result {
        let meta = event.metadata();
        let fmt_level = FmtLevel::new(meta.level(), writer.has_ansi_escapes());
        write!(writer, "{}", fmt_level)?;
        ctx.field_format().format_fields(writer.by_ref(), event)?;

        writeln!(writer)
    }
}

pub fn default_tracing_for_name(name: &'static str, layer: WebhookLayer) -> impl Subscriber {
    registry()
        .with(layer)
        .with(default_tracing().with_filter(filter_target(name)))
}

pub fn get_log_level() -> Option<Level> {
    const LONG_ENV_FILTER_VAR_NAME: &str = "MISHRIT_LOG_LEVEL";
    const SHORT_ENV_FILTER_VAR_NAME: &str = "MT_LOG_LEVEL";

    env::var(LONG_ENV_FILTER_VAR_NAME)
        .or(env::var(SHORT_ENV_FILTER_VAR_NAME))
        .ok()
        .and_then(|v| Level::from_str(&v).ok())
}

fn default_tracing<S>() -> impl Layer<S>
where
    S: Subscriber,
    for<'a> S: registry::LookupSpan<'a>,
{
    tracing_subscriber::fmt::layer()
        .without_time()
        .with_target(false)
        .event_format(CliFmt)
        .with_filter(LevelFilter::from_level(
            get_log_level().unwrap_or(Level::INFO),
        ))
}

pub fn mishrit_filter_target() -> FilterFn<impl Fn(&Metadata<'_>) -> bool> {
    filter_target("mishrit")
}

pub fn filter_target(name: &'static str) -> FilterFn<impl Fn(&Metadata<'_>) -> bool> {
    filter_fn(move |metadata: &Metadata<'_>| metadata.target().starts_with(name))
}
