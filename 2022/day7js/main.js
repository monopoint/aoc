fs = require('fs');

var input;
readInput();

async function readInput(){
    fs.readFile("input", "utf-8", (err, data) => { 
        main(data);
    });
}


function main(input) {
    console.log(input);




}


