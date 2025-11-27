<script lang="ts">
  export let isVisible: boolean;
  export let isActive: boolean;
  export let labelName: string;
</script>

{#if isVisible}
  <div class="item" data--active={isActive}>{labelName}</div>
{/if}

<style>
  .item {
    display: inline-block;
    position: relative;
    height: 30px;
    margin: 5px 0 0 4px;
    background-color: rgba(68, 202, 255, 0.157);
    font-size: 20px;
    font-weight: 600;
    color: rgba(255, 51, 51, 0.285);
    padding: 0 30px;
    border-radius: 4px;
  }
  .item[data--active="true"] {
    color: #f33;
  }
</style>
