import { readdir, readFile } from 'node:fs/promises'
import { join } from 'node:path'
import { exit } from 'node:process'
import { z } from 'zod'

const luck = z.number().min(1)
const schemas = {
  emoji: z.record(z.object({luck, emoji: z.string().emoji()})),
  letter: z.record(z.string().length(1), z.object({luck})),
  steamgame: z.record(z.object({luck, id: z.number()})),
  ytvideo: z.record(z.string().regex(/^[\w\d_-]{11}$/), {luck}),
  ytuser: z.record(z.string(), {luck}),
}

async function validate() {
  const cdir = join('.', 'crates')
  const dirs = (await readdir(cdir)).filter(x => x != "index.json")
  for (let dt of dirs) {
    const dtpath = join(cdir, dt)
    const crates = await readdir(dtpath)
    if (!(dt in schemas)) {
      console.error("Could not check crate type:", dt)
      return -1
    }
    const cz = schemas[dt]
    for (let cf of crates) {
      const f = await readFile(join(dtpath, cf), 'utf8')
      let d
      try {
        d = JSON.parse(f)
      } catch (e) {
        console.error("Invalid JSON crate file: ", cf, ";", e)
        return 2
      }
      const p = await cz.safeParseAsync(d)
      if (!p.success) {
        console.error("Invalid data in crate file:", cf)
        console.error(p.error.toString())
        return 1
      }
    }
  }
  return 0
}

validate().then(c => exit(c))