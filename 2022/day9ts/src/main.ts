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
  let hmoves = input
    .split("\n")
    .map((s) => {
      let m = s.split(" ")[0];
      let n: number = +s.split(" ")[1];
      return Array(n + 1)
        .join(m)
        .split("");
    })
    .flat();
  part1(hmoves);
  part2(hmoves);
}

function part2(moves: String[]) {
  let flags = new Set();
  let h: Knot = { x: 0, y: 0 };

  let rope: Knot[] = [];
  while (rope.length < 9) rope.push({ x: 0, y: 0, p: rope.length });

  let i = 0;
  [...moves].forEach((move) => {
    if (move == "R") h.y++;
    if (move == "L") h.y--;
    if (move == "U") h.x++;
    if (move == "D") h.x--;

    rope.forEach((k) => {
      let p = k.p == 0 ? h : rope[k.p! - 1];
      let dx = p.x - k.x;
      let dy = p.y - k.y;

      if (Math.abs(dx) + Math.abs(dy) >= 3) {
        if (dx != 0) k.x += dx / Math.abs(dx);
        if (dy != 0) k.y += dy / Math.abs(dy);
      } else if (Math.abs(dx) == 2) k.x += dx / Math.abs(dx);
      else if (Math.abs(dy) == 2) k.y += dy / Math.abs(dy);

      if (k.p == 8) flags.add(`${k.x}::${k.y}`);
    });
  });

  console.log("Part 2: " + flags.size);
}

function part1(moves: String[]) {
  let flags = new Set();
  let h: Knot = { x: 0, y: 0 };
  let t: Knot = { x: 0, y: 0 };

  [...moves].forEach((move) => {
    flags.add(`${t.x}::${t.y}`);

    if (move == "R") h.y++;
    if (move == "L") h.y--;
    if (move == "U") h.x++;
    if (move == "D") h.x--;

    let dx = h.x - t.x;
    let dy = h.y - t.y;

    if (Math.abs(dx) + Math.abs(dy) == 3) {
      t.x += dx / Math.abs(dx);
      t.y += dy / Math.abs(dy);
    } else if (Math.abs(dx) == 2) t.x += dx / Math.abs(dx);
    else if (Math.abs(dy) == 2) t.y += dy / Math.abs(dy);
  });

  console.log("Part 1: " + flags.size);
}
