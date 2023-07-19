use std::{collections::HashMap, rc::Rc, borrow::Cow};
use serde::Serialize;

mod emoji;
mod letters;
mod peepo;
mod cats;

pub type Name = Cow<'static, str>;

#[macro_export]
macro_rules! lucks {
    {$($name:literal: $luck:literal),+} => {[$(($name, $luck)),+]};
}

pub struct PackGen {
    pub items: Vec<Box<dyn ItemCreator>>,
    pub loots: Vec<Box<dyn LootCreator>>
}
pub trait ItemCreator {
    fn add_items(&self, ip: &mut ItemPack);
}
impl<T: Fn(&mut ItemPack)> ItemCreator for T {
    fn add_items(&self, ip: &mut ItemPack) { (self)(ip) }
}
pub trait LootCreator {
    fn add_loots(&self, lp: &mut LootPack);
}
impl<T: Fn(&mut LootPack)> LootCreator for T {
    fn add_loots(&self, lp: &mut LootPack) { (self)(lp) }
}

pub struct Item {
    pub name: Name,
    pub itype: ItemType,
    pub cost: usize
}
pub struct Loot {
    pub name: Name,
    pub items: Vec<(Rc<Item>, usize)>
}

pub struct ItemPack {
    pub items: Vec<Item>
}
impl ItemPack {
    pub fn add_item(&mut self, name: impl Into<Name>, itype: impl Into<ItemType>, cost: usize) {
        self.items.push(Item{name: name.into(), itype: itype.into(), cost});
    }
}

pub struct LootPack {
    pub items: HashMap<Name, Rc<Item>>,
    pub loots: Vec<Loot>
}
impl LootPack {
    pub fn item(&self, name: impl Into<Name>) -> Rc<Item> {
        let sname = name.into();
        self.items.get(&sname).unwrap_or_else(|| panic!("Item not found: {}", sname)).clone()
    }
    pub fn add_loot(&mut self, name: impl Into<Name>, items: Vec<(Rc<Item>, usize)>) {
        self.loots.push(Loot{name: name.into(), items});
    }
    pub fn add_loot_names(&mut self, name: impl Into<Name>, items: impl IntoIterator<Item=(impl Into<Name>, usize)>) {
        self.loots.push(Loot{
            name: name.into(),
            items: items.into_iter().map(|(name, count)| (self.item(name), count)).collect()
        });
    }
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum ItemType {
    Emoji(Emoji), Emote(EmoteSrc), Letter(u8), Steam(SteamId), YTVideo(YouTubeVideo)
}
impl ItemType {
    pub const fn group_name(&self) -> &'static str {
        match self {
            Self::Emoji(_) => "emoji",
            Self::Emote(_) => "emote",
            Self::Letter(_) => "letter",
            Self::Steam(_) => "steam",
            Self::YTVideo(_) => "ytvideo"
        }
    }
}
impl From<EmoteSrc> for ItemType {
    fn from(src: EmoteSrc) -> Self {
        Self::Emote(src)
    }
}
impl From<Emoji> for ItemType {
    fn from(emoji: Emoji) -> Self {
        Self::Emoji(emoji)
    }
}
impl From<SteamId> for ItemType {
    fn from(id: SteamId) -> Self {
        Self::Steam(id)
    }
}

#[derive(Serialize, Clone)]
#[serde(tag = "src", content = "id", rename_all = "camelCase")]
pub enum EmoteSrc {
    F(&'static str), B(&'static str), S(&'static str)
}

#[derive(Serialize)]
pub struct Emoji(&'static str);

#[derive(Serialize)]
pub struct SteamId { id: u64 }

#[derive(Serialize)]
pub struct YouTubeVideo { id: &'static str }

macro_rules! fns_from_mods {
    ($($mod:ident)*) => { PackGen {
        items: vec![$(Box::new($mod::items)),*],
        loots: vec![$(Box::new($mod::loots)),*]
    } }
}

pub fn get_packgen() -> PackGen {
    fns_from_mods!(emoji letters peepo cats)
}