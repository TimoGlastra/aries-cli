use crate::error::Result;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct CredentialDefinitionCreateResponse {
    pub credential_definition_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinitionGetByIdResponse {
    #[serde(rename = "credential_definition")]
    pub credential_definition: CredentialDefinition,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CredentialDefinition {
    pub ver: String,
    pub id: String,
    pub schema_id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub tag: String,
    pub value: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CredentialDefinitionGetAllResponse {
    pub credential_definition_ids: Vec<String>,
}

#[async_trait]
pub trait CredentialDefinitionModule {
    /// Requests all the features from the cloudagent
    async fn create(&self, schema_id: String) -> Result<CredentialDefinitionCreateResponse>;
    async fn get_by_id(&self, id: String) -> Result<CredentialDefinitionGetByIdResponse>;
    async fn get_all(&self) -> Result<CredentialDefinitionGetAllResponse>;
}
