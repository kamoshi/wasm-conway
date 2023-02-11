<script lang="ts">
  import type {InitOutput, Universe as _U, Cell as _C} from "../rust-src/pkg";
  import {onMount} from "svelte";
  export let conway: InitOutput;
  export let Universe: typeof _U;
  export let Cell: typeof _C;
  let canvas: HTMLCanvasElement;

  const CELL_SIZE = 5; //px
  const GRID_COLOR = "#CCC";
  const DEAD_COLOR = "#FFF";
  const LIVE_COLOR = "#000";

  const space = Universe.new(64, 64);
  const cols = space.cols();
  const rows = space.rows();

  function getIndex(row, col) {
    return row * cols + col;
  }

  onMount(() => {
    const ctx = canvas.getContext('2d');

    function drawGrid() {
      ctx.beginPath();
      ctx.strokeStyle = GRID_COLOR;

      // vertical lines
      for (let i = 0; i <= cols; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * rows + 1);
      }

      // horizontal lines
      for (let j = 0; j <= rows; j++) {
        ctx.moveTo(0, j * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * cols + 1, j * (CELL_SIZE + 1) + 1);
      }

      ctx.stroke();
    }

    function drawCells() {
      const cellsPtr = space.cells_ptr();
      const cells = new Uint8Array(conway.memory.buffer, cellsPtr, cols * rows);

      ctx.beginPath();

      for (let row = 0; row < rows; row++) {
        for (let col = 0; col < cols; col++) {
          const idx = getIndex(row, col);

          ctx.fillStyle = cells[idx] === Cell.Dead ? DEAD_COLOR : LIVE_COLOR;

          ctx.fillRect(
            col * (CELL_SIZE + 1) + 1,
            row * (CELL_SIZE + 1) + 1,
            CELL_SIZE,
            CELL_SIZE
          );
        }
      }

      ctx.stroke();
    }

    function renderLoop() {
      space.tick();
      drawGrid();
      drawCells();
      requestAnimationFrame(renderLoop);
    }

    requestAnimationFrame(renderLoop);
  });
</script>


<canvas id="conway" bind:this={canvas}
        height={(CELL_SIZE + 1) * rows + 1}
        width={(CELL_SIZE + 1) * cols + 1}>
</canvas>

