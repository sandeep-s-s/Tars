<script>
  import { invoke } from "@tauri-apps/api/core";
  export let request = {};
  export let requestUUid = "";
  export let response = {};

  let loading = false;

  let methods = [
    { id: 1, text: "GET", default: true },
    { id: 2, text: "POST", default: false },
    { id: 3, text: "PUT", default: false },
  ];

  export let method = request["method"];
  export let endpoint = request["endpoint"];

  async function saveRequest(uuid) {
    request = JSON.stringify(request);
    let response = await invoke("save_request", { uuid, request });
  }
  async function sendRequest(request) {
    loading = true;
    // console.log(request)
    request = JSON.stringify(request);
    response = await invoke("send_request", { request });
    // console.log(response)
    loading = false;
  }
</script>

<!-- <form on:submit|preventDefault={sendPostRequest}> -->
<div class="request-top-container">
  <div class="request-select">
    <select bind:value={method} class="form-control">
      {#each methods as method}
        <option value={method.text}>
          {method.text}
        </option>
      {/each}
    </select>
  </div>
  <div class="request-url">
    <input bind:value={endpoint} class={"form-control"} placeholder="URL" />
  </div>
  <div class="request-button">
    <button
      type="submit"
      class="btn btn-dark btn-sm"
      on:click={() => sendRequest(request)}
      ><i class="bi bi-send-fill"></i> Send</button
    >
  </div>
  <div class="request-button">
    <button
      type="submit"
      class="btn btn-outline-dark btn-sm"
      on:click={() => saveRequest(requestUUid)}
      ><i class="bi bi-floppy-fill"></i> Save</button
    >
  </div>
</div>
{#if loading}
  <p>Loading...</p>
  <!-- Show loading message -->
{/if}

<style>
  .request-top-container {
    display: flex;
    gap: 30px;
  }
</style>
