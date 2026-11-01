import { spawnSync } from 'node:child_process';
import { mkdtempSync, mkdirSync } from 'node:fs';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
mkdirSync('artifacts',{recursive:true});
const env={...process.env,HUSH_TEST_DATA_DIR:mkdtempSync(join(tmpdir(),'hush-native-'))};
for(const [command,args]of [['npm',['run','build:native:test']],['npx',['wdio','run','tests/native/wdio.conf.js']]]){const r=spawnSync(command,args,{stdio:'inherit',env});if(r.status!==0)process.exit(r.status??1)}

const reopened=spawnSync('npx',['wdio','run','tests/native/wdio.conf.js'],{stdio:'inherit',env:{...env,HUSH_REOPEN:'1'}});process.exit(reopened.status??1);
