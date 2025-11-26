<script lang="ts">
  import store from "../../store/store";
  import NaviItem from "./NaviItem.svelte";

  $: phase = $store.phase;

  $: isScanEnd = (() => {
    return $store.resultTree != undefined;
  })();
  $: isChooseEnd = (() => {
    return $store.rvReq != undefined;
  })();
  $: isRevalidateEnd = (() => {
    return $store.executeReq != undefined;
  })();
  $: isDevelopEnd = (() => {
    return false;
  })();
</script>

<div class="item" data--active={phase === "listup"}>Scan</div>
<div class="item" data--active={phase === "revalidate"}>Revalidate</div>

<NaviItem labelName={"Scan"} isVisible={true} isActive={!isScanEnd} />
<NaviItem
  labelName={"Choose"}
  isVisible={isScanEnd}
  isActive={isScanEnd && !isChooseEnd}
/>
<NaviItem
  labelName={"Revalidate"}
  isVisible={isChooseEnd}
  isActive={isChooseEnd && !isRevalidateEnd}
/>
<NaviItem labelName={"Develop"} isVisible={isRevalidateEnd} isActive={!isDevelopEnd} />
<NaviItem labelName={"Result"} isVisible={isDevelopEnd} isActive={isDevelopEnd} />
