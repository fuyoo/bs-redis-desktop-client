<script lang="ts" setup>
import { useReqStore } from '@/stores/req';
import JSONFormatter from 'json-formatter-js';
import { onMounted, shallowRef } from 'vue';
const reqStore = useReqStore()
const ctxRef = shallowRef<HTMLElement | undefined>()
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
  ctxRef.value!.innerHTML = ''
  ctxRef.value?.appendChild(formatter.render())
}
onMounted(async () => {
  await fetchInfo()
})
</script>
<template>
  <div class="p-4 z-1 realative text-4.5" ref="ctxRef"></div>
</template>
