use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ContainerId(String);

// TODO: need to validate the fields before processing
// Plan -> ExecutionPlan -> Execution

#[derive(Debug, Deserialize)]
/// It is used to Deserialize requests
pub struct Plan {
    #[serde(skip_serializing_if = "is_default")]
    /// Create a docker container
    pub create: Option<CreateContainer>,
    #[serde(skip_serializing_if = "is_default")]
    /// Get details of a container
    pub info: Option<Info>,
    #[serde(skip_serializing_if = "is_default")]
    /// Modify a container
    pub modify: Option<Modify>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateContainer {
    /// Docker compose file
    #[serde(skip_serializing_if = "is_default")]
    docker_compose: Option<String>,
    /// Auto Start
    #[serde(skip_serializing_if = "is_default")]
    auto_start: Option<usize>,
    /// Webhook URL
    #[serde(skip_serializing_if = "is_default")]
    webhook_url: Option<String>,
    /// Volumes
    /// List of URLs to the directories that should be mounted as volumes
    #[serde(skip_serializing_if = "is_default")]
    volumes: Vec<Volume>,
}

#[derive(Debug, Deserialize)]
pub struct Volume {
    /// URL to the directory that should be mounted as a volume
    pub url: String,
    /// Path in the container where the volume should be mounted
    pub file_name: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    /// Container ID
    pub container_id: ContainerId,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Modify {
    /// Container ID
    pub container_id: ContainerId,
}
