import {spawnSync} from 'node:child_process';
import {mkdirSync,mkdtempSync,cpSync,symlinkSync,readFileSync,writeFileSync,rmSync} from 'node:fs';
import {join} from 'node:path';
import {createHash} from 'node:crypto';
function run(command,args){const r=spawnSync(command,args,{stdio:'inherit'});if(r.status!==0)process.exit(r.status??1)}
const version=JSON.parse(readFileSync('package.json')).version;
if(JSON.parse(readFileSync('src-tauri/tauri.conf.json')).version!==version||!readFileSync('src-tauri/Cargo.toml','utf8').includes(`version = "${version}"`))throw Error('Version mismatch');
// Build only the app with Tauri; DMG creation uses no Finder/System Events/AppleScript.
run('npx',['tauri','build','--target','aarch64-apple-darwin','--bundles','app']);
const base='src-tauri/target/aarch64-apple-darwin/release/bundle';
const app=join(base,'macos/Hush.app');
run('codesign',['--verify','--deep','--strict','--verbose=2',app]);
mkdirSync(join(base,'dmg'),{recursive:true});mkdirSync('artifacts',{recursive:true});
const stage=mkdtempSync('artifacts/dmg-stage-');
cpSync(app,join(stage,'Hush.app'),{recursive:true});
symlinkSync('/Applications',join(stage,'Applications'));
cpSync('docs/KURULUM-VE-KULLANIM.md',join(stage,'KURULUM-VE-KULLANIM.md'));
cpSync('THIRD-PARTY-NOTICES.md',join(stage,'THIRD-PARTY-NOTICES.md'));
const dmg=join(base,'dmg',`Hush-${version}-aarch64.dmg`);
run('hdiutil',['create','-volname','Hush','-srcfolder',stage,'-ov','-format','UDZO',dmg]);
run('hdiutil',['verify',dmg]);
rmSync(stage,{recursive:true,force:true}); // staging copy only; the DMG now holds the app
run('ditto',['-c','-k','--sequesterRsrc','--keepParent',app,join(base,`Hush-${version}-aarch64.app.zip`)]);
const files=[dmg,join(base,`Hush-${version}-aarch64.app.zip`)];
writeFileSync(join(base,'SHA256SUMS.txt'),files.map(p=>`${createHash('sha256').update(readFileSync(p)).digest('hex')}  ${p.split('/').pop()}`).join('\n')+'\n');
run('node',['scripts/verify-release.mjs']);
