<script lang="ts">
  import { afterUpdate } from 'svelte';
  import ResponseText from './ResponseText.svelte';
  import { Role, type Message } from './types';

  export let responses: Message[] = [];

  const scrollToBottom = async (node) => {
    node.scroll({ top: node.scrollHeight, behavior: 'smooth' });
  };

  let list = [];
  let element;

  afterUpdate(() => {
    console.log('afterUpdate');
    if (list) scrollToBottom(element);
  });

  $: if (list && element) {
    console.log('tick');
    scrollToBottom(element);
  }
</script>

<section bind:this={element} class="overflow-y-auto">
  {#each responses as response}
    <ResponseText text={response.message} role={response.role} />
  {:else}
    <ResponseText text="No text to display." role={Role.System} />
  {/each}
</section>
