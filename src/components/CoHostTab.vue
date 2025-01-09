<template>
  <q-tabs class="mx-10" active-class="_tab-active" indicator-color="transparent" dense shrink stretch inline-label
    narrow-indicator outside-arrows mobile-arrows>
    <q-route-tab no-caps :ripple="false" class="p-0 h-35px rounded-20px mt-1.75" @click="handleClose"
      v-for="item in tabStore.tabList" :key="item.id" :to="`/host/${item.id}`">
      <div class="flex justify-center items-center flex-nowrap _focus">
        <i class="i-mdi:server-network text-4 mr-1"></i>
        <div class="max-w-25 text-ellipsis overflow-hidden text-nowrap mr-1" style="line-height: 1;">{{ item.name }}
        </div>
        <q-btn :data-id="item.id" dense flat size="xs" round class="ml-1">
          <span class="i-ic:round-close text-4" :data-id="item.id"></span>
        </q-btn>
      </div>
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

<style lang="scss" scoped>
._focus {
  height: 35px;
  border-radius: 20px;
  padding: 0 15px;

  &:hover>.opacity-0 {
    opacity: 1;
  }
}

._tab-active {
  ._focus {
    background: #0002;
  }
}
</style>
