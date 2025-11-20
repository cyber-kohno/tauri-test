<script lang="ts">
  import store from "../../store/store";
  import ChoosePhase from "./choose/ChoosePhase.svelte";
  import ScanCondPhase from "./scan/ScanCondPhase.svelte";
</script>

<div class="wrap">
  <div class="left">
    <ScanCondPhase />
  </div>
  <div class="right">
    {#if $store.resultTree != undefined}
      <ChoosePhase root={$store.resultTree} />
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
