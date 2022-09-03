use serde::Deserialize;

#[derive(Copy, Clone, Debug, PartialEq, Deserialize)]
pub enum ItemType {
    Equipment,
    Usable,
    Amulet,
}