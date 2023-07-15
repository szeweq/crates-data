
use super::{LootPack, EmoteSrc, ItemType, ItemPack};

const PEEPOS: &[(&str, EmoteSrc)] = &[
    ("peepoArrive", EmoteSrc::B("61afb90b002cdeedc21e9b73")),
    ("peepoBlanket", EmoteSrc::F("262458")),
    ("peepoBlonket", EmoteSrc::B("5b8015ba3c482e0aaa9413be")),
    ("peepoBONK", EmoteSrc::B("5e0c1323031ec77bab4738fb")),
    ("peepoBox", EmoteSrc::B("5b9a383b6c095b77aed377e1")),
    ("peepoBye", EmoteSrc::B("5eef252af91de70dea5baedd")),
    ("peepoChat", EmoteSrc::B("5e1bd08688e62a5f14dc6316")),
    ("peepoClap", EmoteSrc::B("5d38aaa592fc550c2d5996b8")),
    ("peepoClown", EmoteSrc::F("318914")),
    ("PeepoComfy", EmoteSrc::B("5e0502e69e2cd00d968d5677")),
    ("peepoDJ", EmoteSrc::S("6102a37ba57eeb23c0e3e5cb")),
    ("peepoFat", EmoteSrc::F("288800")),
    ("peepoFAT", EmoteSrc::S("60b01e91ad7fb4b50b3a3eaf")),
    ("peepoFinger", EmoteSrc::B("5dac6b0b89488d12cc727354")),
    ("peepoGiggle", EmoteSrc::B("5e1eb57cbca2995f13fb7ce7")),
    ("peepoGiggles", EmoteSrc::B("5e0bcf69031ec77bab473476")),
    ("PeepoGlad", EmoteSrc::F("244322")),
    ("peepoGulag", EmoteSrc::B("5e774ccfd112fc372574d6a1")),
    ("PeepoHappy", EmoteSrc::B("5df0af0e91129e77b47ccadb")),
    ("peepoHawOnAGoose", EmoteSrc::B("5f16cb6ffe85fb4472d0f975")),
    ("peepoHey", EmoteSrc::B("5e162859b640b52102c684f7")),
    ("peepoHug", EmoteSrc::B("5a1702fb8c22a247ead62d95")),
    ("peepoJAM", EmoteSrc::B("5bc3d95236e2661664652806")),
    ("peepoJAMMER", EmoteSrc::B("5bfc4331cd0d1566313b82dd")),
    ("PeepoJuice", EmoteSrc::B("5deaecf4515a2a77bc9e94ab")),
    ("peepoLeave", EmoteSrc::B("61afbb1e002cdeedc21e9bc6")),
    ("peepoLeaveFinger", EmoteSrc::B("5e031d608608fb0da4122461")),
    ("peepoLove", EmoteSrc::F("250614")),
    ("PeepoNoob", EmoteSrc::B("5e8b86328fb1ca5cde5866b5")),
    ("peepoPat", EmoteSrc::B("5c5213e59af3bb75410e87bf")),
    ("peepoPooPoo", EmoteSrc::B("5c3427a55752683d16e409d1")),
    ("peepoRain", EmoteSrc::B("5c6b37bdadab351034b40dc5")),
    ("peepoRiot", EmoteSrc::S("60af0382b38361ea91337096")),
    ("peepoRun", EmoteSrc::B("5bc7ff14664a3b079648dd66")),
    ("PeepoRunCry", EmoteSrc::B("5d8350251df66f68c80c1964")),
    ("peepoS", EmoteSrc::B("5a26924bfc6e584787d98544")),
    ("peepoSad", EmoteSrc::F("230082")),
    ("peepoSHAKE", EmoteSrc::B("5b83938ca69b8634bf059473")),
    ("peepoShortOnAGoose", EmoteSrc::B("5efd70646a65270522157cae")),
    ("peepoShrug", EmoteSrc::B("5a2691c6fc6e584787d98534")),
    ("peepoShut", EmoteSrc::B("5e12e38d3267f72103fd6b15")),
    ("peepoShy", EmoteSrc::S("638767f24cc489ef45239272")),
    ("peepoSimp", EmoteSrc::B("5e49a482d736527d5cd30cdd")),
    ("peepoSmash", EmoteSrc::B("5c72084d41600b0832ab0931")),
    ("peepoSnow", EmoteSrc::B("5f91c1486f583802e38974aa")),
    ("peepoSprint", EmoteSrc::B("5c20a897fef84f19d3274cb0")),
    ("peepoT", EmoteSrc::B("5c268a6b073d667de5c94ceb")),
    ("peepoTrip", EmoteSrc::B("5e1d46a488e62a5f14dc7610")),
    ("peepoWine", EmoteSrc::B("5e199574bca2995f13fb3851")),
    ("peepoWtf", EmoteSrc::B("5a93403a85dd670847d2d2eb")),
    ("peepoWTF", EmoteSrc::F("267880")),
];

const fn emote_cost(es: &EmoteSrc) -> usize {
    match es {
        EmoteSrc::B(_) => 2,
        EmoteSrc::S(_) => 3,
        _ => 1
    }
}

pub(super) fn items(ip: &mut ItemPack) {
    PEEPOS.iter().for_each(|(name, src)| {
        ip.add_item(*name, src.clone(), emote_cost(src));
    })
}

pub(super) fn loots(lp: &mut LootPack) {
    let peepv = PEEPOS.iter().map(|(name, _)| {
        let it = lp.item(name.to_string());
        let luck: usize = if let ItemType::Emote(s) = &it.itype {
            emote_cost(s)
        } else { 1 };
        (it, luck)
    });
    lp.add_loot("peepo", peepv.collect());
}