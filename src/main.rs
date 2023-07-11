use std::{path::Path, fs::{self, File}, collections::HashMap, rc::Rc};

use gen::Name;
use itertools::Itertools;

use crate::gen::LootPack;

mod gen;
mod generr;

fn main() -> Result<(), generr::GenError> {
    let pdist = Path::new("dist");
    if pdist.exists() {
        fs::remove_dir_all(pdist)?;
    }
    fs::create_dir(pdist)?;

    let pg = gen::get_packgen();
    let mut items = Vec::new();
    pg.items.iter().for_each(|i| i.add_items(&mut items));
    let items = items.into_iter()
        .map(|it| (it.name.clone(), Rc::new(it)))
        .collect::<HashMap<_, _>>();
    let mut lp = LootPack{
        items, loots: Vec::new()
    };
    pg.loots.iter().for_each(|i| i.add_loots(&mut lp));

    
    let map_by_type = lp.items.iter()
        .map(|(name, it)| (it.itype.group_name(), name.clone()))
        .into_group_map();
    let loot_names = lp.loots.iter().map(|lt| lt.name.clone()).collect::<Vec<_>>();
    let loot_index = LootIndex{
        types: &map_by_type, loots: &loot_names
    };


    write_json(pdist.join("index.json"), &loot_index)?;

    let items_dir = pdist.join("items");
    fs::create_dir(&items_dir)?;
    for (tp, nm) in map_by_type {
        let vi = nm.iter().map(|n| (n, &lp.items[n].itype)).collect::<HashMap<_, _>>();
        write_json(items_dir.join(format!("{tp}.json")), &vi)?;
    }

    let loots_dir = pdist.join("loots");
    fs::create_dir(&loots_dir)?;
    for lt in lp.loots {
        let lit = lt.items.iter().map(|(n, luck)| (n.name.clone(), *luck)).collect::<HashMap<_, _>>();
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
    types: &'a HashMap<&'static str, Vec<Name>>,
    loots: &'a Vec<Name>
}