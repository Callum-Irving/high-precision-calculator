<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri"

  let ctx = 0;
  let stmt = "";
  let result = "";

  async function initContext() {
    ctx = await invoke("create_context");
  }

  async function evalStmt() {
    let rust_ret = await invoke("tauri_eval_stmt", { stmt, ctx});
    result = rust_ret[0];
    ctx = rust_ret[1];
  }
</script>

<div>
  {#await initContext()}
    <p>Initializing context...</p>
  {:then}
    <input id="calc-input" bind:value={stmt} />
    <button on:click={evalStmt}>
      Evaluate
    </button>
    <p>{result}</p>
  {/await}
</div>