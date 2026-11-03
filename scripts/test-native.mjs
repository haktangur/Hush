import {seedCatalog} from '../tests/native/catalog-fixtures.mjs';
import { spawnSync, spawn } from 'node:child_process';
import { mkdtempSync, mkdirSync, copyFileSync, writeFileSync, readFileSync, existsSync, unlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

mkdirSync('artifacts', { recursive: true });
const env = {
  ...process.env,
  CARGO_TARGET_DIR: join(process.cwd(), 'src-tauri/target/native'),
  HUSH_TEST_DATA_DIR: mkdtempSync(join(tmpdir(), 'hush-native-')),
  HUSH_CATALOG_LIVE: process.argv.includes('--catalog-live') ? '1' : '',
  HUSH_NATIVE_BACKUP: process.argv.includes('--backup') ? '1' : '',
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
if (env.HUSH_NATIVE_BACKUP) {
  const evidence = [];
  for (const phase of ['reopen','crash-before_generation','verify-before_generation','crash-before_pointer','verify-before_pointer','crash-after_pointer','verify-after_pointer']) {
    const marker = join(env.HUSH_TEST_DATA_DIR, 'backup-fault-reached');
    if (existsSync(marker)) unlinkSync(marker);
    const child = spawn('npx', ['wdio','run','tests/native/wdio.conf.js'], {stdio:'inherit',env:{...env,HUSH_BACKUP_PHASE:phase}});
    let killed = false;
    const timer = setInterval(() => {
      if (!phase.startsWith('crash-') || killed || !existsSync(marker)) return;
      const [pid, point] = readFileSync(marker,'utf8').split(':');
      if (point !== phase.slice(6)) return;
      process.kill(Number(pid),'SIGKILL'); killed = true;
      console.log(`Injected SIGKILL at ${point}; PID ${pid}`);
    },100);
    const status = await new Promise(resolve => child.on('exit',resolve));clearInterval(timer);
    evidence.push({phase,status,killed,marker:existsSync(marker)?readFileSync(marker,'utf8'):null,at:new Date().toISOString()});
    writeFileSync(join(env.HUSH_TEST_DATA_DIR,'backup-run.json'),JSON.stringify(evidence,null,2));
    if (phase.startsWith('crash-') ? !killed || status===0 : status!==0) process.exit(1);
  }
  console.log(`P6 round-trip, malformed archives and three SIGKILL/reopen boundaries passed: ${env.HUSH_TEST_DATA_DIR}`);
  process.exit(0);
}
const reopened = spawnSync('npx', ['wdio', 'run', 'tests/native/wdio.conf.js'], {
  stdio: 'inherit', env: { ...env, HUSH_REOPEN: '1' },
});
process.exit(reopened.status ?? 1);
