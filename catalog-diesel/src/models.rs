#[derive(Queryable, Serialize, Debug)]
pub struct Product {
    pub product_id: String,
    pub name: String,
    pub description: String,
    pub price: i32,
    pub count: i32,
    pub image_url:String,
}


pub struct Tag {
    pub tag_id: i32,
    pub name: String,
    pub display_name: String,
}
