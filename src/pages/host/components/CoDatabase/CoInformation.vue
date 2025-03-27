<script setup lang="ts">
import { useI18n } from 'vue-i18n'
import { ElEmpty } from 'element-plus'
const { t } = useI18n()
const prop = defineProps<{ value?: string }>()


</script>

<template>
  <div>
    <el-empty v-show="!value" :description="t('normal.1')"></el-empty>
    <b v-show="value">{{value}}</b>
  </div>
</template>

<style scoped lang="scss"></style>
