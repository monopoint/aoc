import { assert } from 'console';
import * as fs from 'fs';
import { type } from 'os';
import { mainModule } from 'process';

fs.readFile('../input', 'utf-8', (err, data) => {
    main(data);
})

type File = {
    name: String,
    size: number,
    files: File[],
    parent: File | null,
}

type Command = {
    cmd: "ls"|"cd",
    args: String,
    output: String[]
}

function getSize(file: File): number {
    let size: number = file.files.map(f => getSize(f)).reduce((a,c) => a+c, 0);

    return (file.size | 0) + size;
}

function main(input: String){
    let commands = input.split('$ ').map(c => {
        let lines = c.split('\n');
        let cmdLine = lines.shift()?.split(' ');
        return {cmd: cmdLine?.[0], args: cmdLine?.[1], output: lines} as Command;
    });
    //commands.forEach(c => console.log(c));
    
    let root: File = {name: "root", size: 0, files: [], parent: null};
    let current = root;
    let flatFiles: File[] = [];
    flatFiles.push(root);
    

    commands.forEach(c => {
        if (c.cmd == 'cd') {
            if (c.args === "/") current = root;
            else if (c.args === "..") {
                assert(current.parent, "ERROR Trying to cd to empty parent");
                current = current.parent!;
            }
            else {
                current = current.files.find(f => f.name === c.args)!
                assert(current, "ERROR Trying to cd into non existing dir");
            }
        }
        if (c.cmd == 'ls'){
            if (current.files.length === 0) {
                current.files = c.output.map(o => {
                    let oc = o.split(' ');
                    if (oc[0] === 'dir') {
                        let f = {name: oc[1], files: [], parent: current, size: 0} as File;
                        flatFiles.push(f);
                        return f;
                    } else {
                        let f = {name: oc[1], files: [], parent: current, size: parseInt(oc[0])} as File;
                        flatFiles.push(f);
                        return f;
                    }
                })
            }
            
        }
    })

    //console.dir(flatFiles);

    let part1 = flatFiles.filter(f => f.size == 0)
    .map(f => {
        return {file: f, sum: getSize(f)}
    })
    .filter(s => {
        return s.sum <= 100000;}
    ).reduce((a,c) => {
        console.dir(c);
        return a + c.sum
    }, 0);

    console.log("Part 1: " + part1);

    let totalSize = flatFiles
    .filter(f => f.size > 0)
    .reduce((a, f) => a + f.size, 0)
    console.log("Total size " + totalSize);

    let free = 70000000 - totalSize;
    let target = 30000000 - free;
    console.log("Free "+ free),
    console.log("target " + target);

    let part2 = flatFiles
    .filter(f => f.size === 0)
    .map(f => getSize(f))
    .filter(f => f > target)
    .sort().reverse().forEach(f => console.log(f));

    console.log("Part 2: " + part2);

}

