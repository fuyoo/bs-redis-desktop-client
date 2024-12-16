<template>
  <q-page class="flex flex-column items-start justify-center">
    <div class="mt-10 w-120">
      <div class="flex justify-center items-center flex-col pb-5">
        <img src="@/assets/icon.ico" class="w-20 h-20">
        <span>v{{ pkg.version }}</span>
      </div>
      <div class="flex justify-between items-center pl -5 pr-1 h-12 bg-gray-50 mb-5 rounded overflow-hidden">
        <span class="text-4"><i class="i-flowbite:language-outline text-5 mr-1"></i> {{ $t('settings[1]') }}</span>
        <CoLanguage />
      </div>
      <div
        class="flex justify-between items-center px-5 h-12 bg-gray-50 mb-5 rounded relative select-none cursor-pointer">
        <span class="text-4"><i class="i-material-symbols:tips-and-updates-outline-rounded text-5 mr-1"></i>
          {{ $t('settings[2]') }}</span>
        <span class="relative">v{{ pkg.version }}</span>
        <q-badge color="red" floating>new</q-badge>
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import CoLanguage from './components/CoLanguage.vue'
const pkg = window.PKG
</script>
