use crate::lightning_fetcher::LightningResponse;
use chrono::DateTime;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NodeDocument {
    #[serde(rename = "public_key")]
    pub public_key: String,
    pub alias: String,
    pub capacity: String,
    #[serde(rename = "first_seen")]
    pub first_seen: String,
}

impl NodeDocument {
    pub fn from_response(response: LightningResponse) -> Result<Self, String> {
        let unix_time = DateTime::from_timestamp(response.first_seen, 0);

        if unix_time.is_none() {
            return Err(format!(
                "The first_seen time attribute could not be built because for {}",
                response.public_key
            ));
        }

        if response.capacity <= 0 {
            return Err(format!(
                "For unknown reason the capacity of the node {} is {} wich is weird",
                response.public_key, response.capacity
            ));
        }

        let capacity = Decimal::new(response.capacity, 0);
        let capacity = capacity / Decimal::new(100_000_000, 8);
        // We're we have a strange thing:
        // In the "Endpoint para exposição das informações"
        // The response is formatted with dots
        // In the "Requisitos extras" section
        // The response is formatted with comma
        let capacity = capacity.to_string();

        Ok(NodeDocument {
            alias: response.alias,
            capacity: capacity,
            first_seen: unix_time.unwrap().to_rfc3339(),
            public_key: response.public_key,
        })
    }
}

impl TryFrom<LightningResponse> for NodeDocument {
    type Error = String;

    fn try_from(value: LightningResponse) -> Result<Self, Self::Error> {
        NodeDocument::from_response(value)
    }
}

#[cfg(test)]
mod tests {
    use crate::lightning_fetcher::LightningResponse;

    use super::NodeDocument;

    #[test]
    fn it_parses_as_expected()
    {
        let response = LightningResponse {
            capacity: 550_000,
            first_seen: 1727278230,
            ..Default::default()
        };

        let parsed = NodeDocument::try_from(response);
        assert_eq!(parsed.is_ok(), true);
        let parsed = parsed.unwrap();
        assert_eq!(&parsed.first_seen, "2024-09-25T15:30:30+00:00");
        assert_eq!(&parsed.capacity, "0.00550000")
    }
}