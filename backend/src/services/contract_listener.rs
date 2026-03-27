use anyhow::Result;
use crate::services::alert_service::AlertService;
use crate::models::alert::{Alert, AlertChannel};
use std::sync::Arc;

pub struct ContractListener {
    pub alert_service: Arc<AlertService>,
}

impl ContractListener {
    pub fn new(alert_service: Arc<AlertService>) -> Self {
        Self { alert_service }
    }

    pub async fn on_contract_event(&self, event_type: &str, message: &str) -> Result<()> {
        tracing::info!("Contract event detected: {}", event_type);

        let alert = Alert {
            id: uuid::Uuid::new_v4().to_string(),
            title: format!("Contract Event: {}", event_type),
            message: message.to_string(),
            channel: AlertChannel::Email, // Default channel
        };

        // ✅ Fixed: Implemented actual alert sending
        self.alert_service.send_alert(alert).await?;

        Ok(())
    }
}