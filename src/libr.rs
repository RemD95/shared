//! Shared DTOs between server and client.
//! Keep this crate minimal: only types and small helpers.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

/// Repr of an item stored by the server.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Item {
    pub id: Uuid,
    pub name: String,
    pub value: i64,
    pub created_at: DateTime<Utc>,
}

/// Request body for creating an item.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct CreateItemRequest {
    pub name: String,
    pub value: i64,
}

/// Request body for updating an item (partial).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct UpdateItemRequest {
    pub name: Option<String>,
    pub value: Option<i64>,
}

/// Standard error response shape returned by server on errors.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct ErrorResponse {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Request the client can send to `/proxy`.
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyRequest {
    pub url: String,
    #[serde(default)]
    pub method: Option<String>, // e.g. "GET", "POST"
    #[serde(default)]
    pub body: Option<Value>,
}

/// Response shape returned by `/proxy` (simplified).
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ProxyResponse {
    pub status: u16,
    #[serde(default)]
    pub body: Option<Value>,
    #[serde(default)]
    pub headers: Option<Vec<(String, String)>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use serde_json::json;

    #[test]
    fn item_serde_roundtrip() {
        let item = Item {
            id: Uuid::new_v4(),
            name: "test".to_string(),
            value: 123,
            created_at: Utc::now(),
        };
        let s = serde_json::to_string(&item).expect("serialize");
        let de: Item = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(item.id, de.id);
        assert_eq!(item.name, de.name);
        assert_eq!(item.value, de.value);
    }

    #[test]
    fn proxy_request_serde() {
        let req = ProxyRequest {
            url: "https://httpbin.org/get".into(),
            method: Some("GET".into()),
            body: None,
        };
        let s = serde_json::to_string(&req).expect("serialize");
        let de: ProxyRequest = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de.method.unwrap(), "GET");
    }

    #[test]
    fn error_response_serializes_code_optional() {
        let e = ErrorResponse {
            message: "oops".into(),
            code: None,
        };
        let s = serde_json::to_string(&e).expect("serialize");
        assert!(s.contains("oops"));
        // code not present
        assert!(!s.contains("code"));
    }

    #[test]
    fn proxy_response_with_body_roundtrip() {
        let pr = ProxyResponse {
            status: 200,
            body: Some(json!({"ok": true})),
            headers: Some(vec![("content-type".into(), "application/json".into())]),
        };
        let s = serde_json::to_string(&pr).expect("serialize");
        let de: ProxyResponse = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(de.status, 200);
    }
}
