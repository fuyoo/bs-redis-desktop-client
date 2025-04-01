<script setup lang="ts">
import { ElCard } from 'element-plus'
import { useReqStore } from '@/stores/req.ts'
import { storeToRefs } from 'pinia'

const prop = defineProps<{ value?: string }>()
const reqStore = useReqStore()
const fetchInfo = async () => {
  console.log(prop.value)
  const resp = await reqStore.reqWithHost({
    path: '/cmd',
    data: ['type',prop.value]
  })
  console.log(resp)
  const r2 = await reqStore.reqWithHost({
    path: '/cmd',
    data: ['memory','usage',prop.value]
  })
  console.log(r2)
}
fetchInfo()
</script>

<template>
  <div class="mx-2">
    <el-card shadow="never"> </el-card>
    <b>{{ value }}</b>
  </div>
</template>

<style scoped lang="scss"></style>
