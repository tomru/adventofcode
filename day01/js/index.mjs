import * as readline from 'node:readline';
import * as fs from 'node:fs';


const data = [];
const rl = readline.createInterface({ 
    input: fs.createReadStream('./input') 
});

for await (const line of rl) {
    data.push(parseInt(line, 10));
}


let last = null;
let part1 = 0;

for (const v in data) {
    if (last !== null && v > last) counter++;
    last = v;    
}

console.log('part1', part1);

