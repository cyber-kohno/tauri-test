<script lang="ts">
  import store from "../../../store/store";
  import OperationButton from "../../../util/OperationButton.svelte";
  import SyntaxHighlighter from "../../../util/SyntaxHighlighter.svelte";

  $: totalCnt = (() => {
    const req = $store.executeReq;
    if (req == undefined) throw new Error();
    return req.files.length;
  })();
  $: res = (() => {
    const res = $store.executeRes;
    if (res == undefined) throw new Error();
    return res;
  })();

  $: cancel = () => {
    $store.executeRes = undefined;
  };
</script>

<div class="operation-div">
  <div class="progress-frame">
    <div class="progress-inner" style:width="{Math.floor(res.endCnt / totalCnt) * 100}%"></div>
  </div>
</div>
<div class="main">
  <textarea readonly value={res.output}></textarea>
  <!-- <SyntaxHighlighter code={req.output} /> -->
</div>
<div class="operation-div">
  <OperationButton name={"Cancel"} width={160} callback={cancel} />
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
    color: #001d86;
  }
  .operation-div {
    display: inline-block;
    position: relative;
    width: 100%;
    height: 32px;
    background-color: #8888aa44;
    text-align: right;
  }
  .progress-frame {
    display: inline-block;
    position: absolute;
    left: 6px;
    top: 6px;
    width: calc(100% - 12px);
    height: calc(100% - 12px);
    background-color: #ffffff44;
    border: 2px solid rgb(188, 0, 0);
    box-sizing: border-box;
    text-align: left;
    border-radius: 4px;
  }
  .progress-inner {
    display: inline-block;
    position: relative;
    height: 100%;
    background-color: rgb(255, 101, 101);
    border-radius: 4px;
  }
</style>
