<script>
  import Sidebar from "../components/Sidebar.svelte";
  import LeftFold from "../components/LeftFold.svelte";
  import Header from "../components/Header.svelte";
  import RightFold from "../components/RightFold.svelte";

  let requestUUid = "";
  let response = {};
</script>

<!-- <Header /> -->
<div class="container-fluid">
  <div class="row">
    <div class="col-2">
      <LeftFold bind:requestUUid bind:response />
    </div>
    <div class="col-10" >
      <RightFold {requestUUid} bind:response />
    </div>
  </div>
</div>

<style>
</style>
