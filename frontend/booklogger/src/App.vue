<template>
  <!-- <v-app-bar :elevation="2"></v-app-bar> -->
  <v-app>
    <v-app-bar :elevation="2">Hallo Welt!</v-app-bar>
    <v-container>
      <router-view />
    </v-container>
    <v-footer class="d-flex flex-column">
      <div class="px-4 py-2 bg-black text-center w-100">
        {{ new Date().getFullYear() }} — <strong>Booklogger</strong>
      </div>
    </v-footer>
  </v-app>


</template>

<script lang="ts" setup>
  //
</script>
