//
use strum_macros::EnumIter;
use super::order::{OrderA, OrderB, OrderC};
use super::lists::{ListA, ListB};
use crate::overwrite::Order;
use strum_macros;
use std::string::ToString;
use std::fmt::Debug;

#[derive(EnumIter, Debug, Clone)]
pub enum ListIndicator {
    ListA(ListA),
    ListB(ListB),

}

#[derive(EnumIter, Debug, Clone)]
pub enum OrderIndicator {
    //#[strum(serialize = "orderA")]
    OrderA(OrderA),
    OrderB(OrderB),
    OrderC(OrderC),
}

#[derive(EnumIter, Debug, Clone, Hash, Eq, PartialEq)]
pub enum OrderKind {
    //#[strum(serialize = "orderA")]
    OrderA,
    OrderB,
    OrderC,
}



impl OrderKind {
    fn create_in0stance(self) -> T  {

        match self {
            OrderA => {

            },
            OrderB => {

            },
            OrderC => {

            }
        }
    }
}

// impl OrderIndicator {
//     fn create_instance(&self) -> impl Send + Order + 'static + Debug + Default {

//     }
// }