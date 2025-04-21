<script setup lang="ts">
import { useReqStore } from '@/stores/req.ts'
import { reactive, ref, watch } from 'vue'

const prop = defineProps<{ value?: string }>()
const vModel = defineModel()
const vSizeModel = defineModel('size')
const reqStore = useReqStore()
const baseInfo =  reactive({
  type: '',
  memory: '',
  pttl: '',
})
const fetchInfo = async () => {
  const t = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['type', prop.value],
  })
  baseInfo.type = t.data
  vModel.value = t.data
  const mem = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['memory','usage',prop.value]
  })
  baseInfo.memory = mem.data
  vSizeModel.value = mem.data
  const pttl = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['pttl', prop.value]
  })
  baseInfo.pttl = pttl.data
}
fetchInfo()
watch(() => prop.value, () => {
  fetchInfo()
})
</script>

<template>
  <div class="b-b b-b-dashed b-b-#eee p-4 gap-2 flex">
    <b>{{ value }}</b>
    <q-badge><i class="i-iconamoon:type-bold mr-1"></i>  {{baseInfo.type.toUpperCase()}}</q-badge>
    <q-badge> <i class="i-material-symbols:memory-alt mr-1"></i>  {{baseInfo.memory}}bytes</q-badge>
    <q-badge>
      <i class="i-material-symbols:nest-clock-farsight-analog-outline"></i>TTL
      {{baseInfo.pttl+'ms'}}
    </q-badge>
  </div>
</template>

<style scoped lang="scss">

</style>
