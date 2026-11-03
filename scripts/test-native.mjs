import {seedCatalog} from '../tests/native/catalog-fixtures.mjs';
import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync, copyFileSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

mkdirSync('artifacts', { recursive: true });
const env = {
  ...process.env,
  CARGO_TARGET_DIR: join(process.cwd(), 'src-tauri/target/native'),
  HUSH_TEST_DATA_DIR: mkdtempSync(join(tmpdir(), 'hush-native-')),
  HUSH_CATALOG_LIVE: process.argv.includes('--catalog-live') ? '1' : '',
  HUSH_NATIVE_CATALOG: process.argv.includes('--catalog') ? '1' : '',
  HUSH_NATIVE_JOURNAL: process.argv.includes('--journal') ? '1' : '',
  HUSH_NATIVE_COLLECTIONS: process.argv.includes('--collections') ? '1' : '',
  HUSH_NATIVE_ROOM: process.argv.includes('--room') ? '1' : '',
  HUSH_NATIVE_COVER: process.argv.includes('--cover') ? '1' : '',
};
if (env.HUSH_NATIVE_CATALOG) seedCatalog(env.HUSH_TEST_DATA_DIR);
if (env.HUSH_NATIVE_COVER) {
  env.HUSH_TEST_COVER_SOURCE = join(env.HUSH_TEST_DATA_DIR, 'source.png');
  copyFileSync('src-tauri/icons/icon.png', env.HUSH_TEST_COVER_SOURCE);
  writeFileSync('artifacts/native-cover-source.txt', env.HUSH_TEST_COVER_SOURCE);
  console.log(`Select this synthetic PNG in the native chooser: ${env.HUSH_TEST_COVER_SOURCE}`);
}
writeFileSync(join(env.HUSH_TEST_DATA_DIR, 'http-attempts.txt'), '');
console.log(`Native evidence data: ${env.HUSH_TEST_DATA_DIR}`);
for (const [command, args] of [
  ['npm', ['run', 'build:native:test']],
  ['npx', ['wdio', 'run', 'tests/native/wdio.conf.js']],
]) {
  const result = spawnSync(command, args, { stdio: 'inherit', env });
  if (result.status !== 0) process.exit(result.status ?? 1);
}
const reopened = spawnSync('npx', ['wdio', 'run', 'tests/native/wdio.conf.js'], {
  stdio: 'inherit', env: { ...env, HUSH_REOPEN: '1' },
});
process.exit(reopened.status ?? 1);
