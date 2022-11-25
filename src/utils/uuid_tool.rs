use uuid::Uuid;

pub fn generate_uuid() -> String {
    let id = Uuid::new_v4();
    id.to_string()
}
