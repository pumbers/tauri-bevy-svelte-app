<script lang="ts">
  import { onMount } from 'svelte'

  let count = 0
  onMount(() => {
    const interval = setInterval(() => count++, 1000)
    return () => clearInterval(interval)
  })
</script>

<p class="card w-min !mx-auto !p-8">Counter: <code>{count}</code></p>

<style>
  /* styles go here */
</style>
