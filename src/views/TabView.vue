<script setup lang="ts">
import {ref} from 'vue'
import {useRoute} from 'vue-router'
const route = useRoute()
const count = ref(0)
</script>

<template>
  <div>
    <h1>{{route.params}}</h1>
    <a-button type="primary" @click="count++">{{count}}</a-button>
  </div>
</template>

<style scoped>

</style>