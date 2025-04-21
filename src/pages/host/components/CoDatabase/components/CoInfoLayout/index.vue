<script setup lang="ts">
import { computed, defineAsyncComponent, ref } from 'vue'

defineProps<{ value?: string }>()
import CoInfoHeader from "@/pages/host/components/CoDatabase/components/CoInfoHeader/index.vue"
import type { RedisKeyType } from '@/types.ts'
import { ElEmpty } from 'element-plus'
const keyType = ref<RedisKeyType>("")
const typeView = computed(() => {
  switch (keyType.value) {
    case "string":
      return defineAsyncComponent(() => import("@/pages/host/components/CoDatabase/components/CoString/index.vue"))
    case "hash":
      return defineAsyncComponent(() => import("@/pages/host/components/CoDatabase/components/CoHash/index.vue"))
    case "list":
      return defineAsyncComponent(() => import("@/pages/host/components/CoDatabase/components/CoList/index.vue"))
    case "set":
      return defineAsyncComponent(() => import("@/pages/host/components/CoDatabase/components/CoSet/index.vue"))
    case "zset": return ElEmpty
    default:
      return ElEmpty
  }
})
const size = ref<string>()
</script>

<template>
  <div class="w-full h-full">
    <co-info-header :value="value" v-model="keyType" v-model:size="size"/>
    <component :is="typeView" :value="value" :size="size"/>
  </div>
</template>

<style scoped lang="scss"></style>
