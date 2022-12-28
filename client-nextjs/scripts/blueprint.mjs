import glob from 'glob';
import * as fs from 'fs';

glob("public/blueprints/**/*.toml", (err, files) => {
    let names = [];
    for(let file of files) {
        let name = file.split('/').pop().split('.toml')[0]
        names.push(name);
        fs.writeFileSync(`public/blueprints_all/${name}.toml`, (fs.readFileSync(file).toString()));
    }
    fs.writeFileSync('public/blueprints/blueprints.json', JSON.stringify(names));
})