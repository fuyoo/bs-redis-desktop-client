<script lang="ts" setup>
import { onBeforeUnmount, ref } from 'vue'
import CoKeys from '@/pages/host/components/CoKeys/new.vue'
import { useRoute } from 'vue-router'
const route = useRoute()
const key = ref('')
const canMove = ref(false)
let startX = 0
const width = ref(200)
let startWidth = 0
const mousedownFn = (e: MouseEvent) => {
  document.documentElement.onselectstart = (e) => e.preventDefault()
  canMove.value = true
  startX = e.clientX
  startWidth = width.value
}
const mousemoveFn = (e: MouseEvent) => {
  if (canMove.value) {
    // constraint max-min width
    const w = startWidth + (e.clientX - startX)
    if (w > 200 && w < 600) {
      width.value = w
    }
  }
}
const mouseupFn = () => {
  canMove.value = false
  document.documentElement.onselectstart = () => {}
}
const mouseleaveFn = () => {
  canMove.value = false
  document.documentElement.onselectstart = () => {}
}
window.addEventListener('mousemove', mousemoveFn)
window.addEventListener('mouseup', mouseupFn)
window.addEventListener('mouseleave', mouseleaveFn)
onBeforeUnmount(() => {
  window.removeEventListener('mousemove', mousemoveFn)
  window.removeEventListener('mouseup', mouseupFn)
  window.removeEventListener('mouseleave', mouseleaveFn)
})

</script>
<template>
  <div class="flex h-full bg-white" :class="{ moving: canMove }">
    <div
      class="l flex flex-col items-start h-full b-r b-r-dashed b-r-#eee relative"
      :style="{ width: `${width}px` }"
    >
      <CoKeys v-model="key"></CoKeys>
      <div
        @mousedown="mousedownFn"
        class="opacity-0 flex justify-center items-center move w-5 h-5 bg-#ddd shadow-lg absolute top-50% mt--2 right--2 rounded"
      >
        <span class="i-uil:arrows-resize-h"></span>
      </div>
    </div>
    <div class="r flex-1 h-full">
      <router-view :key="route.path"></router-view>
    </div>
  </div>
</template>
<style scoped lang="scss">
.moving {
  cursor: col-resize;

  &:hover .move {
    opacity: 1;
  }
}

.l:hover .move {
  opacity: 1;
}
</style>
