<script setup lang="ts">
import {info, InfoSection} from "@/api/backend.ts";
import {ref} from "vue";
const status = ref<string[][]>([]);
info<String>(InfoSection.All)
    .then(res => {
      status.value = res.data.split("# ").filter(Boolean).map(e => e.split("\r\n").filter(Boolean));
    })
</script>

<template>
  <div class="w-full">
    <a-tabs direction="vertical" class="w-full">
      <a-tab-pane :title="section[0]" v-for="(section,k) in status" :key="k" >
          <a-list class="mb-4 mr-4">
            <a-list-item v-for="item in section.slice(1)" :key="item" >
              {{item}}
            </a-list-item></a-list>
      </a-tab-pane>
    </a-tabs>
  </div>

</template>

<style scoped lang="scss">
</style>