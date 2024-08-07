use anyhow::anyhow;
use log::{debug, info, warn};
use tide::{prelude::*, Request, Response};

const PORT: u16 = 23032;
const OPENSHOCK_ENDPOINT: &str = "https://api.openshock.app/2/shockers/control";

#[derive(Debug, Deserialize)]
struct PiShockApiRequest {
    #[serde(rename = "Username")]   _username: String,
    #[serde(rename = "Name")]       name: String,
    #[serde(rename = "Code")]       code: String,
    #[serde(rename = "Apikey")]     apikey: String,

    #[serde(rename = "Intensity")]  intensity: Option<u32>,
    #[serde(rename = "Duration")]   duration: u32,
    #[serde(rename = "Op")]         operation: u32,
}

#[derive(Debug, Serialize)]
struct OpenShockControlRequest {
    id: String,
    #[serde(rename = "type")] 
    control_type: String,
    intensity: u32,
    duration: u32,
    exclusive: bool,
}

#[derive(Debug, Serialize)]
struct OpenShockApiRequest {
    shocks: Vec<OpenShockControlRequest>,
    #[serde(rename = "customName")] 
    custom_name: String,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    env_logger::init();

    let mut app = tide::new();
    app.at("/api/apioperate").post(apioperate);

    app.listen(format!("0.0.0.0:{}", PORT)).await?;

    Ok(())
}

async fn apioperate(mut req: Request<()>) -> tide::Result {
    info!(
        "pishock api request from {}", 
        req.peer_addr().unwrap_or("unknown address"),
    );

    let pishock_api_request: PiShockApiRequest = req.body_json().await?;

    let (openshock_request, openshock_token) 
        = convert_request(pishock_api_request).await?;

    debug!("openshock req: {}", serde_json::to_string(&openshock_request)?);

    let mut openshock_response = surf::post(OPENSHOCK_ENDPOINT)
        .header("OpenShockToken", openshock_token)
        .body_json(&openshock_request)?
        .send()
        .await?;

    let openshock_body = openshock_response.body_string().await?;

    debug!(
        "openshock res: {} {}", 
        openshock_response.status(), 
        openshock_body,
    );

    if openshock_response.status() != 200 {
        warn!("error from openshock in handling client");
        Ok(Response::builder(200).body(openshock_body).build())
    } else {
        info!("successfully converted and sent api request from client");
        Ok("Operation Succeeded.".into())
    }
}

async fn convert_request(
    pishock_request: PiShockApiRequest,
) -> tide::Result<(OpenShockApiRequest, String)> {

    let openshock_request = OpenShockApiRequest {
        shocks: vec![
            OpenShockControlRequest {
                id: pishock_request.code,
                control_type: match pishock_request.operation {
                    0 => "Shock".to_string(),
                    1 => "Vibrate".to_string(),
                    2 => "Sound".to_string(),
                    _ => return Err(tide::Error::new(
                        422, 
                        anyhow!("invalid operation"),
                    )),
                },
                intensity: pishock_request.intensity.unwrap_or(0),
                duration: pishock_request.duration
                    .checked_mul(1000)
                    .ok_or(tide::Error::new(
                        422,
                        anyhow!("integer overflow in duration"),
                    ))?,
                exclusive: true,
            },
        ],
        custom_name: pishock_request.name,
    };

    Ok((
        openshock_request,
        pishock_request.apikey,
    ))
}
