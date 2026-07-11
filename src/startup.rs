use crate::configuration::{DatabaseSettings, Settings};
use crate::routes::{health_check, subscribe};

use actix_web::dev::Server;
use actix_web::{App, HttpServer, web};
use actix_web::web::Data;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> Result<Self, anyhow::Error> {
        let connection_pool = get_connection_pool(&configuration.database);
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, connection_pool, ).await?;

        Ok(Self { port, server })
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn get_connection_pool(configuration: &DatabaseSettings) -> PgPool {
    // todo come back to this
    PgPoolOptions::new().connect_lazy_with(configuration.connect_options())
}

async fn run(
    listener: TcpListener,
    db_pool: PgPool,
) -> Result<Server, anyhow::Error> {
    // Essentially wraps it in ARC so we can have multiple actix threads reference
    // the services -> i.e. each server uses closure to spawn multiple workers
    // so app states need be shareable
    let db_pool = Data::new(db_pool);
    let server = HttpServer::new(move || {
        App::new()
            // .route("/", web.get().to(home))
            .route("/health_check", web::get().to(health_check))
            .route("/subscriptions", web::post().to(subscribe))
            .app_data(db_pool.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}
