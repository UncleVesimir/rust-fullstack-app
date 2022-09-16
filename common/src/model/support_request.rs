use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};



pub enum SupportRequestState {
    Received,
    Reviewed,
    InProgress,
    Awaiting,
    Paused,
    Completed,
    Rejected
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SupportRequest {
    pub req_global_id: String,
    pub req_name: String,
    pub req_email: String,
    pub support_type: String,
    pub sup_message: String,
    pub state: SupportRequestState,
    pub date_created: DateTime<Utc>
}

impl SupportRequest {
    pub fn new(
        user_name: String,
        email: String,
        support_type: String,
        message: String,
    ) -> SupportRequest {
        SupportRequest { 
            req_global_id: Uuid::new_v4(),
            req_name: user_name,
            req_email: email,
            support_type: support_type,
            sup_message: message,
            state: SupportRequestState::Received,
            date_created: Utc::now(),
        }
    }

    pub fn create_global_id(&self) {
        self.request_global_id = Uuid::new_v4();
    }

    pub fn get_global_id(&self) -> &String {
        &self.request_global_id
    }
}
