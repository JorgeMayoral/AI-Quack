<script lang="ts">
  import ResponseText from './ResponseText.svelte';

  export let responses: string[] = [];
</script>

<section class="overflow-scroll">
  {#each responses as response}
    <ResponseText text={response} />
  {:else}
    <ResponseText text="No text to display." />
  {/each}
</section>
