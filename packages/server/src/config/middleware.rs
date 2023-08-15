use tower::layer::util::Stack;
use tower::ServiceBuilder;
use tower_http::classify::{ServerErrorsAsFailures, SharedClassifier};
use tower_http::compression::CompressionLayer;
use tower_http::cors::{Any, CorsLayer};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn create_middleware() -> Stack<
    CompressionLayer,
    Stack<
        CorsLayer,
        Stack<TraceLayer<SharedClassifier<ServerErrorsAsFailures>>, tower::layer::util::Identity>,
    >,
> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    let middleware_stack = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .layer(CompressionLayer::new())
        .into_inner();

    middleware_stack
}
