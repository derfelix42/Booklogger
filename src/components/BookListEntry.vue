<script setup lang="ts">
import { computed } from 'vue'

const props = defineProps({
  title: String,
  likes: Number
})

const url = computed(() => `/books/${props.likes}`)

</script>

<template>
    <RouterLink :to="url">
        <article>
            <h2>{{ props.title }}</h2>
            <p>{{ props.likes }} likes</p>
        </article>
    </RouterLink>
</template>

<style scoped>
article {
    border: 1px solid #ccc;
    padding: 1em;
    margin: 1em;
}
</style>