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

  $: rootPath = $store.scanRequest.rootPath;
  $: transfer = () => {
    $store.executeReq = {
        files: req.targets.split('\n').map(t => `${rootPath}\\${t}`),
        funcSource: '',
        output: ''
    };
    $store.phase = 'edit';
  };
</script>

<div class="main">
</div>
<div class="operation-div">
  <OperationButton name={"Cancel"} width={160} callback={cancel} />
  <OperationButton name={"Regist"} width={160} callback={transfer} />
</div>

<style>
  .main {
    display: inline-block;
    position: relative;
    width: 100%;
    height: calc(100% - 32px);
    background-color: #d3e2ed;
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
