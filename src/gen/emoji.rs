use super::{ItemType, LootPack, ItemPack, Emoji};

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
    ("llama", "🦙", 2),
    ("cow", "🐮", 8),
    ("duck", "🦆", 6),
    ("penguin", "🐧", 4),
    ("flybug", "🪰", 8),
];

pub(super) fn items(ip: &mut ItemPack) {
    ANIMALS.iter().for_each(|&(name, emoji, _)| {
        ip.add_item(name, Emoji(emoji));
    });
    ip.add_item("knife", Emoji("🔪"));
    ip.add_item("pistol", Emoji("🔫"));
    ip.add_item("glove", Emoji("🧤"));
}

pub(super) fn loots(lp: &mut LootPack) {
    let animv = ANIMALS.iter().map(|&(name, _, luck)| {
        (lp.item(name), luck)
    }).collect::<Vec<_>>();
    lp.add_loot("animals", animv);
    lp.add_loot("rare-knife", vec![(lp.item("knife"), 1), (lp.item("pistol"), 99)]);
    lp.add_loot("rare-gloves", vec![(lp.item("knife"), 5), (lp.item("glove"), 95)]);
}