mod busstop;

use anyhow::Result;
use busstop::busstop_request;
use rmcp::{
    ServerHandler, ServiceExt,
    handler::server::{router::tool::ToolRouter, tool::Parameters},
    model::*,
    schemars, tool, tool_handler, tool_router,
};
use serde::Deserialize;

pub struct BusStop {
    tool_router: ToolRouter<BusStop>,
}

#[derive(serde::Deserialize, schemars::JsonSchema)]
pub struct BusStopNumberRequest {
    /// Singapore bus stop code, e.g. "65561"
    bus_stop_code: String,
}

#[derive(Debug, Deserialize)]
pub struct BusStopResponse {
    #[serde(rename = "BusStopCode")]
    pub bus_stop_code: String,

    #[serde(rename = "Services")]
    pub services: Vec<BusService>,

    // Key is literally "odata.metadata" in the JSON
    #[serde(rename = "odata.metadata")]
    pub odata_metadata: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct BusService {
    #[serde(rename = "ServiceNo")]
    pub service_no: String,

    #[serde(rename = "Operator")]
    pub operator: String,

    #[serde(rename = "NextBus")]
    pub next_bus: Option<BusArrival>,

    #[serde(rename = "NextBus2")]
    pub next_bus2: Option<BusArrival>,

    #[serde(rename = "NextBus3")]
    pub next_bus3: Option<BusArrival>,
}

#[derive(Debug, Deserialize)]
pub struct BusArrival {
    #[serde(rename = "EstimatedArrival")]
    pub estimated_arrival: Option<String>,

    #[serde(rename = "Latitude")]
    pub latitude: Option<String>,

    #[serde(rename = "Longitude")]
    pub longitude: Option<String>,

    #[serde(rename = "Load")]
    pub load: Option<String>,

    #[serde(rename = "Feature")]
    pub feature: Option<String>,

    #[serde(rename = "Type")]
    pub bus_type: Option<String>,

    #[serde(rename = "VisitNumber")]
    pub visit_number: Option<String>,

    #[serde(rename = "OriginCode")]
    pub origin_code: Option<String>,

    #[serde(rename = "DestinationCode")]
    pub destination_code: Option<String>,

    #[serde(rename = "Monitored")]
    pub monitored: Option<u8>,
}

// fn fmt_arrival(a: &Option<BusArrival>) -> String {
//     match a {
//         None => "-".to_string(),
//         Some(a) => {
//             let eta = a.estimated_arrival.as_deref().unwrap_or("-");
//             let load = a.load.as_deref().unwrap_or("-");
//             let feat = a.feature.as_deref().unwrap_or("-");
//             format!("ETA: {eta} | Load: {load} | Feature: {feat}")
//         }
//     }
// }

// fn format_busstop_response(r: &BusStopResponse) -> String {
//     if r.services.is_empty() {
//         return format!("Bus stop {}: no services returned.", r.bus_stop_code);
//     }

//     let mut lines = Vec::new();
//     lines.push(format!("Bus stop: {}", r.bus_stop_code));

//     for s in &r.services {
//         lines.push(format!(
//             "Service {} ({})\n  Next:  {}\n  Next2: {}\n  Next3: {}",
//             s.service_no,
//             s.operator,
//             fmt_arrival(&s.next_bus),
//             fmt_arrival(&s.next_bus2),
//             fmt_arrival(&s.next_bus3),
//         ));
//     }

//     lines.join("\n")
// }

#[tool_router]
impl BusStop {
    fn new() -> Self {
        Self {
            tool_router: Self::tool_router(),
        }
    }

    #[tool(description = "Get bus timings for a bus stop.")]
    async fn get_bus_timings(
        &self,
        Parameters(BusStopNumberRequest { bus_stop_code }): Parameters<BusStopNumberRequest>,
    ) -> String {
        // match busstop_request::<BusStopResponse>(&bus_stop_code).await {
        //     Ok(response) => format_busstop_response(&response),
        //     Err(e) => format!("Error: {:?}", e),
        // }
        // match busstop_request::<BusStopResponse>(&bus_stop_code).await {
        //     Ok(response) => serde_json::to_string_pretty(&response)
        //         .unwrap_or_else(|e| format!("Serialize error: {:?}", e)),
        //     Err(e) => format!("Error: {:?}", e),
        // }
        match busstop_request::<serde_json::Value>(&bus_stop_code).await {
            Ok(response) => serde_json::to_string_pretty(&response)
                .unwrap_or_else(|e| format!("Serialize error: {:?}", e)),
            Err(e) => format!("Error: {:?}", e),
        }
    }
}

#[tool_handler]
impl ServerHandler for BusStop {
    fn get_info(&self) -> ServerInfo {
        ServerInfo {
            capabilities: ServerCapabilities::builder().enable_tools().build(),
            ..Default::default()
        }
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let transport = (tokio::io::stdin(), tokio::io::stdout());
    let service = BusStop::new().serve(transport).await?;
    service.waiting().await?;
    Ok(())
}

// #[tokio::main]
// async fn main() {
//     print!("Starting BusStop service...\n");
//     let bus_stop_code = "65561";

//     match busstop_request::<BusStopResponse>(&bus_stop_code).await {
//         Ok(response) => {
//             let output = format_busstop_response(&response);
//             println!("{}", output);
//         }
//         Err(e) => {
//             println!("Error: {:?}", e);
//         }
//     }
// }
