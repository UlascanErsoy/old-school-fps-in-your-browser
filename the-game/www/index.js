import * as wasm from "old-school-fps";
import { memory } from "old-school-fps/the_game_bg";

const canvas = document.getElementById("game-window");

const ctx = canvas.getContext("2d");

function draw_image(ctx, width, height) {
    const pixelPtr = wasm.draw_image_data(width, height);
    const arr = new Uint8ClampedArray(memory.buffer, pixelPtr, width * 4 * height);
    
    console.log("LEN:", arr);
    const img_data = new ImageData(arr, width, height);

    ctx.putImageData(img_data, 0,0);
}

draw_image(ctx,100, 100);
