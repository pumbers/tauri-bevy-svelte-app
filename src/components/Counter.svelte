<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'

  let count = 0
  onMount(() => {
    const interval = setInterval(async () => (count = await invoke('get_state')), 1000)
    return () => clearInterval(interval)
  })
</script>

<p class="card w-min !mx-auto !p-8">Counter: <code>{count}</code></p>

<style>
  /* styles go here */
</style>
