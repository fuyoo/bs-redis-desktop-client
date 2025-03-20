<script lang="ts" setup>
import { ref } from 'vue';
import { useRoute } from 'vue-router';
import CoKeys from './CoKeys.vue';
import CoDbs from './CoDbs.vue';
const route = useRoute();
const spliter = ref(200)
console.log(route.fullPath)
</script>
<template>
  <q-splitter v-model="spliter" unit="px" :limits="[200, 480]">
    <template v-slot:before>
      <div class="_ml">
        <div class="w-full h-10 flex items-center justify-center">
          <CoDbs></CoDbs>
        </div>
        <CoKeys></CoKeys>
      </div>
    </template>

    <template v-slot:after>
      <q-scroll-area class="h-[calc(100vh-90px)]">
      </q-scroll-area>
    </template>
  </q-splitter>
</template>
<style scoped lang="scss">
._ml {
  user-select: none;
  -webkit-user-select: none;

  * {
    user-select: none;
    -webkit-user-select: none;

  }
}
</style>
