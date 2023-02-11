<script lang="ts">
  import type {InitOutput} from "../rust-src/pkg";

  export let conway: InitOutput;
</script>


<button on:click={conway.greet}>
  Show alert
</button>
