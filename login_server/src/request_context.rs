use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct RequestContext {
    user_id: Uuid
}

impl RequestContext {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            user_id
        }
    }
}