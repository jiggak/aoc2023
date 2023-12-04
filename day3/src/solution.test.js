import { readInput } from './solution';

test('Test part1 sample data', async () => {
   const board = await readInput('data/sample.txt');
   expect(board.doTheThing()).toBe(_);
});

test('Test part1 input data', async () => {
   const board = await readInput('data/input.txt');
   expect(board.doTheThing()).toBe(_);
});

test('Test part2 sample data', async () => {
   const board = await readInput('data/sample.txt');
   expect(board.doTheThing({gearTest: true})).toBe(_);
});

test('Test part2 input data', async () => {
   const board = await readInput('data/input.txt');
   expect(board.doTheThing({gearTest: true})).toBe(_);
});
