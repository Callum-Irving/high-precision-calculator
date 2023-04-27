<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";

  let ctx;
  let stmt = "";
  let result = "";

  let cells = [{
    stmt: "",
    result: "",
  }];

  async function initContext() {
    ctx = await invoke("create_context");
  }

  async function evalStmt(stmt) {
    let rust_ret = await invoke("tauri_eval_stmt", { stmt, ctx});
    ctx = rust_ret[1];

    let result = rust_ret[0];
    console.log(result);
    return result;
  }

  function newCell() {
    cells = [...cells, {stmt: "", result: ""}];
  }
</script>

<div>
  {#await initContext()}
    <p>Initializing context...</p>
  {:then}
    {#each cells as cell, index}
      <div class="row">
        <input bind:value={cell.stmt} />
        <button on:click={async () => cell.result = await evalStmt(cell.stmt)}>
          Eval
        </button>
        <p>Result: {cell.result}</p>
      </div>
    {/each}
    <button on:click={newCell}>+</button>
  {/await}
</div>