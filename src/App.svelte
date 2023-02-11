<script lang="ts">
  import init, { Universe, Cell } from "rust-src";
  import Board from "./Board.svelte";

  const wasm = init();
</script>

<main>
  {#await wasm}
    Loading data...
  {:then conway}
    <Board {conway} {Universe} {Cell} />
  {/await}
</main>
