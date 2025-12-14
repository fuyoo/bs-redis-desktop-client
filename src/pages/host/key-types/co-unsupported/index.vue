<script setup lang="tsx">
import { useReqStore } from '@/stores/req.ts'
import { ref, shallowRef } from 'vue'
import { useRoute } from 'vue-router'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'

const route = useRoute()
const reqStore = useReqStore()
const content = shallowRef<string>('')
const size = ref<string>('')
const fetchData = async () => {
  const { data } = await reqStore.reqWithHost<string>({
    path: '/cmd',
    data: ['get', atob(route.params.key as string)]
  })
  // more than 1Mb, we are truncate it.
  if (Number(size.value) > 1024 * 1024) {
    content.value = data.slice(0, 1024 * 1024) + '...'
    return
  }
  content.value = data
}
fetchData()
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="string" />
    <div class="w-full h-full flex justify-center items-center">
      <n-empty size="huge" :description="$t('tips.0')"></n-empty>
    </div>
  </div>
</template>

<style scoped lang="scss"></style>
