import { memory } from "sandpiles/sandpiles_bg";
import { World } from "sandpiles";

/* Settings */

const pixSize = 2;
const stepsPerFrame = 100;
const colors = ['#000', '#FF0', '#F80', '#F00', '#0FF', '#08F', '#00F'];
const canvas = document.querySelector('canvas');

/* Setup */

// Width & height
const width =  Math.round(window.innerWidth  / pixSize);
const height = Math.round(window.innerHeight / pixSize);
// Canvas
canvas.width =  width  * pixSize;
canvas.height = height * pixSize;
const ctx = canvas.getContext('2d');
// World
const world = World.new(width, height);

/* Functions */

const getIndex = (row, col) => {
    return row * width + col;
};

const renderCanvas = () => {
    const ptr = world.get_cells_ptr();
    const cells = new Uint32Array(memory.buffer, ptr, width * height);

    for (let row = 0; row < height; row++) {
        for (let col = 0; col < width; col++) {
            const cell = cells[getIndex(row, col)];
            
            ctx.fillStyle = cell < colors.length ? colors[cell] : '#0F0';
            ctx.fillRect(col * pixSize, row * pixSize, pixSize, pixSize);
        }
    }
}

const updateLoop = () => {
    renderCanvas();
    console.log(world.get_iteration());

    if (world.compute_steps(stepsPerFrame)) {
        requestAnimationFrame(updateLoop);
    }
};

/* Run The Things!! */

requestAnimationFrame(updateLoop);
