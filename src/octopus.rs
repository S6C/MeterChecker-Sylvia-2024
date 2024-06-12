//! Contains rust interfaces for using the [Octopus Energy REST API](https://developer.octopus.energy/rest/)
use http::StatusCode;
use leptos::{server, ServerFnError};
use serde::{Deserialize, Serialize};
use std::env;
use time::OffsetDateTime;

const API_KEY_ENV_VAR: &str = "OCTOPUS_API_KEY";

const API_BASE_URI: &str = "https://api.octopus.energy/v1/";
const METER_POINT_ENDPOINT: &str = "electricity-meter-points/";
const METER_ENDPOINT: &str = "/meters/";
const CONSUMPTION_ENDPOINT: &str = "/consumption/";

#[server(GetConsumption)]
pub async fn get_consumption(
    mpan: String,
    serial: String,
) -> Result<Result<f32, GetConsumptionError>, ServerFnError> {
    let response = reqwest::Client::new()
        .get(
            API_BASE_URI.to_owned()
                + METER_POINT_ENDPOINT
                + &mpan
                + METER_ENDPOINT
                + &serial
                + CONSUMPTION_ENDPOINT,
        )
        .header(
            "Authorization",
            http_auth_basic::Credentials::new(
                &env::var(API_KEY_ENV_VAR).expect(&format!(
                    "{} Environment Variable not found",
                    API_KEY_ENV_VAR
                )),
                "",
            )
            .as_http_header(),
        )
        .query(&[("group_by", "day"), ("page_size", "1")])
        .send()
        .await?;

    match response.status() {
        StatusCode::OK => Ok(Ok(response.json::<Readings>().await?.results[0].consumption)),
        StatusCode::NOT_FOUND => Ok(Err(GetConsumptionError::InvalidMeterDetails)),
        code => {
            println!(
                "Unexpected Octopus response code: {:?}\nresponse body: {}",
                code,
                response.text().await?
            );
            Err(ServerFnError::ServerError(format!(
                "Unexpected Octopus response code: {:?}",
                code
            )))
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub enum GetConsumptionError {
    InvalidMeterDetails,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Readings {
    count: usize,
    next: Option<String>,
    previous: Option<String>,
    results: Vec<ConsumptionInterval>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ConsumptionInterval {
    consumption: f32,
    #[serde(with = "time::serde::rfc3339")]
    interval_start: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    interval_end: OffsetDateTime,
}
