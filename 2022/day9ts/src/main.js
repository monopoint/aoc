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
const fs = __importStar(require("fs"));
fs.readFile("../input", "utf-8", (err, data) => {
    console.log(err ? err : "");
    main(data);
});
function main(input) {
    console.log("Hello AoC");
    //console.log(input);
    let hmoves = input
        .split("\n")
        .map((s) => {
        let m = s.split(" ")[0];
        let n = +s.split(" ")[1];
        //console.log("" + m + n);
        return Array(n + 1)
            .join(m)
            .split("");
    })
        .flat();
    //part1(hmoves);
    part2(hmoves);
}
function delay(ms) {
    return new Promise((resolve) => setTimeout(resolve, ms));
}
function part2(moves) {
    console.log(moves);
    let flags = new Set();
    let h = { x: 0, y: 0 };
    let rope = [];
    while (rope.length < 9) {
        rope.push({ x: 0, y: 0, p: rope.length });
    }
    let i = 0;
    [...moves].forEach((move) => {
        i++;
        // Mark position of tail
        //console.log(`${h.x}::${h.y}`);
        //flags.add(`${t.x}::${t.y}`);
        //console.log("Move " + move);
        //console.log(flags);
        if (i == 493) {
            console.log(i);
            console.log("move " + move);
            console.log(h);
            console.log(rope);
        }
        // Move head
        if (move == "R")
            h.y++;
        if (move == "L")
            h.y--;
        if (move == "U")
            h.x++;
        if (move == "D")
            h.x--;
        // move tails
        rope.forEach((k) => {
            let p = k.p == 0 ? h : rope[k.p - 1];
            if (i == 492) {
                console.log("current: " + JSON.stringify(k));
                console.log("parent: " + JSON.stringify(p));
            }
            // Maybe move tail
            let dx = p.x - k.x;
            let dy = p.y - k.y;
            if (i == 492) {
                console.log("dx: " + JSON.stringify(dx));
                console.log("dy: " + JSON.stringify(dy));
            }
            // Diagonal
            if (Math.abs(dx) + Math.abs(dy) >= 3) {
                if (dx != 0)
                    k.x += dx / Math.abs(dx);
                if (dy != 0)
                    k.y += dy / Math.abs(dy);
            }
            // Simple
            else if (Math.abs(dx) == 2)
                k.x += dx / Math.abs(dx);
            else if (Math.abs(dy) == 2)
                k.y += dy / Math.abs(dy);
            if (k.p == 8)
                flags.add(`${k.x}::${k.y}`);
            if (i == 492) {
                console.log("current: " + JSON.stringify(k));
                console.log("parent: " + JSON.stringify(p));
                console.log("----");
            }
        });
    });
    console.log(rope);
    console.log("Part 2: " + flags.size);
}
function part1(moves) {
    //console.log(hmoves);
    let flags = new Set();
    let h = { x: 0, y: 0 };
    let t = { x: 0, y: 0 };
    [...moves].forEach((move) => {
        // Mark position of tail
        //console.log(`${h.x}::${h.y}`);
        flags.add(`${t.x}::${t.y}`);
        //console.log("Move " + move);
        //console.log(flags);
        // Move head
        if (move == "R")
            h.y++;
        if (move == "L")
            h.y--;
        if (move == "U")
            h.x++;
        if (move == "D")
            h.x--;
        // Maybe move tail
        let dx = h.x - t.x;
        let dy = h.y - t.y;
        // Diagonal
        if (Math.abs(dx) + Math.abs(dy) == 3) {
            t.x += dx / Math.abs(dx);
            t.y += dy / Math.abs(dy);
        }
        // Simple
        else if (Math.abs(dx) == 2)
            t.x += dx / Math.abs(dx);
        else if (Math.abs(dy) == 2)
            t.y += dy / Math.abs(dy);
    });
    console.log("Part 1: " + flags.size);
}
