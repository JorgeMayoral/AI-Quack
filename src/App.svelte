<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri';
  import { onMount } from 'svelte';
  import ApiKeyInput from './lib/components/ApiKeyInput.svelte';
  import ResponseList from './lib/components/ResponseList.svelte';
  import TextForm from './lib/components/TextForm.svelte';
  import { Role, type Message } from './lib/types';

  onMount(async () => {
    needApiKey = await invoke('check_api_key');
  });

  let needApiKey: boolean;
  let loading: boolean = false;
  let responses: Message[] = [
    {
      role: Role.System,
      message:
        'Hi! My name is Quacker. Can I help you with a programming problem?',
    },
  ];
  const getTextResponse = async ({ detail }) => {
    loading = true;
    const userPrompt = detail.text;
    responses = [...responses, { role: Role.User, message: userPrompt }];
    const response: string = await invoke('get_text_response', {
      userPrompt,
    });
    responses = [...responses, { role: Role.System, message: response }];
    loading = false;
  };

  const sendApiKey = async (event) => {
    const apiKey = event.target[0].value;
    await invoke('set_api_key', { apiKey });
    needApiKey = false;
  };
</script>

<main
  class="py-6 px-4 flex flex-col gap-4 justify-between h-screen bg-cat-base"
>
  {#if needApiKey}
    <ApiKeyInput on:submit={sendApiKey} />
  {:else}
    <ResponseList {responses} />
    <TextForm on:submit={getTextResponse} {loading} />
  {/if}
</main>
