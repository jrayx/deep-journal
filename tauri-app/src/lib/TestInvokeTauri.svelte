<script lang="ts">    
    // When using the Tauri global script (if not using the npm package)
    // Be sure to set `app.withGlobalTauri` in `tauri.conf.json` to true
    const invoke = window.__TAURI__.core.invoke;
    
    const invokeTauriHelloWorld = () => {
        invoke('hello_world', { invoke_message: 'Hello!' }).then((response: String) => {
            console.log(response);
        });
    }
    
    const invokeTauriReadFile = () => {
        invoke('read_file').then((response: String) => {
            const text = new TextDecoder("utf-8").decode(response);
            console.log(text);
        });
    }
    
    // // example error handling
    // const invokeLogin = () => {
    //     invoke('login', { user: 'tauri', password: '0j4rijw8=' })
    //     .then((message) => console.log(message))
    //     .catch((error) => console.error(error));
    // }
</script>

<button onclick={invokeTauriHelloWorld}>
  Invoke Hello World Tauri Command
</button>

<button onclick={invokeTauriReadFile}>
  Invoke Read File Tauri Command
</button>