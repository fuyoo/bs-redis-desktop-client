<script lang="ts" setup>
defineProps<{
  db: number | string
}>()
</script>
<template>
  <h1 style="height: 900px;">{{ db }}</h1>
</template>
