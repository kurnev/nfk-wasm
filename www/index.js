// Import the WebAssembly memory at the top of the file.
import { memory } from "nfk-wasm/nfk_bg";
import { GlobalState } from "nfk-wasm";

const state = GlobalState.new();
state.spawn_hero();

const canvas = document.getElementById("map");
var ctx = canvas.getContext("2d");

const renderLoop = () => {
  state.tick();
  draw();
  requestAnimationFrame(renderLoop);
};

ctx.fillStyle = getRandomColor();
const oldColor = ctx.fillStyle;

const draw = () => {
  const ptr = state.blocks_ptr();
  const cells = new Uint32Array(memory.buffer, ptr, 8000);

  ctx.beginPath();
  for (let i = 0; i < cells.length; i += 4) {
    if (cells[i + 2] && cells[i + 3]) {
      if (i === 4000) {
        ctx.fillStyle = "red";
      }
      ctx.fillRect(cells[i], cells[i + 1], cells[i + 2], cells[i + 3]);
      if (i === 4000) {
        ctx.fillStyle = oldColor;
      }
    }
  }
  ctx.stroke();
};

function getRandomColor() {
  var letters = "0123456789ABCDEF";
  var color = "#";
  for (var i = 0; i < 6; i++) {
    color += letters[Math.floor(Math.random() * 16)];
  }
  return color;
}

renderLoop();
