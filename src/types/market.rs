use std::error::Error;
use std::hash::{Hash, Hasher};

use crate::types::atomic::{Order, Boards, Execution};

pub trait Market {
    fn unique_id(&self) -> String;

    // Immutable
    fn boards(&self) -> Result<Boards, Box<Error>>;
    fn executions(&self) -> Result<Vec<Execution>, Box<Error>>;
    fn orders(&self) -> Result<Vec<Order>, Box<Error>>;

    // Mutable
    fn send_order(&self, order: Order) -> Result<Order, Box<Error>>;
    fn cancel_order(&self, order: Order) -> Result<Order, Box<Error>>;
}
