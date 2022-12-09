import * as fs from "fs";

fs.readFile("../input", "utf-8", (err, data) => {
  console.log(err ? err : "");
  main(data);
});

type Knot = {
  x: number;
  y: number;
  p?: number;
};
function main(input: String) {
  console.log("Hello AoC");
  //console.log(input);

  let hmoves = input
    .split("\n")
    .map((s) => {
      let m = s.split(" ")[0];
      let n: number = +s.split(" ")[1];
      //console.log("" + m + n);
      return Array(n + 1)
        .join(m)
        .split("");
    })
    .flat();

  //part1(hmoves);
  part2(hmoves);
}
function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function part2(moves: String[]) {
  console.log(moves);
  let flags = new Set();
  let h: Knot = { x: 0, y: 0 };

  let rope: Knot[] = [];
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
    if (move == "R") h.y++;
    if (move == "L") h.y--;
    if (move == "U") h.x++;
    if (move == "D") h.x--;

    // move tails

    rope.forEach((k) => {
      let p = k.p == 0 ? h : rope[k.p! - 1];
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
        if (dx != 0) k.x += dx / Math.abs(dx);
        if (dy != 0) k.y += dy / Math.abs(dy);
      }

      // Simple
      else if (Math.abs(dx) == 2) k.x += dx / Math.abs(dx);
      else if (Math.abs(dy) == 2) k.y += dy / Math.abs(dy);

      if (k.p == 8) flags.add(`${k.x}::${k.y}`);
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

function part1(moves: String[]) {
  //console.log(hmoves);
  let flags = new Set();
  let h: Knot = { x: 0, y: 0 };
  let t: Knot = { x: 0, y: 0 };

  [...moves].forEach((move) => {
    // Mark position of tail
    //console.log(`${h.x}::${h.y}`);
    flags.add(`${t.x}::${t.y}`);
    //console.log("Move " + move);
    //console.log(flags);

    // Move head
    if (move == "R") h.y++;
    if (move == "L") h.y--;
    if (move == "U") h.x++;
    if (move == "D") h.x--;

    // Maybe move tail
    let dx = h.x - t.x;
    let dy = h.y - t.y;

    // Diagonal
    if (Math.abs(dx) + Math.abs(dy) == 3) {
      t.x += dx / Math.abs(dx);
      t.y += dy / Math.abs(dy);
    }

    // Simple
    else if (Math.abs(dx) == 2) t.x += dx / Math.abs(dx);
    else if (Math.abs(dy) == 2) t.y += dy / Math.abs(dy);
  });

  console.log("Part 1: " + flags.size);
}
