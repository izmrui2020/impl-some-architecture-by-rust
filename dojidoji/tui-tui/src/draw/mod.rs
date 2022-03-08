//
pub mod draw;
use strum::ToString;
use strum_macros::{self, EnumIter};

#[derive(strum_macros::ToString, Copy, Clone, Debug)]
pub enum HomeItem {
    Home,
    Ranking,
    Select,
}

impl From<HomeItem> for usize {
    fn from(input: HomeItem) -> usize {
        match input {
            HomeItem::Home => 0,
            HomeItem::Ranking => 1,
            HomeItem::Select => 2,
        }
    }
}

#[derive(EnumIter, strum_macros::ToString, PartialEq, Eq, Debug, Clone, Copy)]
pub enum MenuItem {
    Item1,
    Item2,
    Item3,
    Item4,
    Item5,
    Item6,
    Item7,
    Item8,
    Item9,
    Item10,
}

impl From<MenuItem> for usize {
    fn from(input: MenuItem) -> usize {
        match input {
            MenuItem::Item1 => 1,
            MenuItem::Item2 => 2,
            MenuItem::Item3 => 3,
            MenuItem::Item4 => 4,
            MenuItem::Item5 => 5,
            MenuItem::Item6 => 6,
            MenuItem::Item7 => 7,
            MenuItem::Item8 => 8,
            MenuItem::Item9 => 9,
            MenuItem::Item10 => 10,
        }
    }
}