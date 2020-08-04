use serde::{Deserialize};

#[derive(Deserialize)]
pub struct QueryPagination{
    page: Option<i64>,
    limit: Option<i64>,
    order: Option<String>
}

impl QueryPagination{
    pub fn get_page(&self)->i64{
        match self.page {
            Some(page) => page,
            None => 1i64
        }
    }
    pub fn get_limit(&self)->i64{
        match self.limit {
            Some(limit) => limit,
            None => 20i64
        }
    }
    pub fn get_order(&self)->String{
        match &self.order {
            Some(order) => {
                let input = order.to_uppercase();
                if input != "ASC" || input != "DESC" { return "DESC".to_string(); }
                return order.to_string();
            },
            None => {
                return "DESC".to_string();
            }
        };
    }
    pub fn get_offset(&self)->i64{
        (self.get_page() - 1i64)*self.get_limit()
    }
}