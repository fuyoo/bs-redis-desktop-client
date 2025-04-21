<script lang="ts" setup>
import { computed, onBeforeUnmount, ref } from 'vue'
import CoKeys from './components/CoKeys/index.vue'
import CoDbs from './components/CoDatabase/index.vue'
import CoInfoLayout from './components/CoInfoLayout/index.vue'
import { ElEmpty } from 'element-plus'
import { useI18n } from 'vue-i18n'
const { t } = useI18n()
const key = ref('type:string')
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
const keyInfo = computed(() => {
  if (key.value) {
    return CoInfoLayout
  }
  return ElEmpty
})
</script>
<template>
  <div class="flex" :class="{ moving: canMove }">
    <div class="l flex flex-col items-start h-full b-r b-r-dashed b-r-#eee relative" :style="{ width: `${width}px` }">
        <div class="w-full h-10 flex items-center justify-center">
          <CoDbs></CoDbs>
        </div>
        <CoKeys v-model="key"></CoKeys>
      <div
        @mousedown="mousedownFn"
        class="opacity-0 flex justify-center items-center
        move w-5 h-5 bg-#ddd shadow-lg absolute top-50% mt--2 right--2 rounded"
      >
        <span class="i-uil:arrows-resize-h"></span>
      </div>
    </div>
    <div class="r flex-1 h-full">
      <q-scroll-area class="h-full">
        <component :is="keyInfo" :description="t('normal.1')" :value="key"></component>
      </q-scroll-area>
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
