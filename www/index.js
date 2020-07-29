// Import the WebAssembly memory at the top of the file.
import { memory } from "nfk-wasm/nfk_bg";
import { GlobalState } from "nfk-wasm";

const state = GlobalState.new();
state.spawn_hero();

const canvasMap = document.getElementById("map");
const ctxMap = canvasMap.getContext("2d");
const canvasHero = document.getElementById("hero_1");
const ctxHero = canvasHero.getContext("2d");

ctxHero.fillStyle = "red";

const renderLoop = () => {
  state.tick();
  draw();
  drawHero();
  requestAnimationFrame(renderLoop);
};

ctxMap.fillStyle = getRandomColor();
const oldColor = ctxMap.fillStyle;

const draw = () => {
  const ptr = state.blocks_ptr();
  const cells = new Uint32Array(memory.buffer, ptr, 4000);

  ctxMap.beginPath();

  for (let i = 0; i < cells.length; i += 4) {
    if (cells[i + 2] && cells[i + 3]) {
      ctxMap.fillRect(cells[i], cells[i + 1], cells[i + 2], cells[i + 3]);
      if (i === 4000) {
        ctxMap.fillStyle = oldColor;
      }
    }
  }
  ctxMap.stroke();
};

const drawHero = () => {
  const ptr = state.heroes_ptr();
  const cells = new Uint32Array(memory.buffer, ptr, 4000);

  // clean prevois frame
  ctxHero.clearRect(0, 0, canvasHero.width, canvasHero.height);

  ctxHero.beginPath();
  const i = 0;

  ctxHero.fillRect(cells[i], cells[i + 1], cells[i + 2], cells[i + 3]);
  ctxHero.stroke();
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
