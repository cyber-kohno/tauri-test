<script lang="ts">
  import store from "../../store/store";
  import OperationButton from "../../util/OperationButton.svelte";

  $: targets = $store.rvReq.targets;

  $: cancel = () => {
    $store.phase = "listup";
  };
  $: test = () => {};
</script>

<div class="main">
  <textarea
    value={targets}
    oninput={(e) => {
      $store.rvReq.targets = e.currentTarget.value;
    }}
  ></textarea>
</div>
<div class="operation-div">
  <OperationButton name={"Reset"} width={160} callback={cancel} />
  <OperationButton name={"Test"} width={160} callback={test} />
</div>

<style>
  .main {
    display: inline-block;
    position: relative;
    width: 100%;
    height: calc(100% - 60px);
    background-color: white;
  }
  textarea {
    display: inline-block;
    position: relative;
    width: 100%;
    height: 100%;
    resize: none;
    background-color: #fff;
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
</style>
