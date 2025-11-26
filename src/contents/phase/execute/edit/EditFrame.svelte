<script lang="ts">
  import DevelopPhase from "./DevelopPhase.svelte";
  import OutputPhase from "./OutputPhase.svelte";

</script>

<div class="wrap">
  <div class="left">
    <DevelopPhase />
  </div>
  <div class="right">
    <OutputPhase />
  </div>
</div>

<style>
  .wrap {
    display: inline-block;
    position: relative;
    background-color: #fec;
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
    background-color: #eff;
  }
  .right {
    background-color: #eff;
  }
</style>
