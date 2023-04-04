<form on:submit|preventDefault>
  <div class="w-full mb-4 border rounded-lg bg-Surface1 border-Surface2">
    <div class="px-4 py-2 rounded-t-lg bg-Surface0">
      <label for="comment" class="sr-only">Your comment</label>
      <textarea
        id="comment"
        rows="4"
        class="w-full px-0 text-sm border-0 bg-Surface0 focus:ring-transparent text-Text placeholder-Subtext1"
        placeholder="Write your question..."
      />
    </div>
    <div
      class="flex items-center justify-between px-3 py-2 border-t border-Surface2"
    >
      <button
        type="submit"
        class="inline-flex items-center py-2.5 px-4 text-xs font-medium text-center text-Surface0 bg-Sky rounded-lg focus:ring-transparent hover:bg-Blue"
      >
        Send Question
      </button>
    </div>
  </div>
</form>
