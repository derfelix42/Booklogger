<script setup lang="ts">
import { ref } from 'vue'

defineProps<{ msg: string }>()

const count = ref(0)
</script>

<template>
  <h1>Welcome to the Booklogger!</h1>

  <div class="image_container">
    <img alt="Vue logo" src="../assets/logo.png" class="logo" />
  </div>

  <section>
    <div>
      <button>
        lets get started... >
      </button>
    </div>

  </section>

</template>

<style scoped>
div {
  margin: 2em;
}
img {
  width: 10em;
}
button {
  padding: 1em;
  font-size: 1.2em;
}
</style>
