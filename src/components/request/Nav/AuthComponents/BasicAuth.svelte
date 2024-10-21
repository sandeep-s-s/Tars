<!-- ChildComponent2.svelte -->
<script>
  export let auth;
</script>

<div class="row mt-2">
  <div class="col">
    <label for="userName" class="form-label">User Name</label>
    <input type="text" placeholder="Username" bind:value={auth.username} class="form-control"/>
  </div>
  <div class="col">
    <label for="password" class="form-label">Password</label>
    <input type="text" placeholder="Password" bind:value={auth.password} class="form-control"/>
  </div>
</div>
