use super::{ItemPack, ItemType, LootPack};


pub(super) fn items(ip: &mut ItemPack) {
    ('a'..='z').chain('A'..='Z').for_each(|c| {
        ip.add_item(c.to_string(), ItemType::Letter(c as u8));
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