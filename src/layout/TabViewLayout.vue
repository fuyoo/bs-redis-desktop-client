<script setup lang="ts">
import useTabStore from '../store/tabs'

const tabStore = useTabStore()
import TabView from '@/views/TabView.vue'
</script>

<template>
  <div>
    <div v-for="(item) of tabStore.list" :key="item.id" :title="item.name" v-show="item.id == tabStore.activeTab?.id">
      <TabView/>
    </div>
  </div>
</template>

<style scoped>

</style>