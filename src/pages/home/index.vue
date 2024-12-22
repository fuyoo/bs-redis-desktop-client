<template>
  <q-page class="row items-center justify-evenly">
    <div>
      <q-btn @click="changeTab">new Host</q-btn>
    </div>
  </q-page>
</template>

<script setup lang="ts">
import { useTabStore } from '../../stores/tab'

const tab = useTabStore()
const changeTab = () => {
  tab.change({
    id: Math.random().toString(36).slice(2),
    name: `Host-${Math.random().toString(36).slice(2)}`,
  })
}
</script>
