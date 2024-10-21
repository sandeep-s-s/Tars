<!-- ChildComponent3.svelte -->
<script>
  export let auth;
</script>

<div class="row">
  <div class="col-10">
    <label for="token" class="form-label">Token</label>
    <input type="text" placeholder="Token" bind:value={auth.token} class="form-control"/>
  </div>
</div>
