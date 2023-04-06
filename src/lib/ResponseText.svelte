<script lang="ts">
  export let text: string = '';
</script>

<p class="text-cat-text">
  {#if text}
    {text}
  {:else}
    No text to display.
  {/if}
</p>
