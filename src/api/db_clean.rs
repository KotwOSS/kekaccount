use std::io::Error;

use tokio::time::{sleep, Duration};

use crate::{colors, database::PgPool};

pub async fn main(_pool: &PgPool, interval: u64) -> Result<(), Error> {
    println!(
        "{}START{} db cleaner",
        colors::LIGHT_BLUE,
        colors::RESET
    );

    loop {
        sleep(Duration::from_millis(interval)).await;

        println!(
            "{}CLEANING{} database",
            colors::LIGHT_BLUE,
            colors::RESET
        );
    };
}