import glob from 'glob';
import * as fs from 'fs';
import toml from "toml";

glob("public/blueprints/**/*.toml", (err, files) => {
    let names = [];
    let blueprintJson = {};
    for(let file of files) {
        let name = file.split('/').pop().split('.toml')[0]
        names.push(name);
        let json = toml.parse((fs.readFileSync(file).toString()));
        blueprintJson[name] = json;
        fs.writeFileSync(`public/blueprints_all/${name}.toml`, (fs.readFileSync(file).toString()));
    
    }
    fs.writeFileSync('public/blueprints/blueprints.json', JSON.stringify(blueprintJson, null, 4));
})