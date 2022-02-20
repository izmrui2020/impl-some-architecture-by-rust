//
use strum_macros::EnumIter;
use tokio::net::tcp::ReuniteError;
use super::order::{OrderA, OrderB, OrderC};
use super::lists::{ListA, ListB};
use crate::overwrite::Order;
use strum_macros;
use anyhow::Result;
use super::order;
use std::string::ToString;
use std::fmt::{Debug};

#[derive(EnumIter, Debug, Clone)]
pub enum ListIndicator {
    ListA(ListA),
    ListB(ListB),
}

#[derive(EnumIter, Debug, Clone)]
pub enum OrderIndicator {
    //#[strum(serialize = "orderA")]
    A(OrderA),
    B(OrderB),
    C(OrderC),
}

//https://qiita.com/carrotflakes/items/896ce7f49931c64a2954

use async_recursion::async_recursion;
impl OrderIndicator {
    #[async_recursion]
    pub async fn sort(&mut self) -> Result<()> {
        //println!("call sort by {:?}", self);

        use OrderIndicator::*;

        match self {
            A(order) => { 
                println!("call OrderA sort() method");
                return order.sort().await;
            },
            B(order) => {
                println!("call OrderB sort() method");
                return order.sort().await;
            },
            C(order) => {
                println!("call OrderC sort() method");
                return order.sort().await;
            }
        };

    }
}

#[derive(EnumIter, Debug, Clone, Hash, Eq, PartialEq)]
pub enum OrderKind {
    //#[strum(serialize = "orderA")]
    OrderkindA,
    OrderkindB,
    OrderkindC,
}



impl OrderKind {
    pub fn create_instance(&self) -> OrderIndicator {

        use OrderKind::*;

        match self {
            OrderkindA => {
                OrderIndicator::A(OrderA::default())
            },
            OrderkindB => {
                OrderIndicator::B(OrderB::default())
            },
            OrderkindC => {
                OrderIndicator::C(OrderC::default())
            }
        }
    }
}
