<template>
  <q-tabs class="mx-10" dense shrink stretch inline-label outside-arrows mobile-arrows>
    <q-route-tab
      @click="handleClose"
      v-for="item in tabStore.tabList"
      :key="item.id"
      :to="`/host/${item.id}`"
      icon="storage"
      :label="item.name"
    >
      <q-btn :data-id="item.id" dense flat size="xs" round class="ml-1">
        <span class="i-ic:round-close text-4" :data-id="item.id"></span>
      </q-btn>
    </q-route-tab>
  </q-tabs>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core'
import { watch } from 'vue'
import { useRoute } from 'vue-router'
import { useTabStore } from '../stores/tab'
const route = useRoute()
const tabStore = useTabStore()
watch(
  () => route.params.id,
  async (id: string | string[] | undefined) => {
    const tab = tabStore.tab(id as string)
    if (tab) {
      await invoke('tab_change', { tab })
    } else {
      await invoke('tab_change', { tab: { id: id || 'main' } })
    }
  },
)
// below logic aim to reset webview position when the window was resizing.
let timer: any
window.addEventListener('resize', () => {
  // throttle to improve performance
  clearTimeout(timer)
  setTimeout(async () => {
    if (route.params.id) {
      await invoke('tab_view_resize', { id: route.params.id })
    }
  }, 160)
})

// handle tab close logic.
const handleClose = async (ev: Event) => {
  const id = (ev.target as HTMLButtonElement).dataset.id
  if (id) {
    // prevent q-route-tab component route to new path.
    ev.preventDefault()
    tabStore.close(id)
  }
}
// init tab
tabStore.update()
</script>
