import * as process from 'process';
import { readInput } from './solution.js';

if (process.argv.length != 3) {
    console.error(`${process.argv[1]} [input.txt]`);
    process.exit(1);
}

readInput(process.argv[2])
    .then(board => {
        const total = board.doTheThing();
        console.log(total);
    })
    .catch(e => console.error('error', e));
