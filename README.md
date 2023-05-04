# crates-data
A collection of crate opening data.

## Crate types
The crate types define how an item should look and what data it should have. All files are contained in a directory which defines its type. These are the following types:

- Letter
- Emoji
- Steam Game ID
- Youtube Video (using its ID)
- Youtube User (name without `@`)
- Emotes (FrankerFaceZ, BetterTTV, 7TV)

## Crate files
Crates have its own unique file name. All crates are JSON objects with item name keys and data with luck values and other neccessary data.

There is an `crates/index.json` file in this repo. It contains lists of crates grouped by its type.

## Luck value
Each item in a crate has a "luck value". This is a number of chances an item can be randomly selected out of all items. The higher value the more common that item is selected.