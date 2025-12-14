import { shallowRef, onMounted, onBeforeUnmount } from 'vue'

export const useResize = (sub?: number) => {
  const height = shallowRef(100)
  const width = shallowRef(0)
  let resizeTimer = 0 as any
  const onResize = () => {
    clearTimeout(resizeTimer)
    resizeTimer = setTimeout(() => {
      const dom = document.querySelector('#app')
      height.value = (dom?.getBoundingClientRect()?.height || 0) - (sub || 0)
      width.value = dom?.getBoundingClientRect()?.width || 0
    }, 50)
  }
  // dynamic set tree height
  onMounted(() => {
    window.addEventListener('resize', onResize)
    onResize()
  })

  onBeforeUnmount(() => {
    window.removeEventListener('resize', onResize)
    clearTimeout(resizeTimer)
  })

  return {
    height,
    width,
  }
}
