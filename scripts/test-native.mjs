import {seedCatalog} from '../tests/native/catalog-fixtures.mjs';
import { spawnSync, spawn } from 'node:child_process';
import { mkdtempSync, mkdirSync, copyFileSync, writeFileSync, readFileSync, existsSync, unlinkSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';

if(process.argv.includes('--regressions')) {
 for(const flag of ['', '--room', '--collections', '--journal', '--catalog', '--backup', '--quality']) {
  const result=spawnSync('npm',['run','test:native','--',...(flag?[flag]:[])],{stdio:'inherit',env:process.env});
  if(result.status!==0)process.exit(result.status??1);
 }
 process.exit(0);
}
mkdirSync('artifacts', { recursive: true });
const env = {
  ...process.env,
  CARGO_TARGET_DIR: join(process.cwd(), 'src-tauri/target/native'),
  HUSH_TEST_DATA_DIR: process.argv.includes('--manual-resume') ? JSON.parse(readFileSync('artifacts/manual-session.json','utf8')).root : mkdtempSync(join(tmpdir(), 'hush-native-')),
  HUSH_CATALOG_LIVE: process.argv.includes('--catalog-live') ? '1' : '',
  HUSH_NATIVE_QUALITY: process.argv.includes('--quality') ? '1' : '',
  HUSH_NATIVE_VISUAL: process.argv.includes('--visual') ? '1' : '',
  HUSH_NATIVE_PERFORMANCE: process.argv.includes('--performance') ? '1' : '',
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
  ['npm', ['run', env.HUSH_NATIVE_PERFORMANCE ? 'build:native:profile' : 'build:native:test']],
  ['npx', ['wdio', 'run', 'tests/native/wdio.conf.js']],
]) {
  if (command === 'npx' && (process.argv.includes('--manual')||process.argv.includes('--manual-resume'))) {
    writeFileSync(join(env.HUSH_TEST_DATA_DIR,'offline'),'1');
    const child=spawn(join(process.cwd(),'src-tauri/target/native/debug/hush'),[],{env,stdio:'ignore',detached:true});child.unref();
    writeFileSync('artifacts/manual-session.json',JSON.stringify({root:env.HUSH_TEST_DATA_DIR,pid:child.pid,startedAt:new Date().toISOString()}));
    console.log(`Manual isolated offline Hush session: PID ${child.pid}; ${env.HUSH_TEST_DATA_DIR}`);
    process.exit(0);
  }
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
