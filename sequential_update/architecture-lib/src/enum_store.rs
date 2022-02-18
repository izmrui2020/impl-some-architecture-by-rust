//
use strum_macros::EnumIter;
use super::order::{OrderA, OrderB, OrderC};
// use super::lists::{ListA, ListB};

// #[derive(EnumIter, Debug, Clone)]
// pub enum ListIndicator {
//     ListA(ListA),
//     ListB(ListB),

// }

#[derive(EnumIter, Debug, Clone)]
pub enum OrderIndicator {
    OrderA(OrderA),
    OrderB(OrderB),
    OrderC(OrderC),
}