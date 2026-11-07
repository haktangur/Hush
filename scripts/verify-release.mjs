import {readFileSync,readdirSync,statSync,writeFileSync,mkdirSync} from 'node:fs';
import {spawnSync,execFileSync} from 'node:child_process';
const app='src-tauri/target/aarch64-apple-darwin/release/bundle/macos/Hush.app';
const binary=`${app}/Contents/MacOS/hush`;
const forbidden=['HUSH_TEST_DATA_DIR','app.hush.desktop.e2e','e2e-only','tauri_plugin_wdio','wdio-webdriver','plugin:wdio','backup-fault-reached','backup-save-selection','backup-open-selection','backup-io-fault','http-attempts.txt','catalog-offline','mock-adapter'];
const findings=[];
function scan(path){for(const n of readdirSync(path)){const p=`${path}/${n}`;if(statSync(p).isDirectory())scan(p);else {const b=readFileSync(p);for(const token of forbidden)if(b.includes(token))findings.push(`${p}: ${token}`);if(/\.(sqlite3|hushbackup|env|p8|p12|key)$/.test(n))findings.push(p)}}}
scan(app);scan('dist');
const secretPattern=/(?:-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----|gh[pousr]_[A-Za-z0-9]{30,}|AKIA[0-9A-Z]{16})/;
for(const path of execFileSync('git',['ls-files','-z'],{encoding:'utf8'}).split('\0').filter(Boolean)) {
 if(secretPattern.test(readFileSync(path,'utf8')))findings.push(`Secret-like content in ${path}`);
}

const arch=execFileSync('lipo',['-archs',binary],{encoding:'utf8'}).trim();if(arch!=='arm64')findings.push(`Unexpected architecture ${arch}`);
const sign=spawnSync('codesign',['--verify','--deep','--strict',app],{encoding:'utf8'});if(sign.status!==0)findings.push('Invalid signature');
const identity=spawnSync('codesign',['-dv','--verbose=4',app],{encoding:'utf8'}).stderr;if(!identity.includes('Signature=adhoc'))findings.push('Not ad-hoc');
const tree=execFileSync('cargo',['tree','--locked','--manifest-path','src-tauri/Cargo.toml','-e','normal'],{encoding:'utf8'});if(/tauri-plugin-wdio/.test(tree))findings.push('Test dependency in production graph');
mkdirSync('artifacts',{recursive:true});writeFileSync('artifacts/release-audit.json',JSON.stringify({arch,adHoc:identity.includes('Signature=adhoc'),forbidden,findings,productionDependencyGraphHasWdio:false},null,2));
if(findings.length)throw Error(findings.join('\n'));
console.log('Release audit passed: arm64, ad-hoc, no known test markers, fixture data or WDIO production dependencies.');
