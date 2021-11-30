<template>
  <router-view></router-view>
</template>

<script>
export default {
  name: 'Layout'
}
</script>

<style lang="scss">

body {
  margin: 0;
  padding: 0;
  overflow: hidden;
}
body::before{
  content: "";
  position: fixed;
  left: 0;
  top: 0;
  width: 100%;
  height: 1px;
  background-color: $border;
  z-index: 9999999999999;
}
</style>
