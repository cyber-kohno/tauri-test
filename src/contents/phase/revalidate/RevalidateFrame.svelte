<script lang="ts">
  import RvResultPhase from "./RvResultPhase.svelte";
  import RvTargetPhase from "./RvTargetPhase.svelte";

</script>

<div class="wrap">
  <div class="left">
    <RvTargetPhase />
  </div>
  <div class="right">
    <RvResultPhase />
  </div>
</div>

<style>
  .wrap {
    display: inline-block;
    position: relative;
    background-color: rgb(55, 237, 0);
    width: 100%;
    height: 100%;
    > .left,
    .right {
      display: inline-block;
      position: relative;
      vertical-align: top;
      width: 50%;
      height: 100%;
    }
  }
  .left {
    background-color: rgb(211, 223, 236);
  }
  .right {
    background-color: #eff;
  }
</style>
