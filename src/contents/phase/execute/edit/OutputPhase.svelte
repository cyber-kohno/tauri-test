<script lang="ts">
  import store from "../../../store/store";
  import OperationButton from "../../../util/OperationButton.svelte";
  import SyntaxHighlighter from "../../../util/SyntaxHighlighter.svelte";

  $: req = (() => {
    const req = $store.executeReq;
    if (req == undefined) throw new Error();
    return req;
  })();

  $: cancel = () => {
    req.output = '';
    $store.executeReq = {...req};
    // $store.phase = "revalidate";
  };
</script>

<div class="main">
  <textarea
    readonly
    value={req.output}
  ></textarea>
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
    height: calc(100% - 32px);
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
</style>
