<template>
  <q-tabs
    class="mx-10"
    dense
    v-model="tab"
    shrink
    stretch
    inline-label
    outside-arrows
    mobile-arrows
  >
    <q-route-tab
      v-for="index in 10"
      :key="index"
      :to="`/host/${index}`"
      icon="storage"
      :label="`HOST-${index}`"
    />
  </q-tabs>
</template>

<script lang="ts" setup>
import { invoke } from '@tauri-apps/api/core'
import { ref, watch } from 'vue'
import { useRoute } from 'vue-router'

const tab = ref('t1')
const route = useRoute()
watch(
  () => route.params.id,
  async (id: string | string[] | undefined) => {
    console.log(id)
    const resp = await invoke('tab_change', { tab: { id: id || 'main', name: 'host' } })
    console.log(resp)
  },
)
// blow logic aim to reset webview position when the window was resized.
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
</script>
