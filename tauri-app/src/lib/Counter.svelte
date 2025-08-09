<script lang="ts">
  let count: number = $state(0)
  const increment = () => {
    count += 1
    // console.log(count)
  }
</script>

<button onclick={increment}>
  count is {count}
</button>
