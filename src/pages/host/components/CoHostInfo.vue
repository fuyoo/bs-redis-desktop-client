<script lang="ts" setup>
import { onMounted, shallowRef } from 'vue'
import { db, type ConnectionHost } from '@/db'
import { useRoute } from 'vue-router'
import { request } from '@/api'
import JSONFormatter from 'json-formatter-js'
import { useReqStore } from '@/stores/req'
const route = useRoute()
const ctxRef = shallowRef<HTMLElement | undefined>()
const reqStore = useReqStore()
const fetchInfo = async () => {
  const data = await reqStore.reqWithHost<string>({
    path: '/info',
    data: '',
  })
  const cfg = {} as Record<string, any>
  data.data
    .split('#')
    .filter((e) => !!e)
    .map((e) => e.trimStart())
    .map((e) => e.split('\r\n').filter((e) => !!e))
    .forEach((per) => {
      const key = per[0]
      cfg[key] = []
      for (const i of per) {
        if (i != key) {
          cfg[key].push(i)
        }
      }
    })
  const formatter = new JSONFormatter(cfg)
  ctxRef.value?.appendChild(formatter.render())
}
onMounted(() => {
  fetchInfo()
})
</script>
<template>
  <div class="p-4 z-1 realative text-4.5" ref="ctxRef"></div>
</template>

<style lang="scss" scope>
.renderjson a {
  text-decoration: none;
}
</style>
