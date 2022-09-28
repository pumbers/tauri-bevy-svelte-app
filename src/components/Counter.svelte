<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api'
  import { listen, UnlistenFn } from '@tauri-apps/api/event'

  let count = 0
  onMount(() => {
    // const interval = setInterval(async () => (count = await invoke('get_state')), 1000)
    // return () => clearInterval(interval)

    let unlisten: UnlistenFn | undefined = undefined
    listen('send_state', (event) => {
      count = event.payload as number
    }).then((r) => (unlisten = r))

    return () => {
      if (unlisten) unlisten()
    }
  })
</script>

<p class="card w-min !mx-auto !p-8">Counter: <code>{count}</code></p>
<button class="border rounded p-2 bg-blue-500 text-white" on:click={() => invoke('reset_counter')}
  >Reset</button
>

<style>
  /* styles go here */
</style>
