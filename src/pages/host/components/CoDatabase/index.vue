<script lang="ts" setup>
import { ref } from 'vue';
import CoKeys from './CoKeys.vue';
import CoDbs from './CoDbs.vue';
import CoInformation from './CoInformation.vue'
const split = ref(200)
const key = ref('')
</script>
<template>
  <q-splitter v-model="split" unit="px" :limits="[200, 480]">
    <template v-slot:before>
      <div class="_ml">
        <div class="w-full h-10 flex items-center justify-center">
          <CoDbs></CoDbs>
        </div>
        <CoKeys v-model="key"></CoKeys>
      </div>
    </template>

    <template v-slot:after>
      <q-scroll-area class="h-[calc(100vh-90px)]">
        <co-information :value="key"></co-information>
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
