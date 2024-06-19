<script setup lang="ts">
import BookListEntry from './BookListEntry.vue'
import { reactive, onMounted } from 'vue'
import { invoke } from '@tauri-apps/api'

const props = defineProps({
    id: Number
})

const books = reactive([])

invoke('greet', { name: 'World' })
  // `invoke` returns a Promise
  .then((response) => console.log(response))

onMounted(async () => {
    const response = await invoke('get_books', { })
    console.log(response)
    Object.assign(books, response)
})


</script>

<template>
    <h1>Book List</h1>
    <p v-if="props.id">You selected: {{ props.id }}</p>
    <section>
        <BookListEntry v-for="book in books" :title="book" :likes="book"/>
    </section>
</template>

<style scoped>
section {
    display: flex;
    flex-wrap: wrap;
    gap: 1em;
}
</style>
