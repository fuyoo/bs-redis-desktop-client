<template>
  <div
    class="fullscreen bg-blue text-white text-center q-pa-md flex flex-center"
    data-tauri-drag-region
  >
    <div>
      <div style="font-size: 30vh" data-tauri-drag-region class="cursor-default">404</div>

      <div class="text-h2 cursor-default" data-tauri-drag-region style="opacity: 0.4">
        Oops. Nothing here...
      </div>

      <q-btn
        class="q-mt-xl"
        color="white"
        text-color="blue"
        unelevated
        to="/"
        label="Go Home"
        no-caps
      />
    </div>
  </div>
</template>

<script setup lang="ts">
//
</script>
