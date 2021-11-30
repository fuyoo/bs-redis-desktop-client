<template>
  <i :class="`remix icon-${name}`"></i>
</template>

<script>
import './assets/icon.css'

export default {
  name: 'Icon',
  props: {
    name: {
      required: true,
      type: String
    }
  }
}
</script>

<style scoped>

</style>
