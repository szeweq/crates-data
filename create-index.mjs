import { readdir, writeFile } from 'node:fs/promises'
import { join } from 'node:path'

async function createIndex() {
  let out = {}
  const cdir = join('.', 'crates')
  const dirs = await readdir(cdir)
  for (let dt of dirs) {
    const dtpath = join(cdir, dt)
    const crates = await readdir(dtpath)
    out[dt] = crates.filter(x => x.endsWith(".json")).map(x => x.substring(0, x.length-5))
  }
  await writeFile(join(cdir, 'index.json'), JSON.stringify(out))
}

createIndex()