<script>
  import Sidebar from "../components/Sidebar.svelte";
  import LeftFold from "../components/LeftFold.svelte";
  import Header from "../components/Header.svelte";
  import RightFold from "../components/RightFold.svelte";
  import Toast from "../Util/Toast.svelte";

  let requestUUid = "";
  let response = {};
  
  function refreshTabs() {
    const event = new CustomEvent('tabsRefresh');
    window.dispatchEvent(event);
  }
</script>

<div class="container-fluid">
  <div class="row">
    <div class="col-2">
      <LeftFold bind:requestUUid bind:response on:refreshTabs={refreshTabs} />
    </div>
    <div class="col-10">
      <RightFold {requestUUid} bind:response />
    </div>
  </div>
</div>
<Toast />
