import * as fs from 'fs';
import * as readline from 'readline';

export async function readInput(filePath) {
    const rl = readline.createInterface({
        input: fs.createReadStream(filePath)
    });

    let rows = 0, cols = 0, data = [];

    for await (const line of rl) {
        if (cols == 0) {
            cols = line.length;
        }

        for (const c of [...line]) {
            data.push(new Cell(data.length, c, data.at(-1)));
        }

        rows ++;
    }

    return new Board(data, rows, cols);
}

function range(start, length) {
    return [...Array(length).keys()].map(x => x+start);
}

export class Board {
    constructor(data, rows, cols) {
        this.data = data;
        this.rows = rows;
        this.cols = cols;
    }

    doTheThing(gearTest = false) {
        let total = 0;

        for (const cell of this.data) {
            if (cell.isSymbol) {
                const numbers = this.adjacentNumbers(cell);
                if (!gearTest) {
                    numbers.forEach(num => total += num);
                } else {
                    if (cell.content == '*' && numbers.length == 2) {
                        total += numbers[0] * numbers[1];
                    }
                }
            }
        }

        return total;
    }

    // FIXME this doesn't account for numbers that are adjacent to multiple
    // symbols, it will include those numbers twice in the results.
    // This doesn't seem to matter with the problem input data, but it bugs me.
    adjacentNumbers(cell) {
        const coord = this.indexToCoords(cell.index);

        const limitX = this.cols - 1;
        const limitY = this.rows - 1;

        let search = [];

        const xRange = range(coord.x - 1, 3);

        // generate search coords around rect from top left
        search.push(...xRange.map(x => new Coord(x, coord.y - 1)));
        search.push(new Coord(coord.x + 1, coord.y));
        search.push(...xRange.map(x => new Coord(x, coord.y + 1)));
        search.push(new Coord(coord.x - 1, coord.y));

        let result = new Set();

        for (const coord of search) {
            if (coord.x < 0 || coord.x > limitX || coord.y < 0 || coord.y > limitY) {
                // skip if coord out of range
                continue;
            }

            const i = this.coordsToIndex(coord);
            if (this.data[i].isNumber) {
                result.add(this.data[i].number());
            }
        }

        return Array.from(result);
    }

    indexToCoords(index) {
        return new Coord(
            index % this.cols, // col (X)
            Math.floor(index / this.cols) // row (Y)
        );
    }

    coordsToIndex(coords) {
        return coords.y * this.cols + coords.x;
    }
}

const symbols = [
    '`', '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '_', '-', '+', '=',
    '[', ']', '{', '}', ';', "'", ':', '"', '\\', '|', ',', '<', '>', '/', '?', '~'
];

class Cell {
    constructor(index, content, previous) {
        this.index = index;
        this.content = content;
        this.isSymbol = symbols.indexOf(content) != -1;
        this.isNumber = !this.isSymbol && content != '.';

        this.previous = null;
        this.next = null;

        if (this.isNumber && previous && previous.isNumber) {
            this.previous = previous;
            this.previous.next = this;
        }
    }

    number() {
        let cursor = this;
        while (true) {
            if (cursor.previous) {
                cursor = cursor.previous;
            } else {
                break;
            }
        }

        let number = '';

        while (true) {
            number += cursor.content;

            if (cursor.next) {
                cursor = cursor.next;
            } else {
                break;
            }
        }

        return parseInt(number);
    }
}

class Coord {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}
