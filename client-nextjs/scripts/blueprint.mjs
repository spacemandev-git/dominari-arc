import glob from 'glob';
import * as fs from 'fs';

glob("../assets/blueprints/**/*.toml", (err, files) => {
    let names = [];
    for(let file of files) {
        let name = file.split('/').pop().split('.toml')[0]
        names.push(name);
    }
    fs.writeFileSync('public/blueprints.json', JSON.stringify(names));
})