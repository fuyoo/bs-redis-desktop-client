<script setup lang="ts">
import { Key } from '@/tools/keys.ts'

const key = Key()
</script>

<template>
  <div class="h-full w-full flex justify-center items-center">
    <n-empty size="huge" :description="$t('tips.2',{key:key})"></n-empty>
  </div>
</template>

<style scoped lang="scss">

</style>
