<script>
  import Sidebar from "../components/Sidebar.svelte";
  import LeftFold from "../components/LeftFold.svelte";
  import Header from "../components/Header.svelte";
  import RightFold from "../components/RightFold.svelte";


</script>

<Header />
<div class="container">
  <div class="sidebar">
    <Sidebar />
  </div>
  <div class="leftFold">
    <LeftFold />
  </div>
  <div class="rightFold">
    <RightFold />
  </div>
</div>

<style>
  * {
    padding: 0;
    margin: 0;
  }
  .container {
    display: flex;
    height: 100vh;
    /* width: 100%; */
  }
  .sidebar {
    display: flex;
    flex: 1;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    border-right: 1px solid whitesmoke;
    background-color: #eaf1fb;
  }
  .leftFold {
    display: flex;
    flex: 3;
    flex-direction: column;
    align-items: flex-start;
    border-right: 1px solid whitesmoke;
  }
  .rightFold {
    /* display: flex; */
    flex: 11;
    /* align-items: center; */
    /* justify-content: flex-start; */
    border-right: 1px solid whitesmoke;
  }
</style>
