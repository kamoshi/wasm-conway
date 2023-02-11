<script lang="ts">
  import init from "rust-src";
  import Board from "./Board.svelte";

  const wasm = init();
</script>

<main>
  {#await wasm}
    Loading data...
  {:then conway}
    <Board {conway} />
  {/await}
</main>
