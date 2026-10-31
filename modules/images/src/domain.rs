pub struct Image {
    pub id: i64,
    pub product_id: i64,
    pub file_path: String,
    pub is_primary: bool,
    pub display_order: i32,
}

pub struct ImageInput {
    pub file_path: String,
    pub is_primary: bool,
    pub display_order: i32,
}
