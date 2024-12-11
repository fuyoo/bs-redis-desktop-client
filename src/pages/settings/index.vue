<template>
  <q-page class="flex flex-column items-start justify-center">
    <div class="mt-10 w-120">
      <div class="mb-5 font-700 text-4.5 flex items-center">
        <i class="i-ic:settings mr-2 text-5"></i>
        {{$t('settings[0]')}}
      </div>
      <div
        class="flex justify-between items-center pl -5 pr-1 h-12 bg-gray-50 mb-5 rounded overflow-hidden"
      >
        <span class="text-4"
          ><i class="i-flowbite:language-outline text-5 mr-1"></i> {{$t('settings[1]')}}</span
        >
        <CoLanguage />
      </div>
      <div
        class="flex justify-between items-center px-5 h-12 bg-gray-50 mb-5 rounded relative select-none cursor-pointer"
      >
        <span class="text-4"
          ><i class="i-material-symbols:tips-and-updates-outline-rounded text-5 mr-1"></i>
          {{$t('settings[2]')}}</span
        >
        <span class="relative">v2.0.0-dev </span>
        <q-badge color="red" floating>new</q-badge>
      </div>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import CoLanguage from './components/CoLanguage.vue'
</script>
