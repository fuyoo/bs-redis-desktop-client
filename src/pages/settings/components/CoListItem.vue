<script lang="ts" setup>

</script>
<template>
  <n-el tag="div"
    class="flex justify-between  items-center px-5 h-12  bg-[var(--tab-color)]  mb-5 rounded relative select-none cursor-pointer">
    <n-text>
      <slot name="icon"></slot>
      <slot name="name"></slot>
    </n-text>
    <slot></slot>
  </n-el>
</template>
<style lang="scss" scoped></style>
