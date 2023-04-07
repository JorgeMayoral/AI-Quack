<script lang="ts">
  import { Role } from '../types';

  export let role: Role;
  export let text: string = '';
</script>

<div class="p-4">
  {#if role === Role.System}
    <div class="flex items-center mb-4">
      <div class="flex-none flex flex-col items-center space-y-1 mr-4">
        <img
          class="rounded-full w-10 h-10"
          src="/quacker_profile.png"
          alt="Quacker"
        />
        <span class="block text-xs text-cat-text hover:underline">{role}</span>
      </div>
      <div
        class="flex-1 bg-cat-surface1 text-cat-text p-2 rounded-lg mb-2 relative"
      >
        <div>{text}</div>

        <div
          class="absolute left-0 top-1/2 transform -translate-x-1/2 rotate-45 w-2 h-2 bg-cat-surface1"
        />
      </div>
    </div>
  {:else}
    <div class="flex items-center flex-row-reverse mb-4">
      <div class="flex-none flex flex-col items-center space-y-1 ml-4">
        <img
          class="rounded-full w-10 h-10"
          src="/user_profile.png"
          alt="User"
        />
        <span class="block text-xs text-cat-text hover:underline">You</span>
      </div>
      <div
        class="flex-1 bg-cat-surface2 text-cat-text p-2 rounded-lg mb-2 relative"
      >
        <div>
          {text}
        </div>

        <div
          class="absolute right-0 top-1/2 transform translate-x-1/2 rotate-45 w-2 h-2 bg-cat-surface2"
        />
      </div>
    </div>
  {/if}
</div>
