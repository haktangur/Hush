import {createHash} from 'node:crypto';
import {mkdirSync,writeFileSync,readFileSync} from 'node:fs';
import {join} from 'node:path';
export function seedCatalog(root){
 const cache=join(root,'catalog');mkdirSync(cache,{recursive:true});
 const put=(url,value,suffix='json')=>writeFileSync(join(cache,`${createHash('sha256').update(url).digest('hex')}.${suffix}`),typeof value==='string'||Buffer.isBuffer(value)?value:JSON.stringify(value));
 const book=new URL('https://openlibrary.org/search.json');for(const [k,v] of Object.entries({q:'P5 sentetik kitap',limit:'20',page:'1',lang:'tr',fields:'key,title,author_name,first_publish_year,cover_i,edition_key'}))book.searchParams.append(k,v);
 put(book.href,{docs:[{key:'/works/OL900000001W',title:'P5 sentetik kitap',author_name:['Sentetik yazar'],first_publish_year:2024}]});
 put('https://openlibrary.org/works/OL900000001W/editions.json?limit=20',{entries:[{key:'/books/OL900000001M',title:'P5 kitap baskısı',publish_date:'2025'}]});
 put('https://openlibrary.org/books/OL900000001M.json',{title:'P5 kitap baskısı',publish_date:'2025',number_of_pages:240,isbn_13:['9780000000002'],publishers:['Sentetik yayınevi'],languages:[{key:'/languages/tur'}]});
 const group='11111111-1111-4111-8111-111111111111',release='22222222-2222-4222-8222-222222222222';
 const album=new URL('https://musicbrainz.org/ws/2/release-group/');for(const [k,v] of Object.entries({query:'releasegroup:"P5 sentetik albüm" AND primarytype:album',fmt:'json',limit:'20',offset:'0'}))album.searchParams.append(k,v);
 const credit=[{name:'Sentetik sanatçı',joinphrase:' & '},{name:'İkinci sanatçı'}];
 put(album.href,{'release-groups':[{id:group,title:'P5 sentetik albüm','artist-credit':credit,'first-release-date':'2023'}]});
 put(`https://musicbrainz.org/ws/2/release?release-group=${group}&fmt=json&limit=20`,{releases:[{id:release,title:'P5 albüm baskısı',date:'2024',country:'TR'}]});
 put(`https://musicbrainz.org/ws/2/release-group/${group}?inc=artist-credits&fmt=json`,{title:'P5 sentetik albüm','artist-credit':credit,'first-release-date':'2023'});
 put(`https://musicbrainz.org/ws/2/release/${release}?inc=recordings+artist-credits&fmt=json`,{media:[{tracks:[{title:'Sentetik parça',length:123456}]}]});
 put(`https://coverartarchive.org/release/${release}/front-500`,readFileSync('tests/fixtures/room-album.webp'),'cover');
 put(`https://coverartarchive.org/release-group/${group}/front-500`,'','missing');
 const bad='33333333-3333-4333-8333-333333333333';
 put(`https://musicbrainz.org/ws/2/release/${bad}?inc=recordings+artist-credits&fmt=json`,{media:[]});
 put(`https://coverartarchive.org/release/${bad}/front-500`,'synthetic invalid image','cover');
}
