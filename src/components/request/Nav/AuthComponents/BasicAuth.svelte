<!-- ChildComponent2.svelte -->
<script>
  export let auth;
</script>

<div class="basic-auth-container">
  <div class="col">
    <label for="userName" class="form-label"><strong>User Name</strong></label>
    <input
      type="text"
      placeholder="Username"
      bind:value={auth.username}
      class="form-control"
    />
  </div>
  <div class="col">
    <label for="password" class="form-label"><strong>Password</strong></label>
    <input
      type="text"
      placeholder="Password"
      bind:value={auth.password}
      class="form-control"
    />
  </div>
</div>

<style>
  .basic-auth-container {
    display: flex;
    gap: 12px;
  }
</style>
