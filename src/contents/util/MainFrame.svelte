<script lang="ts">
    import Phase from "./Phase.svelte";
    import TargetFilter from "./target/TargetFilter.svelte";
</script>

<div class="header">
    <Phase />
</div>
<div class="main"><TargetFilter /></div>

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
