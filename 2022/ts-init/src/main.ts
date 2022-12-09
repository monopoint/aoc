import * as fs from "fs";

fs.readFile("input", "utf-8", (err, data) => {
  console.log(err ? err : "");
  main(data);
});

function main(input: String) {
  console.log("Hello AoC");
  console.log(input);
}
