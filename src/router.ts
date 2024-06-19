import { createMemoryHistory, createRouter } from 'vue-router'

import HelloWorld from './components/HelloWorld.vue'
import BookList from './components/BookList.vue'
// import AboutView from './AboutView.vue'

const routes = [
  { path: '/', component: HelloWorld },
  { path: '/books', component: BookList },
  { path: '/books/:id', component: BookList, props: true },
  // { path: '/about', component: AboutView },
]

export const router = createRouter({
  history: createMemoryHistory(),
  routes,
})