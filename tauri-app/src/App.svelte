<script lang="ts">
  // When using the Tauri API npm package:
  import { invoke } from '@tauri-apps/api/core';
  import Counter from './lib/Counter.svelte'
  import TestInvokeTauri from './lib/TestInvokeTauri.svelte'
  
</script>

<main>

  <div class="card">
    <TestInvokeTauri />
  </div>

</main>

<style>
  .card {
    padding: 2em;
    border: 1px solid #eee;
    border-radius: 8px;
    background: white;
    box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  }
</style>
