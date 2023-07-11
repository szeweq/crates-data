use std::borrow::Cow;
use super::{ItemType, Item, LootPack};

const ANIMALS: &[(&str, &str, usize)] = &[
    ("dog", "🐶", 9),
    ("cat", "🐱", 8),
    ("bunny", "🐰", 7),
    ("mouse", "🐭", 6),
    ("guineapig", "🐹", 4),
    ("bear", "🐻", 5),
    ("panda", "🐼", 7),
    ("koala", "🐨", 2),
    ("lion", "🦁", 5),
    ("tiger", "🐯", 4),
    ("fox", "🦊", 7),
    ("polarbear", "🐻‍❄️", 4),
    ("wolf", "🐺", 2),
    ("zebra", "🦓", 3),
    ("unicorn", "🦄", 1),
    ("horse", "🐴", 6),
    ("pig", "🐷", 9),
    ("sheep", "🐑", 6),
    ("llama", "🦙", 2)
];

pub(super) fn items(vec: &mut Vec<Item>) {
    ANIMALS.iter().for_each(|&(name, emoji, _)| {
        vec.push(Item {name: Cow::Borrowed(name), itype: ItemType::Emoji{emoji}});
    });
    vec.push(Item {name: Cow::Borrowed("knife"), itype: ItemType::Emoji{emoji: "🔪"}});
    vec.push(Item {name: Cow::Borrowed("pistol"), itype: ItemType::Emoji{emoji: "🔫"}});
}

pub(super) fn loots(lp: &mut LootPack) {
    let animv = ANIMALS.iter().map(|&(name, _, luck)| {
        (lp.item(name), luck)
    }).collect::<Vec<_>>();
    lp.add_loot("animals", animv);
    let rkv = vec![(lp.item("knife"), 1), (lp.item("pistol"), 99)];
    lp.add_loot("rare-knife", rkv);
}