<script lang="ts">
  import { writable } from "svelte/store";
  import store from "../../store/store";

  $: targets = $store.rvReq.targets;
</script>

<div class="wrap">
  <div class="left">
    <textarea
      value={targets}
      oninput={(e) => {
        $store.rvReq.targets = e.currentTarget.value;
      }}
    ></textarea>
  </div>
  <div class="right"></div>
</div>

<style>
  .wrap {
    display: inline-block;
    position: relative;
    background-color: #fec;
    width: calc(100% - 8px);
    height: calc(100% - 8px);
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
    background-color: rgb(211, 223, 236);
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
  .right {
    background-color: #eff;
  }
</style>
