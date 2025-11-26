<script lang="ts">
  import store from "../../store/store";
  import OperationButton from "../../util/OperationButton.svelte";

  $: req = (() => {
    const req = $store.rvReq;
    if (req == undefined) throw new Error();
    return req;
  })();

  $: cancel = () => {
    $store.rvReq = undefined;
    $store.phase = "listup";
  };
  $: test = () => {};

  $: rootPath = $store.scanRequest.rootPath;
</script>

<!-- ルートパス -->
<div class="fixed-label">{rootPath}</div>
<div class="main">
  <textarea
    value={req.targets}
    oninput={(e) => {
      req.targets = e.currentTarget.value;
    }}
  ></textarea>
</div>
<div class="operation-div">
  <OperationButton name={"Cancel"} width={160} callback={cancel} />
  <OperationButton name={"Test"} width={160} callback={test} />
</div>

<style>
  .main {
    display: inline-block;
    position: relative;
    width: 100%;
    height: calc(100% - 64px);
    background-color: #d3d3d3;
  }
  textarea {
    display: inline-block;
    position: relative;
    margin: 4px 0 0 4px;
    width: calc(100% - 8px);
    height: calc(100% - 8px);
    background-color: white;
    resize: none;
    outline: none;
    box-sizing: border-box;
    border: none;
    font-size: 18px;
    color: #000;
  }
  .operation-div {
    display: inline-block;
    position: relative;
    width: 100%;
    height: 32px;
    background-color: #8888aa44;
    text-align: right;
  }
  .fixed-label {
    display: inline-block;
    position: relative;
    width: 100%;
    height: 32px;
    background-color: #f4f4ffd7;
    box-sizing: border-box;
    color: rgb(34, 74, 177);
    font-style: italic;
    font-size: 18px;
    line-height: 28px;
    font-weight: 400;
    text-align: left;
    padding: 0 0 0 4px;
    white-space: nowrap;
  }
</style>
