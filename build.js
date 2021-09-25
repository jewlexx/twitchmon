const fs = require('fs/promises');
const path = require('path');
const ncc = require('@vercel/ncc');
const packageObj = require('./package.json');

ncc(path.resolve(__dirname, 'src', 'index.ts'), {
  cache: path.resolve(__dirname, '.cache'),
  minify: true,
  license: 'LICENSE',
}).then(async ({ code, assets }) => {
  const outDir = path.resolve(__dirname, 'out');
  await fs.mkdir(outDir, { recursive: true });
  const output = code.replaceAll('{VERSION}', packageObj.version);
  await fs.writeFile(path.resolve(outDir, 'index.js'), output);
  await fs.writeFile(
    path.resolve(outDir, 'LICENSE.txt'),
    assets.LICENSE.source,
  );
  await fs.writeFile(
    path.resolve(outDir, 'index.d.ts'),
    assets['index.d.ts'].source,
  );
});
