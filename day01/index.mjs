import * as readline from 'node:readline';
import * as fs from 'node:fs';

const rl = readline.createInterface({ 
    input: fs.createReadStream('./input') 
});

let last = null;
let counter = 0;

for await (const line of rl) {
    const v = parseInt(line, 10);
    if (last !== null && v > last) counter++;
    last = v;    
}

console.log(counter);

