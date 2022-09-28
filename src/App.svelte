<script lang="ts">
  import './main.scss'
  import { Router, Route, Link } from 'svelte-routing'
  import Home from './routes/Home.svelte'
  import Files from './routes/Files.svelte'
  import Settings from './routes/Settings.svelte'

  export let url = ''
</script>

<Router {url}>
  <div id="navbar" class="grid grid-cols-6 gap-x-2">
    <div>Navbar</div>
    <div><Link to="home">Home</Link></div>
    <div><Link to="files">Files</Link></div>
    <div />
    <div />
    <div><Link to="settings">Settings</Link></div>
  </div>
  <div class="rounded-md shadow-md my-4 p-4">
    <Route path=""><Home /></Route>
    <Route path="files" component={Files} />
    <Route path="settings" component={Settings} />
  </div>
</Router>

<style>
  #navbar div {
    @apply border;
  }
</style>
