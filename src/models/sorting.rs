pub struct Sorting {
    pub sort: Option<String>,
    pub order: Option<String>,
}

pub enum SortField {
    Name,
    Price,
    CreatedAt,
}

pub enum SortOrder {
    Asc,
    Desc,
}

//Sorting examples
impl Sorting {
    pub fn normalize(self) -> (SortField, SortOrder) {
        let sort_field = match self.sort.as_deref() {
            Some("price") => SortField::Price,
            Some("created_at") => SortField::CreatedAt,
            _ => SortField::Name,
        };

        let order = match self.order.as_deref() {
            Some("desc") => SortOrder::Desc,
            _ => SortOrder::Asc,
        };

        (sort_field, order)
    }
}

//Sorting in SQL example
// pub async fn list_products(&self, 
//     page: u32, 
//     per_page: u32, 
//     sort: SortField, 
//     order: SortOrder) {
//         let offset = (page - 1) * per_page;

//         let order_clause = match(sort, order) {
//             (SortField::Name, SortOrder::Asc) => "ORDER BY name ASC",
//             (SortField::Name, SortOrder::Desc) => "ORDER BY name DESC",
//             (SortField::Price, SortOrder::Asc) => "ORDER BY price ASC",
//             (SortField::Price, SortOrder::Desc) => "ORDER BY name DESC",
//             (SortField::CreatedAt, SortOrder::Asc) => "ORDER BY created_at ASC",
//             (SortField::CreatedAt, SortOrder::Desc) => "ORDER BY created_at DESC",
//         };
//         let query = format!(
//             "SELECT id, name, price, description FROM products {} LIMIT $1 OFFSET $2", order_clause
//         );

//         let rows = sqlx::query_as(&query)
//         .bind(per_page as i64)
//         .bind(offset as i64)
//         .fetch_all(&self.pool)
//         .await?;

//         Ok(rows.into_iter().map(Product::from).collect())
//     }