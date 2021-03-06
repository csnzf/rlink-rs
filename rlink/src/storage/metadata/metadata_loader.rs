use crate::api::cluster::{ResponseCode, StdResponse};
use crate::runtime::ApplicationDescriptor;
use crate::utils::http_client::get_sync;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct MetadataLoader {
    coordinator_address: String,
    application_descriptor_cache: Option<ApplicationDescriptor>,
}

impl MetadataLoader {
    pub fn new(coordinator_address: &str) -> Self {
        MetadataLoader {
            coordinator_address: coordinator_address.to_string(),
            application_descriptor_cache: None,
        }
    }

    pub fn get_job_descriptor_from_cache(&mut self) -> ApplicationDescriptor {
        if let Some(a) = &self.application_descriptor_cache {
            a.clone()
        } else {
            self.get_application_descriptor()
        }
    }

    pub fn get_application_descriptor(&mut self) -> ApplicationDescriptor {
        let url = format!("{}/metadata", self.coordinator_address);
        loop {
            match get_sync(url.as_str()) {
                Ok(resp) => {
                    let resp_model: StdResponse<ApplicationDescriptor> =
                        serde_json::from_str(resp.as_str()).unwrap();
                    let StdResponse { code, data } = resp_model;
                    if code != ResponseCode::OK || data.is_none() {
                        panic!(
                            "get remote JobDescriptor with error code: ".to_owned() + resp.as_str()
                        );
                    }

                    let application_descriptor = data.unwrap();
                    self.application_descriptor_cache = Some(application_descriptor.clone());

                    return application_descriptor;
                }
                Err(e) => {
                    error!("get metadata(`JobDescriptor`) error. {}", e);
                    std::thread::sleep(std::time::Duration::from_secs(2));
                }
            }
        }
    }
}
