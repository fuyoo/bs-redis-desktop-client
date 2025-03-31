<script lang="ts" setup>
import { onBeforeUnmount, ref } from 'vue'
import CoKeys from './CoKeys.vue'
import CoDbs from './CoDbs.vue'
import CoInformation from './CoInformation.vue'
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
  <div class="flex h-full" :class="{ moving: canMove }">
    <div class="_ml l b-r b-r-dashed b-r-#eee relative" :style="{ width: `${width}px` }">
        <div class="w-full h-10 flex items-center justify-center">
          <CoDbs></CoDbs>
        </div>
        <CoKeys v-model="key"></CoKeys>
      <div
        @mousedown="mousedownFn"
        class="opacity-0 flex justify-center items-center  move w-5 h-5 bg-#ddd shadow-lg absolute top-50% mt--2 right--2 rounded"
      >
        <span class="i-uil:arrows-resize-h"></span>
      </div>
    </div>
    <div class="r flex-1">
      <q-scroll-area class="h-[calc(100vh-90px)]">
        <co-information :value="key"></co-information>
      </q-scroll-area>
    </div>
  </div>
</template>
<style scoped lang="scss">
._ml *{
  -webkit-user-select: none;
  user-select: none;
}
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
