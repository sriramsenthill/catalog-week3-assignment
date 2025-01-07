// utils/sort_stage.rs
use bson::{doc, Document};

pub fn build_sort_stage(sort_by: &Option<String>, order: &Option<String>) -> Option<Document> {
    sort_by.as_ref().map(|sort_by| {
        let order = if order.as_deref() == Some("desc") {
            -1
        } else {
            1
        };
        doc! { "$sort": { sort_by.clone(): order } }
    })
}
