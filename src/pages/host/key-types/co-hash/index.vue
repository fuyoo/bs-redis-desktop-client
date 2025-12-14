<script setup lang="ts">
import { ref } from 'vue'
import CoInfoHeader from '@/pages/host/components/CoInfoHeader/index.vue'
const size = ref('')
</script>

<template>
  <div class="w-full h-full">
    <co-info-header v-model:size="size" type="hash">
      <n-space>
        <n-button type="primary" size="tiny">
          <template #icon><i class="i-material-symbols:edit-square-rounded"></i></template>
          {{ $t('actions[3]') }}
        </n-button>
      </n-space>
    </co-info-header>
  </div>
</template>

<style scoped lang="scss"></style>
