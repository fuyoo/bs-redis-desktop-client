<script setup lang="ts">
import {ref} from 'vue'
import {useRoute} from 'vue-router'
import { status } from '@/api/backend';
const route = useRoute()
const count = ref(0)

status<any>({})
.then( console.log)
</script>

<template>
  <div>
    <h1>{{route.params}}</h1>
    <a-button type="primary" @click="count++">{{count}}</a-button>
  </div>
</template>

<style scoped>

</style>