<script setup lang="ts">
import { useReqStore } from '@/stores/req.ts'
import { shallowRef } from 'vue'

const props = defineProps<{
  value: string,
  size: string
}>()
const reqStore = useReqStore()
const content = shallowRef<string>('')
const fetchData = async () => {
  const { data } = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['get', props.value],
  })
  content.value = data
}
fetchData()
</script>

<template>
  <div class="w-full h-full">
    <textarea class="w-full h-full resize-none b-none outline-none"  v-model="content"/>
  </div>
</template>

<style scoped lang="scss"></style>
