use std::{path::Path, fs::{self, File}, collections::{HashMap, BTreeMap}, rc::Rc};

use gen::Name;

use crate::gen::{LootPack, ItemPack};

mod gen;
mod generr;

fn main() -> Result<(), generr::GenError> {
    let pdist = Path::new("dist");
    if pdist.exists() {
        fs::remove_dir_all(pdist)?;
    }
    fs::create_dir(pdist)?;

    let pg = gen::get_packgen();
    let mut ip = ItemPack { items: Vec::new() };
    pg.items.iter().for_each(|i| i.add_items(&mut ip));
    let items = ip.items.into_iter()
        .map(|it| (it.name.clone(), Rc::new(it)))
        .collect::<HashMap<_, _>>();
    let mut lp = LootPack{
        items, loots: Vec::new()
    };
    pg.loots.iter().for_each(|i| i.add_loots(&mut lp));

    let mut map_by_type = BTreeMap::new();
    lp.items.iter().for_each(|(name, it)| {
        map_by_type.entry(it.itype.group_name()).or_insert_with(Vec::new).push(name.clone());
    });
    map_by_type.iter_mut().for_each(|(_, v)| v.sort());

    let loot_costs = lp.loots.iter().map(|lt| {
        let gsum = lt.items.iter().map(|&(_, l)| (l*l) as f64).sum::<f64>() / lt.items.len() as f64;
        (lt.name.clone(), gsum.sqrt().round() as usize)
    }).collect::<BTreeMap<_, _>>();
    let loot_index = LootIndex{
        types: &map_by_type, loots: &loot_costs
    };

    write_json(pdist.join("index.json"), &loot_index)?;

    let items_dir = pdist.join("items");
    fs::create_dir(&items_dir)?;
    for (tp, nm) in map_by_type {
        let vi = nm.iter().map(|n| (n, &lp.items[n].itype)).collect::<BTreeMap<_, _>>();
        write_json(items_dir.join(format!("{tp}.json")), &vi)?;
    }

    let loots_dir = pdist.join("loots");
    fs::create_dir(&loots_dir)?;
    for lt in lp.loots {
        let lit = lt.items.iter().map(|(n, luck)| (n.name.clone(), *luck)).collect::<BTreeMap<_, _>>();
        write_json(loots_dir.join(format!("{}.json", lt.name)), &lit)?;
    }

    println!("Generated!");
    Ok(())
}

fn write_json<P: AsRef<Path>, T: serde::ser::Serialize>(path: P, data: &T) -> Result<(), generr::GenError> {
    Ok(serde_json::to_writer(File::create(path)?, data)?)
}

#[derive(serde::Serialize)]
struct LootIndex<'a> {
    types: &'a BTreeMap<&'static str, Vec<Name>>,
    loots: &'a BTreeMap<Name, usize>
}