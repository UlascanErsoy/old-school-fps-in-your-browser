import * as wasm from "old-school-fps";
import { memory } from "old-school-fps/the_game_bg";

const canvas = document.getElementById("game-window");

const ctx = canvas.getContext("2d");

let game = wasm.Game.new();


async function draw_image(ctx, width, height) {
    const pixelPtr = game.render();
    const arr = new Uint8ClampedArray(memory.buffer, pixelPtr, width * 4 * height);
    
    //console.log("LEN:", arr);
    const img_data = new ImageData(arr, width, height);
    ctx.putImageData(img_data, 0,0);
}

function changeResolution(canvas, scaleFactor) {
    // Set up CSS size.
    canvas.style.width = canvas.style.width || canvas.width + 'px';
    canvas.style.height = canvas.style.height || canvas.height + 'px';

    // Resize canvas and scale future draws.
    canvas.width = Math.ceil(canvas.width * scaleFactor);
    canvas.height = Math.ceil(canvas.height * scaleFactor);
    var ctx = canvas.getContext('2d');
    ctx.scale(scaleFactor, scaleFactor);
}

addEventListener("keypress", (event) => {
    
    console.log(event.code);
    switch (event.code){
        case 'KeyW':
            game.update_player(0.1, 0, false);
            break;
        case 'KeyS':
            game.update_player(-0.1, 0, false);
            break;
        case 'KeyA':
            game.update_player(0.1, 0, true);
            break;
        case 'KeyD':
            game.update_player(-0.1, 0, true);
            break;
        case 'KeyQ':
            game.update_player(0.0, -3, false);
            break;
        case 'KeyE':
            game.update_player(-0.0, 3, false);
            break;
    }
});

async function render() {
    draw_image(ctx,320, 240);
    setTimeout(render, 10);
    //requestAnimationFrame(render);
}

changeResolution(canvas, 0.25); 
render();

