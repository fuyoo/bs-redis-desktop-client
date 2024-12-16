<template>
  <q-tabs class="mx-10" dense shrink stretch inline-label outside-arrows mobile-arrows>
    <q-route-tab @click="handleClose" v-for="item in hosts" :key="item.id" :to="`/host/${item.id}`" icon="storage"
      :label="item.name">
      <q-btn :data-id="item.id" dense flat size="xs" round class="ml-1">
        <span class="i-ic:round-close text-4" :data-id="item.id"></span>
      </q-btn>
    </q-route-tab>
  </q-tabs>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'
const router = useRouter()
const route = useRoute()
watch(
  () => route.params.id,
  async (id: string | string[] | undefined) => {
    const resp = await invoke('tab_change', { tab: { id: id || 'main', name: 'host' } })
    console.log(resp)
  },
)
// below logic aim to reset webview position when the window was resizing.
let timer: any
window.addEventListener('resize', () => {
  clearTimeout(timer)
  setTimeout(async () => {
    if (route.params.id) {
      const resp = await invoke('tab_view_resize', { id: route.params.id })
      console.log(resp)
    }
  }, 160)
})
// todo: this tabs list should be fetched from rust side.
const hosts = ref(new Array(10).fill(0).map((e, i) => ({ id: i + '', name: `HOST-${i}` })))

// handle tab close logic.
const handleClose = async (ev: Event) => {
  const id = (ev.target as HTMLButtonElement).dataset.id
  if (id) {
    // prevent q-route-tab component route to new path.
    ev.preventDefault()
    // close webview at rust side.
    await invoke('tab_close', { tab: { id } })
    // update tabs list.
    const index = hosts.value.findIndex((e) => e.id === id)
    hosts.value.splice(hosts.value.findIndex((e) => e.id === id), 1)
    const nid = hosts.value[index]?.id || hosts.value[index - 1]?.id
    // focus on next tab.
    if (route.params.id) {
      if (nid) {
        await router.push(`/host/${nid}`)
      } else {
        // if tab empty, go to home page.
        await router.push(`/`)
      }
    }
  }
}
</script>
