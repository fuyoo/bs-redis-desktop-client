<script lang="ts" setup>
import { useTheme } from '@/hooks/life';
const t = inject('theme') as Ref<any>
const { getMode, setTheme } = useTheme()
const theme = ref(getMode())
const onChange = () => {
  setTheme(theme.value, t)
}
</script>
<template>
  <n-radio-group @change="onChange" v-model:value="theme" size="small" type="button">
    <n-radio-button value="light">
      <i class="i-material-symbols:dark-mode-rounded"></i>
    </n-radio-button>
    <n-radio-button value="dark">
      <i class="i-material-symbols:light-mode-rounded"></i>
    </n-radio-button>
    <n-radio-button value="auto">
      <i class="i-material-symbols:night-sight-auto-rounded"></i>
    </n-radio-button>
  </n-radio-group>
</template>
