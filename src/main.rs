use std::{path::Path, fs::{self, File}, collections::{HashMap, BTreeMap}, rc::Rc};

use gen::Name;

use crate::gen::{LootPack, ItemPack};

mod gen;
mod generr;

fn main() -> Result<(), generr::GenError> {
    let run_start = std::time::Instant::now();
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

    let map_by_type = lp.items.iter()
        .fold(BTreeMap::new(), |mut m, (name, it)| {
            if m.entry(it.itype.group_name())
                .or_insert_with(BTreeMap::new)
                .insert(name.clone(), it.cost).is_some() {
                panic!("Duplicate item: {}", name);
            }
            m
        });

    let loot_costs = lp.loots.iter().map(|lt| {
        let costsum = lt.items.iter().map(|(_, l)| *l as f64).sum::<f64>();
        let gsum = lt.items.iter().map(|(it, l)| (l*l*it.cost) as f64).sum::<f64>() / costsum;
        (lt.name.clone(), gsum.sqrt().round() as usize)
    }).collect::<BTreeMap<_, _>>();
    let loot_index = LootIndex{
        types: &map_by_type, loots: &loot_costs
    };

    write_json(pdist.join("index.json"), &loot_index)?;

    save_items_in_types(pdist, &lp, &map_by_type)?;
    save_loots(pdist, &lp)?;

    println!("Generated!");
    println!("Time elapsed: {:.2?}", run_start.elapsed());
    Ok(())
}

fn write_json<P: AsRef<Path>, T: serde::ser::Serialize>(path: P, data: &T) -> Result<(), generr::GenError> {
    Ok(serde_json::to_writer(File::create(path)?, data)?)
}

#[derive(serde::Serialize)]
struct LootIndex<'a> {
    types: &'a BTreeMap<&'static str, BTreeMap<Name, usize>>,
    loots: &'a BTreeMap<Name, usize>
}

fn save_items_in_types(out_dir: &Path, lp: &LootPack, items_by_type: &BTreeMap<&'static str, BTreeMap<Name, usize>>) -> Result<(), generr::GenError> {
    let items_dir = out_dir.join("items");
    fs::create_dir(&items_dir)?;
    for (tp, nm) in items_by_type {
        let vi = nm.keys()
            .map(|n| (n, &lp.items[n].itype))
            .collect::<BTreeMap<_, _>>();
        write_json(items_dir.join(format!("{tp}.json")), &vi)?;
    }
    Ok(())
}

fn save_loots(out_dir: &Path, lp: &LootPack) -> Result<(), generr::GenError> {
    let loots_dir = out_dir.join("loots");
    fs::create_dir(&loots_dir)?;
    for lt in &lp.loots {
        let lit = lt.items.iter().map(|(n, luck)| (n.name.clone(), *luck)).collect::<BTreeMap<_, _>>();
        write_json(loots_dir.join(format!("{}.json", lt.name)), &lit)?;
    }
    Ok(())
}