use std::borrow::Cow;

use super::{Item, ItemType, LootPack};


pub(super) fn items(vec: &mut Vec<Item>) {
    ('a'..='z').chain('A'..='Z').for_each(|c| {
        vec.push(Item {name: Cow::Owned(c.to_string()), itype: ItemType::Letter(c as u8) });
    });
}
pub(super) fn loots(lp: &mut LootPack) {
    let lowercase = ('a'..='z')
        .map(|c| (lp.item(c.to_string()), 1))
        .collect();
    lp.add_loot("lowercase-letters", lowercase);

    let uppercase = ('A'..='Z')
        .map(|c| (lp.item(c.to_string()), 1))
        .collect();
    lp.add_loot("uppercase-letters", uppercase);
}