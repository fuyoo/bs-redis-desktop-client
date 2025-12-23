<template>
  <div class="flex flex-column items-start justify-center">
    <div class="mt-10 w-120">
      <div class="flex justify-center items-center flex-col pb-5">
        <img src="@/assets/icon.ico" class="w-20 h-20">
        <n-text>v{{ version }}</n-text>
      </div>
      <co-list-item>
        <template #icon>
          <i class="i-flowbite:language-outline text-5 mr-1"></i>
        </template>
        <template #name>
          {{ $t('settings[1]') }}
        </template>
        <CoLanguage />
      </co-list-item>
      <co-list-item>
        <template #icon>
          <i class="i-fluent:dark-theme-20-filled text-5 mr-1" />
        </template>
        <template #name>
          {{ $t('settings[3]') }}
        </template>
        <CoTheme />
      </co-list-item>
      <co-list-item>
        <template #icon>
          <i class="i-material-symbols:tips-and-updates-outline-rounded text-5 mr-1"></i>
        </template>
        <template #name>{{ $t('settings[2]') }}</template>
        <n-badge :dot="update">
          <n-button text @click="askUpdate()" class="relative">v{{ version }}</n-button>
        </n-badge>
      </co-list-item>
    </div>
  </div>
</template>

<script setup lang="ts">
import CoLanguage from './components/CoLanguage.vue'
import CoTheme from './components/CoTheme.vue'
import CoListItem from './components/CoListItem.vue'
import { getVersion } from '@tauri-apps/api/app';
import { useUpdate } from '@/hooks/update';
const { checkUpdate, askUpdate } = useUpdate()
const version = ref('')
const update = ref<any>()
getVersion().then(async res => {
  version.value = res
  update.value = await checkUpdate()
})
</script>
