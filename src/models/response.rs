use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
  pub message: String,
  pub data: Value
}

#[derive(Debug)]
pub struct ResponseWithStatus {
  pub status_code: u16,
  pub response: Response,
}