<script lang="ts">
  import store from "../../../store/store";
  import DevelopPhase from "./DevelopPhase.svelte";
  import OutputPhase from "./OutputPhase.svelte";
</script>

<div class="wrap">
  <div class="left">
    <DevelopPhase />
  </div>
  <div class="right">
    {#if $store.executeRes != undefined}
      <OutputPhase />
    {/if}
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
