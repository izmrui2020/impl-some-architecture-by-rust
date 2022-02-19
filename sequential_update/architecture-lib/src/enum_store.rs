//
use strum_macros::EnumIter;
use super::order::{OrderA, OrderB, OrderC};
use super::lists::{ListA, ListB};
use crate::overwrite::Order;
use std::fmt::Debug;

#[derive(EnumIter, Debug, Clone)]
pub enum ListIndicator {
    ListA(ListA),
    ListB(ListB),

}

#[derive(EnumIter, Debug, Clone)]
pub enum OrderIndicator {
    OrderA(OrderA),
    OrderB(OrderB),
    OrderC(OrderC),
}

// impl OrderIndicator {
//     fn create_instance(&self) -> impl Send + Order + 'static + Debug + Default {

//     }
// }