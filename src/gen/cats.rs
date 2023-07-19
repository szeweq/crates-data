use super::{LootPack, EmoteSrc, ItemType, ItemPack};

const CATS: &[(&str, EmoteSrc)] = &[
    ("catJAM", EmoteSrc::B("5f1b0186cf6d2144653d2970")),
    ("catKISS", EmoteSrc::B("5f455410b2efd65d77e8cb14")),
    ("popCat", EmoteSrc::B("5fa8f232eca18f6455c2b2e1")),
    ("sadCat", EmoteSrc::B("5b96e7f1bbf4663f648795b1")),
    ("catDisco", EmoteSrc::B("5fd03d2acac2fb4621e5690a")),
    ("RAGEY", EmoteSrc::S("6309e73ffe72a7a37ff476f5")),
    ("uuh", EmoteSrc::S("645c29a0a93fbaf6fa2b5647")),
    ("plonk", EmoteSrc::S("63d2df40cd611db1717b420f")),
    ("Applecatrun", EmoteSrc::B("5f768d5fe016be4a882ee19a")),
    ("SadCatThumbsUp", EmoteSrc::S("60f701c931ba6ae6227ba6f8"))
];

const fn emote_cost(es: &EmoteSrc) -> usize {
    match es {
        EmoteSrc::B(_) => 2,
        EmoteSrc::S(_) => 3,
        _ => 1
    }
}

pub(super) fn items(ip: &mut ItemPack) {
    CATS.iter().for_each(|(name, src)| {
        ip.add_item(*name, src.clone(), emote_cost(src));
    })
}

pub(super) fn loots(lp: &mut LootPack) {
    let catv = CATS.iter().map(|(name, _)| {
        let it = lp.item(name.to_string());
        let luck: usize = if let ItemType::Emote(s) = &it.itype {
            emote_cost(s)
        } else { 1 };
        (it, luck)
    });
    lp.add_loot("cats", catv.collect());
}