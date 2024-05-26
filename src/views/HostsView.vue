<script setup lang="ts">
import useTabStore from '../store/tabs'
const tabStore = useTabStore()
const jumpTo = () => {
  const id = Math.ceil(Math.random() * 10e9)
  tabStore.addTab({
    id:id,
    path:`/tab/${id}`,
    name: "测试tab"+id
  })
}
</script>

<template>
  <div>
    <a-button type="primary" @click="jumpTo()">添加一个tab页</a-button>
  </div>
</template>

<style scoped>

</style>