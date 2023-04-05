<form on:submit|preventDefault>
  <div
    class="w-full mb-4 border rounded-lg bg-cat-surface1 border-cat-surface2"
  >
    <div class="px-4 py-2 rounded-t-lg bg-cat-surface0">
      <label for="comment" class="sr-only">Your comment</label>
      <textarea
        id="comment"
        rows="4"
        class="w-full px-0 text-sm border-0 bg-cat-surface0 focus:ring-transparent text-cat-text placeholder-cat-subtext1"
        placeholder="Write your question..."
      />
    </div>
    <div
      class="flex items-center justify-between px-3 py-2 border-t border-cat-surface2"
    >
      <button
        type="submit"
        class="inline-flex items-center py-2.5 px-4 text-xs font-medium text-center text-cat-surface0 bg-cat-sky rounded-lg focus:ring-transparent hover:bg-cat-blue"
      >
        Send Question
      </button>
    </div>
  </div>
</form>
