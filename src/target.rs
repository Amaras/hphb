use crate::character::{Hero, Villain};
#[derive(Debug, PartialEq, Clone)]
pub enum Target {
    Hero(Hero),
    Villain(Villain),
    Location,
}
