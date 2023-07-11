use std::{collections::HashMap, rc::Rc, borrow::Cow};
use serde::Serialize;

mod emoji;
mod letters;
mod peepo;

pub type Name = Cow<'static, str>;

pub struct PackGen {
    pub items: Vec<Box<dyn ItemCreator>>,
    pub loots: Vec<Box<dyn LootCreator>>
}
pub trait ItemCreator {
    fn add_items(&self, vec: &mut Vec<Item>);
}
impl<T: Fn(&mut Vec<Item>)> ItemCreator for T {
    fn add_items(&self, vec: &mut Vec<Item>) { (self)(vec) }
}
pub trait LootCreator {
    fn add_loots(&self, lp: &mut LootPack);
}
impl<T: Fn(&mut LootPack)> LootCreator for T {
    fn add_loots(&self, lp: &mut LootPack) { (self)(lp) }
}

pub struct Item {
    pub name: Name,
    pub itype: ItemType
}
pub struct Loot {
    pub name: Name,
    pub items: Vec<(Rc<Item>, usize)>
}

pub struct LootPack {
    pub items: HashMap<Name, Rc<Item>>,
    pub loots: Vec<Loot>
}
impl LootPack {
    pub fn item(&self, name: impl Into<Name>) -> Rc<Item> {
        self.items[&name.into()].clone()
    }
    pub fn add_loot(&mut self, name: impl Into<Name>, items: Vec<(Rc<Item>, usize)>) {
        self.loots.push(Loot{name: name.into(), items});
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ItemType {
    Emoji{ emoji: &'static str }, Emote(EmoteSrc), Letter(u8)
}
impl ItemType {
    pub fn group_name(&self) -> &'static str {
        match self {
            ItemType::Emoji{..} => "emoji",
            ItemType::Emote(_) => "emote",
            ItemType::Letter(_) => "letter"
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(tag = "src", content = "id", rename_all = "camelCase")]
pub enum EmoteSrc {
    F(&'static str), B(&'static str), S(&'static str)
}

macro_rules! fns_from_mods {
    ($($mod:ident)*) => { PackGen {
        items: vec![$(Box::new($mod::items)),*],
        loots: vec![$(Box::new($mod::loots)),*]
    } }
}

pub fn get_packgen() -> PackGen {
    fns_from_mods!(emoji letters peepo)
}