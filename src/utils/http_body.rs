use serde::{Serialize,Deserialize};

#[derive(Deserialize,Serialize)]
pub struct MessageWithData<T> {
    pub msg: &'static str,
    pub data: T
}

#[derive(Deserialize,Serialize)]
pub struct Message {
    pub msg: &'static str
}