<script setup lang="ts">
import { useRoute } from 'vue-router';

const route = useRoute()
</script>
<template>
  <div>
    <span>{{ route.params.id }}</span>
  </div>
</template>
