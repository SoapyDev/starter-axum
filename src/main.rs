mod app;
mod telemetry;
mod config;


#[cfg(feature = "ssr")]
#[tokio::main]
async fn main() {
    use axum::Router;
    use leptos::logging::log;
    use leptos::prelude::*;
    use leptos_axum::{generate_route_list, LeptosRoutes};
    use app::*;
    use telemetry::ssr::{get_subscriber, init_subscriber};
    use config::ssr::get_configuration;

    // Configuration
    let settings = get_configuration().expect("Failed to read configuration.");
    let conf = leptos::prelude::get_configuration(None).expect("Failed to read leptos configuration.");

    // Tracing
    let log_level = settings.tracing_level.clone();
    let subscriber = get_subscriber("Log".into(), log_level, std::io::stdout);
    init_subscriber(subscriber);
    tracing::debug!("Configuration loaded : {:?}", settings);

    // Setup
    let mut leptos_options = conf.leptos_options;
    let port = settings.application.port;
    leptos_options.site_addr.set_port(port);
    let addr = leptos_options.site_addr;

    // Generate the list of routes in your Leptos App
    let routes = generate_route_list(App);

    let app = Router::new()
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    // Run our app with hyper
    tracing::info!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind to address");
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[cfg(not(feature = "ssr"))]
pub fn main() {
    // no client-side main function
    // unless we want this to work with e.g., Trunk for pure client-side testing
    // see lib.rs for hydration function instead
}
