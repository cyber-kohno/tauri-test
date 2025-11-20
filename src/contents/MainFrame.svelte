<script lang="ts">
  import { onMount } from "svelte";
  import NaviFrame from "./util/NaviFrame.svelte";
  import store from "./store/store";
  import RevalidateFrame from "./phase/revalidate/RevalidateFrame.svelte";
  import ListupFrame from "./phase/listup/ListupFrame.svelte";

  onMount(() => {
    const handler = (e: MouseEvent) => {
      e.preventDefault();
    };
    window.addEventListener("contextmenu", handler);

    // クリーンアップ（コンポーネントが破棄されるとき）
    return () => {
      window.removeEventListener("contextmenu", handler);
    };
  });

  $: phase = $store.phase;
</script>

<div class="header">
  <NaviFrame />
</div>
<div class="main">
  {#if phase === "listup"}
    <ListupFrame />
  {:else if phase === "revalidate"}
    <RevalidateFrame />
  {/if}
</div>

<style>
  .header {
    display: inline-block;
    position: relative;
    width: 100%;
    height: 40px;
    background-color: #fff;
  }
  .main {
    display: inline-block;
    position: relative;
    width: 100%;
    height: calc(100% - 40px);
    background-color: white;
  }
</style>
