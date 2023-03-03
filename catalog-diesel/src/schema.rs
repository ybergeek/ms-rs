// @generated automatically by Diesel CLI.

diesel::table! {
    product (product_id) {
        product_id -> Varchar,
        name -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
        price -> Nullable<Int4>,
        count -> Nullable<Int4>,
        image_url -> Nullable<Varchar>,
    }
}

diesel::table! {
    product_tag (product_id, tag_id) {
        product_id -> Varchar,
        tag_id -> Int4,
    }
}

diesel::table! {
    tag (tag_id) {
        tag_id -> Int4,
        name -> Nullable<Varchar>,
        display_name -> Nullable<Varchar>,
    }
}

diesel::joinable!(product_tag -> product (product_id));
diesel::joinable!(product_tag -> tag (tag_id));

diesel::allow_tables_to_appear_in_same_query!(
    product,
    product_tag,
    tag,
);
