<script lang="ts">    
    const invokeTauri = () => {
    // When using the Tauri global script (if not using the npm package)
    // Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
    const invoke = window.__TAURI__.core.invoke;

    // Invoke the command
    invoke('my_custom_command');
  }
</script>

<button onclick={invokeTauri}>
  Invoke Custom Tauri Command
</button>
