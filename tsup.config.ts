import { Options } from 'tsup'

const config: Options = {
  // --format cjs --dts --minify --clean --legacy-output dist/index.js
  entry: ['index.ts'],
  dts: true,
  sourcemap: true,
  format: ['cjs', 'esm'],
}

export default config
