<script lang="ts">
    export let name!: string;
    export let indent!: number;
</script>

<div style:margin-left="{indent * 10}px">{name}</div>

<style>
    div {
        display: block;
        position: relative;
        /*width: 200px;*/
        height: 24px;
        margin-top: 1px;
        background-color: #aaccff44;
        font-size: 14px;
        color: #3333aa;
        margin-left: 100;
    }
</style>
