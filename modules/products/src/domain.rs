pub struct Product {
    pub id: i64,
    pub name: String,
    pub price: i64,
    pub description: String,
    pub category_id: i64,
}

pub struct ProductListItem {
    pub product: Product,
    pub category_name: String,
    pub primary_image_path: Option<String>,
}

pub struct ProductDetails {
    pub product: Product,
    pub images: Vec<ProductImage>,
}

pub struct ProductImage {
    pub id: i64,
    pub file_path: String,
    pub is_primary: bool,
    pub display_order: i32,
}

pub struct ProductFilter {
    pub search: Option<String>,
    pub category_id: Option<i64>,
    pub min_price: Option<i64>,
    pub max_price: Option<i64>,
}

pub struct Pagination {
    pub page: i64,
    pub page_size: i64,
}

impl Pagination {
    pub fn offset(&self) -> i64 {
        (self.page - 1).saturating_mul(self.page_size)
    }
}

pub struct ProductPage {
    pub items: Vec<ProductListItem>,
    pub total_items: i64,
}

pub struct ProductInput {
    pub name: String,
    pub price: i64,
    pub description: String,
    pub category_id: i64,
}
