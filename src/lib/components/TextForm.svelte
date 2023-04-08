<script lang="ts">
  import { createEventDispatcher } from 'svelte';

  export let loading: boolean;

  const distpatch = createEventDispatcher();

  let inputField;

  const handleSubmit = (e: Event) => {
    const text = e.target[0].value;
    distpatch('submit', { text });
    inputField.value = '';
  };
</script>

<form on:submit|preventDefault={handleSubmit}>
  <div
    class="w-full mb-4 border rounded-lg bg-cat-surface1 border-cat-surface2"
  >
    <div class="px-4 py-2 rounded-t-lg bg-cat-surface0">
      <label for="comment" class="sr-only">Your comment</label>
      <textarea
        bind:this={inputField}
        id="comment"
        rows="4"
        class="w-full px-0 text-sm border-0 bg-cat-surface0 focus:ring-transparent text-cat-text placeholder-cat-subtext1 disabled:opacity-50"
        placeholder="Write your question..."
        disabled={loading}
      />
    </div>
    <div
      class="flex items-center justify-between px-3 py-2 border-t border-cat-surface2"
    >
      <button
        type="submit"
        class="inline-flex items-center py-2.5 px-4 text-xs font-medium text-center text-cat-surface0 bg-cat-sky rounded-lg focus:ring-transparent hover:bg-cat-blue disabled:opacity-50 disabled:cursor-not-allowed"
        disabled={loading}
      >
        {#if loading}
          <svg
            class="animate-spin -ml-1 mr-3 h-5 w-5 text-cat-surface0"
            xmlns="http://www.w3.org/2000/svg"
            fill="none"
            viewBox="0 0 24 24"
          >
            <circle
              class="opacity-25"
              cx="12"
              cy="12"
              r="10"
              stroke="currentColor"
              stroke-width="4"
            />
            <path
              class="opacity-75"
              fill="currentColor"
              d="M4 12a8 8 0 018-8v8H4z"
            />
          </svg>
        {:else}
          Send Question
        {/if}
      </button>
    </div>
  </div>
</form>
