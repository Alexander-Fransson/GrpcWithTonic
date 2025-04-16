use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub _user_id: Uuid
}

impl RequestContext {
    pub fn new(_user_id: Uuid) -> Self {
        Self {
            _user_id
        }
    }
}