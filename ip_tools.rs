use serde::{Deserialize, Serialize};

pub async fn get_ip_info(ip: String) -> String
{
    let ip = format!("http://ip-api.com/json/{}", ip);
    let resp = reqwest::get(&ip).await.unwrap().text().await;
    match resp {
        Ok(ok) => {
            let jsonmsg: Message = serde_json::from_str(&ok).unwrap();
            match jsonmsg {
                 Message::Success(success) => success.get_string(),
                 Message::Fail(fail) => fail.message,
            }
        }
        Err(error) => error.to_string()
    }
}
impl IPInfo 
{
    fn get_string(&self) -> String
    {
        let mut result = String::new();
        result.push_str("Country: ");
        result.push_str(&self.country);
        result.push_str("\r\n");
        result.push_str("Region: ");
        result.push_str(&self.region);
        result.push_str("\r\n");
        result.push_str("City: ");
        result.push_str(&self.city);
        result.push_str("\r\n");
        result.push_str("ISP: ");
        result.push_str(&self.isp);
        result.push_str("\r\n");
        result.push_str("Organization: ");
        result.push_str(&self.org);
        result.push_str("\r\n");
        result.push_str("ASN: ");
        result.push_str(&self.asn);
        result
    }
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IPInfo {
    status: String,
    country: String,
    country_code: String,
    region: String,
    region_name: String,
    city: String,
    zip: String,
    lat: f64,
    lon: f64,
    timezone: String,
    isp: String,
    org: String,
    #[serde(rename = "as")]
    asn: String,
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Fail {
    status: String,
    message: String,
    query: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[serde(untagged)] // WHY DOESN'T IT WORK IF I DO tag = "status" WHY DOES THIS WORK THEN ITS THE SAME THING
enum Message {
    Success(IPInfo),
    Fail(Fail)
}