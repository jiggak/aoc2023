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

    doTheThing() {
        let total = 0;

        for (let i = 0; i<this.data.length; i++) {
            const cell = this.data[i];

            if (cell.isNumber && this.cellHasAdjacentSymbol(cell)) {
                total += cell.getNumber();
                i = cell.lastDigit().index;
            }
        }

        return total;
    }

    cellHasAdjacentSymbol(cell) {
        // this function assumes cell is the first char of part number

        // top left coord of search
        const topLeft = this.indexToCoords(cell.index);
        topLeft.x --;
        topLeft.y --;

        // bottom right coord of search
        const bottomRight = this.indexToCoords(cell.lastDigit().index);
        bottomRight.x ++;
        bottomRight.y ++;

        return this.boxHasAdjacentSymbol(topLeft, bottomRight);
    }

    boxHasAdjacentSymbol(p1, p2) {
        const limitX = this.cols - 1;
        const limitY = this.rows - 1;

        let search = [];

        const xRange = range(p1.x, p2.x + 1 - p1.x);

        // generate search coords around rect from top left
        search.push(...xRange.map(x => new Coord(x, p1.y)));
        search.push(new Coord(p2.x, p1.y + 1));
        search.push(...xRange.map(x => new Coord(x, p2.y)));
        search.push(new Coord(p1.x, p1.y + 1));

        for (const coord of search) {
            if (coord.x < 0 || coord.x > limitX || coord.y < 0 || coord.y > limitY) {
                // skip if coord out of range
                continue;
            }

            const i = this.coordsToIndex(coord);
            if (this.data[i].isSymbol) {
                return true;
            }
        }

        return false;
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

    lastDigit() {
        if (!this.next) {
            return this;
        }

        let cursor = this.next;
        while (true) {
            if (cursor.next) {
                cursor = cursor.next;
            } else {
                return cursor;
            }
        }
    }

    getNumber() {
        if (!this.next) {
            return parseInt(this.content);
        }

        let number = this.content;
        let cursor = this.next;

        while (true) {
            number += cursor.content;

            if (cursor.next) {
                cursor = cursor.next;
            } else {
                return parseInt(number);
            }
        }
    }
}

class Coord {
    constructor(x, y) {
        this.x = x;
        this.y = y;
    }
}
