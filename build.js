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

  Object.entries(assets)
    .map(([key, value]) => ({
      name: key.split('/').pop(),
      dir: path.resolve(outDir, key.split('/').slice(0, -1).join('/')),
      data: value.source,
    }))
    .filter(({ name }) => {
      if (name.endsWith('.flf')) {
        return name === 'Standard.flf';
      } else {
        return true;
      }
    })
    .forEach(async ({ name, dir, data }) => {
      await fs.mkdir(dir, { recursive: true });
      await fs.writeFile(path.resolve(dir, name), data);
    });
});
