<script setup lang="ts">
const props = withDefaults(defineProps<{
  mini?: boolean
  label: string
  icon: string
  id: number | string
}>(), {
  mini: false
})
const emit = defineEmits<{ click: [number | string], close: [number | string] }>()

const click = (key: number | string) => {
  emit('click', key)
}

const close = (key: number | string) => {
  emit('close', key)
}
</script>

<template>
  <div class="rounded-[4px] hover:bg-amber text-white flex p-6 hover:text-black" @click="click(id)">
    <div :class="icon"></div>
    <div v-show="!mini">
      {{ label }}
    </div>
    <div class="i-ic-twotone-close"></div>
  </div>
</template>

<style scoped>

</style>