import path from 'node:path'
import { fileURLToPath } from 'node:url'
import { defineConfig } from 'tsdown'
import fs from 'node:fs'

const filePath = path.dirname(fileURLToPath(import.meta.url))
const ouputDir = path.join(filePath, 'dist')
export default defineConfig({
  entry: {
    index: 'src/index.ts',
    start: 'src/start/index.ts',
  },
  format: ['esm'],
  unbundle: true,
  dts: true,
  clean: true,
  minify: true,
  shims: true,
  target: 'node22',
  sourcemap: false,
  treeshake: true,
  platform: 'node',
  outDir: ouputDir,
  fixedExtension: true,
  nodeProtocol: true,
  banner: {
    js: '// powered by puniyu\n',
    dts: '// powered by puniyu\n',
  },
onSuccess: async () => {
  if (fs.existsSync(ouputDir)) {
    const files = await fs.promises.readdir(ouputDir)
    await Promise.all(
      files
        .filter(file => file.endsWith('.d.mts'))
        .map(async (file) => {
          const oldPath = path.join(ouputDir, file)
          const newPath = path.join(ouputDir, file.replace('.d.mts', '.d.ts'))
          await fs.promises.rename(oldPath, newPath)
        })
    )
  }
}
})
