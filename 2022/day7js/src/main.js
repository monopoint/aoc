"use strict";
var __createBinding = (this && this.__createBinding) || (Object.create ? (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    var desc = Object.getOwnPropertyDescriptor(m, k);
    if (!desc || ("get" in desc ? !m.__esModule : desc.writable || desc.configurable)) {
      desc = { enumerable: true, get: function() { return m[k]; } };
    }
    Object.defineProperty(o, k2, desc);
}) : (function(o, m, k, k2) {
    if (k2 === undefined) k2 = k;
    o[k2] = m[k];
}));
var __setModuleDefault = (this && this.__setModuleDefault) || (Object.create ? (function(o, v) {
    Object.defineProperty(o, "default", { enumerable: true, value: v });
}) : function(o, v) {
    o["default"] = v;
});
var __importStar = (this && this.__importStar) || function (mod) {
    if (mod && mod.__esModule) return mod;
    var result = {};
    if (mod != null) for (var k in mod) if (k !== "default" && Object.prototype.hasOwnProperty.call(mod, k)) __createBinding(result, mod, k);
    __setModuleDefault(result, mod);
    return result;
};
Object.defineProperty(exports, "__esModule", { value: true });
const console_1 = require("console");
const fs = __importStar(require("fs"));
fs.readFile('../input', 'utf-8', (err, data) => {
    main(data);
});
function getSize(file) {
    let size = file.files.map(f => getSize(f)).reduce((a, c) => a + c, 0);
    return (file.size | 0) + size;
}
function main(input) {
    let commands = input.split('$ ').map(c => {
        var _a;
        let lines = c.split('\n');
        let cmdLine = (_a = lines.shift()) === null || _a === void 0 ? void 0 : _a.split(' ');
        return { cmd: cmdLine === null || cmdLine === void 0 ? void 0 : cmdLine[0], args: cmdLine === null || cmdLine === void 0 ? void 0 : cmdLine[1], output: lines };
    });
    //commands.forEach(c => console.log(c));
    let root = { name: "root", size: 0, files: [], parent: null };
    let current = root;
    let flatFiles = [];
    flatFiles.push(root);
    commands.forEach(c => {
        if (c.cmd == 'cd') {
            if (c.args === "/")
                current = root;
            else if (c.args === "..") {
                (0, console_1.assert)(current.parent, "ERROR Trying to cd to empty parent");
                current = current.parent;
            }
            else {
                current = current.files.find(f => f.name === c.args);
                (0, console_1.assert)(current, "ERROR Trying to cd into non existing dir");
            }
        }
        if (c.cmd == 'ls') {
            if (current.files.length === 0) {
                current.files = c.output.map(o => {
                    let oc = o.split(' ');
                    if (oc[0] === 'dir') {
                        let f = { name: oc[1], files: [], parent: current, size: 0 };
                        flatFiles.push(f);
                        return f;
                    }
                    else {
                        let f = { name: oc[1], files: [], parent: current, size: parseInt(oc[0]) };
                        flatFiles.push(f);
                        return f;
                    }
                });
            }
        }
    });
    //console.dir(flatFiles);
    let part1 = flatFiles.filter(f => f.size == 0)
        .map(f => {
        return { file: f, sum: getSize(f) };
    })
        .filter(s => {
        return s.sum <= 100000;
    }).reduce((a, c) => {
        console.dir(c);
        return a + c.sum;
    }, 0);
    console.log("Part 1: " + part1);
    let totalSize = flatFiles
        .filter(f => f.size > 0)
        .reduce((a, f) => a + f.size, 0);
    console.log("Total size " + totalSize);
    let free = 70000000 - totalSize;
    let target = 30000000 - free;
    console.log("Free " + free),
        console.log("target " + target);
    let part2 = flatFiles
        .filter(f => f.size === 0)
        .map(f => getSize(f))
        .filter(f => f > target)
        .sort().reverse().forEach(f => console.log(f));
    console.log("Part 2: " + part2);
}
