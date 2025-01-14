<script setup lang="ts">
import { useRoute } from 'vue-router'

const route = useRoute()
import CoHostInfo from './components/CoHostInfo.vue'
</script>
<template>
  <co-host-info></co-host-info>
</template>
