"""Mutate only the synthetic archive supplied by the native test runner."""
import sys, zipfile, json, hashlib, sqlite3, tempfile
from pathlib import Path
root=Path(sys.argv[1])
with zipfile.ZipFile(root/'complete.hushbackup') as z:
    original={n:z.read(n) for n in z.namelist()}
def emit(name, change, rehash=False):
    files=dict(original); change(files)
    if rehash:
        m=json.loads(files['manifest.json']);m['files']=[dict(path=n,bytes=len(b),sha256=hashlib.sha256(b).hexdigest()) for n,b in files.items() if n!='manifest.json'];files['manifest.json']=json.dumps(m).encode()
    with zipfile.ZipFile(root/(name+'.hushbackup'),'w',zipfile.ZIP_DEFLATED) as z:
        for n,b in files.items():z.writestr(n,b)
def dbchange(sql):
    def change(f):
        with tempfile.TemporaryDirectory(dir=root) as d:
            p=Path(d)/'db';p.write_bytes(f['library.sqlite3']);c=sqlite3.connect(p);c.executescript(sql);c.close();f['library.sqlite3']=p.read_bytes()
    return change
(root/'not-zip.hushbackup').write_bytes(b'broken archive')
(root/'truncated.hushbackup').write_bytes((root/'complete.hushbackup').read_bytes()[:200])
for name,path in [('traversal','../escape'),('absolute','/tmp/escape'),('backslash','assets\\escape'),('unexpected','secret.txt')]:emit(name,lambda f,p=path:f.update({p:b'x'}))
emit('hash',lambda f:f.update({'library.sqlite3':f['library.sqlite3']+b'x'}))
emit('missing',lambda f:f.pop(next(n for n in f if n.endswith('thumb.webp'))))
emit('image',lambda f:f.update({next(n for n in f if n.endswith('thumb.webp')):b'bad image'}),True)
emit('future',dbchange('PRAGMA user_version=99;'),True)
emit('fk',dbchange("PRAGMA foreign_keys=OFF; UPDATE game_screenshots SET asset_id='missing';"),True)
emit('details',dbchange('DELETE FROM book_details;'),True)
emit('settings',dbchange("UPDATE app_settings SET payload_json=json_set(payload_json,'$.room.neonIntensity',101);"),True)
emit('draft',dbchange("UPDATE drafts SET payload_json='{}';"),True)
emit('trigger',dbchange('CREATE TRIGGER unexpected AFTER INSERT ON tags BEGIN DELETE FROM tags; END;'),True)
emit('slot',dbchange("UPDATE room_slots SET item_id=(SELECT id FROM library_items WHERE kind='book' LIMIT 1),asset_id=NULL WHERE slot_id='record_stand';"),True)
with zipfile.ZipFile(root/'duplicate.hushbackup','w') as z:
    for n,b in original.items():z.writestr(n,b)
    z.writestr('manifest.json',original['manifest.json'])
with zipfile.ZipFile(root/'symlink.hushbackup','w') as z:
    for n,b in original.items():
        info=zipfile.ZipInfo(n);info.create_system=3;info.external_attr=(0o120777 if n=='library.sqlite3' else 0o100600)<<16;z.writestr(info,b)
# Advertised central-directory size exceeds DB limit; no large allocation/disk filling.
b=bytearray((root/'complete.hushbackup').read_bytes());pos=0
while True:
    pos=b.find(b'PK\x01\x02',pos)
    if pos<0:break
    length=int.from_bytes(b[pos+28:pos+30],'little')
    if b[pos+46:pos+46+length]==b'library.sqlite3':b[pos+24:pos+28]=(256*1024*1024+1).to_bytes(4,'little');break
    pos+=46+length
(root/'oversize.hushbackup').write_bytes(b)
