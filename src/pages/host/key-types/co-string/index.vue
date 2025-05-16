<script setup lang="ts">
import { useReqStore } from '@/stores/req.ts'
import { ref, shallowRef } from 'vue'
import { useRoute } from 'vue-router'
import CoInfoHeader from "@/pages/host/components/CoInfoHeader/index.vue"
const route = useRoute()
const reqStore = useReqStore()
const content = shallowRef<string>('')
const fetchData = async () => {
  const { data } = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['get', atob(route.params.key as string)],
  })
  content.value = data
}
const size = ref<string>('')
fetchData()
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="string"/>
    <textarea class="w-full h-full resize-none b-none outline-none"  v-model="content"/>
  </div>
</template>

<style scoped lang="scss"></style>
