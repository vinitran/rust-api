use tracing::{level_filters::LevelFilter, Metadata};
use tracing_subscriber::{
    fmt,
    layer::{Filter, SubscriberExt},
    util::SubscriberInitExt,
    EnvFilter, Layer,
};

struct InfoOnlyFilter;

impl<S> Filter<S> for InfoOnlyFilter {
    fn enabled(&self, meta: &Metadata<'_>, _: &tracing_subscriber::layer::Context<'_, S>) -> bool {
        meta.level() == &tracing::Level::INFO
    }
}

pub fn set_up(directive: &'static str) {
    let filter = EnvFilter::builder()
        .from_env()
        .unwrap()
        .add_directive(directive.parse().unwrap());

    let error_layer = fmt::Layer::new()
        .with_writer(std::io::stderr)
        .with_filter(LevelFilter::ERROR);

    let info_layer = fmt::Layer::new()
        .with_writer(std::io::stdout)
        .with_filter(InfoOnlyFilter)
        .with_filter(filter);

    tracing_subscriber::registry()
        .with(error_layer)
        .with(info_layer)
        .init();
}
