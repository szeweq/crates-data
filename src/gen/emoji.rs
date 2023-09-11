use crate::lucks;

use super::{LootPack, ItemPack, Emoji};

macro_rules! const_emojis {
    ($sname:ident {$($emoji:literal as $name:ident: $cost:literal),+}) => {
        const $sname: &[(&str, &str, usize)] = &[
            $((stringify!($name), $emoji, $cost)),+
        ];
    };
}

const_emojis!(ANIMALS {
    "🐶" as dog: 2,
    "🐱" as cat: 3,
    "🐰" as bunny: 4,
    "🐭" as mouse: 1,
    "🐹" as guineapig: 5,
    "🐻" as bear: 5,
    "🐻‍❄️" as polarbear: 7,
    "🐼" as panda: 7,
    "🐨" as koala: 6,
    "🐯" as tiger: 7,
    "🦁" as lion: 8,
    "🦊" as fox: 5,
    "🐺" as wolf: 8,
    "🦓" as zebra: 7,
    "🦄" as unicorn: 20,
    "🐴" as horse: 6,
    "🐷" as pig: 1,
    "🐑" as sheep: 4,
    "🦙" as llama: 9,
    "🐮" as cow: 2,
    "🦆" as duck: 4,
    "🐧" as penguin: 6,
    "🪰" as flybug: 1
});
const_emojis!(FRUITS {
    "🍎" as apple: 5,
    "🍌" as banana: 2,
    "🍒" as cherry: 3,
    "🍇" as grapes: 5,
    "🍋" as lemon: 10,
    "🍊" as orange: 7,
    "🍐" as pear: 7,
    "🍓" as strawberry: 3,
    "🍉" as watermelon: 8,
    "🍅" as tomato: 3,
    "🥝" as kiwi: 11,
    "🍍" as pineapple: 8,
    "🍑" as peach: 8
});
const_emojis!(VEGGIES {
    "🥕" as carrot: 4,
    "🥔" as potato: 1,
    "🥒" as cucumber: 10,
    "🌽" as corn: 7,
    "🧅" as onion: 1,
    "🥦" as broccoli: 8,
    "🫚" as ginger: 7
});
const_emojis!(SWEETS {
    "🍬" as candy: 1,
    "🍪" as cookie: 2,
    "🍦" as icecream: 2,
    "🍰" as shortcake: 3,
    "🧁" as cupcake: 2,
    "🥧" as pie: 6,
    "🍭" as lollipop: 1,
    "🍫" as chocolate: 7,
    "🍮" as custard: 2,
    "🍩" as donut: 2,
    "🍯" as honeypot: 20,
    "🍧" as shavedice: 2
});
const_emojis!(BUILDINGS {
    "🏠" as house: 2,
    "🏡" as house2: 3,
    "🏚️" as house3: 1,
    "🏫" as school: 6,
    "🏢" as office: 5,
    "🏤" as postoffice: 4,
    "🏰" as castle: 10,
    "🏟️" as stadium: 8,
    "🏨" as hotel: 8,
    "🏥" as hospital: 7,
    "🏭" as factory: 6,
    "🏦" as bank: 10
});

pub(super) fn items(ip: &mut ItemPack) {
    ANIMALS.iter().chain(FRUITS).chain(VEGGIES).chain(SWEETS).chain(BUILDINGS).for_each(|&(name, emoji, cost)| {
        ip.add_item(name, Emoji(emoji), cost);
    });
    ip.add_item("knife", Emoji("🔪"), 3200);
    ip.add_item("pistol", Emoji("🔫"), 10);
    ip.add_item("glove", Emoji("🧤"), 160);

}

pub(super) fn loots(lp: &mut LootPack) {
    lp.add_loot_names("animals", lucks!{
        "dog": 9,
        "cat": 8,
        "bunny": 7,
        "mouse": 6,
        "guineapig": 4,
        "bear": 5,
        "panda": 7,
        "koala": 2,
        "lion": 5,
        "tiger": 4,
        "fox": 7,
        "polarbear": 4,
        "wolf": 2,
        "zebra": 3,
        "unicorn": 1,
        "horse": 6,
        "pig": 9,
        "sheep": 6,
        "llama": 2,
        "cow": 8,
        "duck": 6,
        "penguin": 4,
        "flybug": 8
    });
    lp.add_loot_names("fruits", lucks!{
        "apple": 2,
        "banana": 4,
        "cherry": 3,
        "grapes": 5,
        "lemon": 1,
        "orange": 2,
        "pear": 3,
        "strawberry": 3,
        "watermelon": 2,
        "tomato": 5,
        "kiwi": 1,
        "pineapple": 2,
        "peach": 2
    });
    lp.add_loot_names("veggies", lucks!{
        "carrot": 2,
        "potato": 4,
        "cucumber": 1,
        "corn": 2,
        "onion": 5,
        "broccoli": 2,
        "ginger": 2
    });
    lp.add_loot_names("sweets", lucks!{
        "candy": 9,
        "cookie": 7,
        "icecream": 7,
        "shortcake": 7,
        "cupcake": 7,
        "pie": 4,
        "lollipop": 9,
        "chocolate": 3,
        "custard": 8,
        "donut": 8,
        "honeypot": 2,
        "shavedice": 7
    });
    lp.add_loot_names("buildings", lucks!{
        "house": 10,
        "house2": 8,
        "house3": 12,
        "school": 8,
        "office": 7,
        "postoffice": 7,
        "castle": 1,
        "stadium": 2,
        "hotel": 3,
        "hospital": 3,
        "factory": 3,
        "bank": 1
    });
    lp.add_loot_names("rare-knife", [("knife", 1), ("pistol", 99)]);
    lp.add_loot_names("rare-gloves", [("glove", 2), ("pistol", 98)]);
}