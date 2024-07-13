<script setup lang="ts">

import Status from "@/views/TabView/Status.vue"
import Data from "@/views/TabView/Data.vue"
import {shallowRef} from "vue";

const cmp = shallowRef(Status as any)
</script>

<template>
  <div class="w-full">
    <Component :is="cmp" @ok="() => cmp = Data"/>
  </div>
</template>

<style scoped>

</style>