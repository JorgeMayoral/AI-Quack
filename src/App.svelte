<script lang="ts">
  import ResponseList from './lib/ResponseList.svelte';
  import TextForm from './lib/TextForm.svelte';

  import { invoke } from '@tauri-apps/api/tauri';
  import { Role, type Message } from './lib/types';

  let responses: Message[] = [
    {
      role: Role.System,
      message:
        'Hi! My name is Quacker. Can I help you with a programming problem?',
    },
  ];
  let getTextResponse = async (event) => {
    let userPrompt = event.target[0].value;
    responses = [...responses, { role: Role.User, message: userPrompt }];
    let response: string = await invoke('get_text_response', {
      userPrompt,
    });
    responses = [...responses, { role: Role.System, message: response }];
  };
</script>

<main
  class="py-6 px-4 flex flex-col gap-4 justify-between h-screen bg-cat-base"
>
  <ResponseList {responses} />
  <TextForm on:submit={getTextResponse} />
</main>
