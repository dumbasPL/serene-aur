pub mod runner;
pub mod package;

mod repository;
mod web;
pub mod config;
mod build;
mod database;

use std::sync::{Arc};
use actix_web::{App, HttpMessage, HttpServer};
use actix_web::web::Data;
use anyhow::Context;
use tokio::sync::{RwLock};
use crate::build::schedule::BuildScheduler;
use crate::build::Builder;
use crate::config::CONFIG;
use crate::package::Package;
use crate::runner::{Runner};
use crate::repository::PackageRepository;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // initializing database
    let db = database::connect().await?;

    // initializing runner
    let runner = Arc::new(RwLock::new(
        Runner::new()
            .context("failed to connect to docker")?
    ));

    // initializing repository
    let repository = Arc::new(RwLock::new(
        PackageRepository::new().await
            .context("failed to create package repository")?
    ));

    // initializing builder
    let builder = Arc::new(RwLock::new(
        Builder::new(db.clone(), runner.clone(), repository.clone())
    ));

    // creating scheduler
    let mut schedule = BuildScheduler::new(builder.clone()).await
        .context("failed to start package scheduler")?;

    for package in Package::find_all(&db).await? {
        schedule.schedule(&package).await
            .context(format!("failed to start schedule for package {}", &package.base))?;
    }

    schedule.start().await?;

    let schedule = Arc::new(RwLock::new(schedule));

    // web app
    HttpServer::new(move ||
        App::new()
            .app_data(Data::new(db.clone()))
            .app_data(Data::from(schedule.clone()))
            .app_data(Data::from(builder.clone()))
            .service(repository::webservice())
            .service(web::add)
            .service(web::list)
            .service(web::status)
            .service(web::remove)
            .service(web::build)
            .service(web::get_build)
            .service(web::get_logs)
            .service(web::settings)
    ).bind(("0.0.0.0", CONFIG.port))?.run().await?;

    Ok(())
}



