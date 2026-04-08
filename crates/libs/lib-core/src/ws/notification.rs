use serde::{de::Error as DeError, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use super::error::Result;

#[derive(Debug, Clone)]
pub struct WsNotification<P = Value> {
    pub method: String,
    pub params: Option<P>,
}

impl<P: Serialize> WsNotification<P> {
    pub fn stringify(&self) -> Result<String> {
        Ok(serde_json::to_string(&self)?)
    }
    
    pub fn to_json_value(&self) -> Result<Value> {
        Ok(serde_json::to_value(self)?)
    }
}

pub trait IntoWsNotification: Sized {
    const METHOD: &'static str;

    fn into_ws_notification(self) -> WsNotification<Self> {
        WsNotification {
            method: Self::METHOD.to_string(),
            params: Some(self),
        }
    }
}

impl<T: IntoWsNotification> From<T> for WsNotification<T> {
    fn from(params: T) -> Self {
        WsNotification {
            method: T::METHOD.to_string(),
            params: Some(params),
        }
    }
}

impl<P> Serialize for WsNotification<P>
where
    P: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let params_value = match &self.params {
            Some(p) => Some(serde_json::to_value(p).map_err(serde::ser::Error::custom)?),
            None => None,
        };

        let rpc_notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": self.method,
            "params": params_value
        });

        rpc_notification.serialize(serializer)
    }
}

impl<'de, P> Deserialize<'de> for WsNotification<P>
where
    P: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Value::deserialize(deserializer)?;
        
        let rpc_notification = value.as_object().ok_or_else(|| {
            DeError::custom("expected JSON object")
        })?;
        
        let method = rpc_notification
            .get("method")
            .and_then(|v| v.as_str())
            .ok_or_else(|| DeError::custom("missing method field"))?
            .to_string();
        
        let params = match rpc_notification.get("params") {
            Some(params_value) => {
                let p = P::deserialize(params_value.clone()).map_err(DeError::custom)?;
                Some(p)
            }
            None => None,
        };
        
        Ok(WsNotification { method, params })
    }
}