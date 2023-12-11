import { readInput } from './solution';
import * as fs from 'fs';

const tests = JSON.parse(fs.readFileSync('../tests/day3/tests.json'));

test('Test part1 sample data', async () => {
   const board = await readInput('../tests/day3/sample.txt');
   expect(board.doTheThing()).toBe(tests['sample.txt'].part1);
});

test('Test part1 input data', async () => {
   const board = await readInput('../tests/day3/input.txt');
   expect(board.doTheThing()).toBe(tests['input.txt'].part1);
});

test('Test part2 sample data', async () => {
   const board = await readInput('../tests/day3/sample.txt');
   expect(board.doTheThing({gearTest: true})).toBe(tests['sample.txt'].part2);
});

test('Test part2 input data', async () => {
   const board = await readInput('../tests/day3/input.txt');
   expect(board.doTheThing({gearTest: true})).toBe(tests['input.txt'].part2);
});
