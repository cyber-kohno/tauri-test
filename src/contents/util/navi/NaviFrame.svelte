<script lang="ts">
  import store from "../../store/store";

    $:  phase = $store.phase;
</script>

<div class="item" data--active={phase === 'listup'}>Scan</div>
<div class="item" data--active={phase === 'revalidate'}>Revalidate</div>

<style>
    .item {
        display: inline-block;
        position: relative;
        height: 30px;
        margin: 5px 0 0 4px;
        background-color: #a99;
        font-size: 20px;
        font-weight: 600;
        color: #fff;
        padding: 0 20px;
        border-radius: 4px;
    }
    .item[data--active=true] {
        color: #f33;
    }
</style>
