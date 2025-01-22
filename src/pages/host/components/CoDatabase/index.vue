<script lang="ts" setup>
import { ref } from 'vue';

const spliter = ref(200)
import CoKeys from './CoKeys.vue';
</script>
<template>
  <q-splitter v-model="spliter" unit="px" :limits="[200, 480]">
    <template v-slot:before>
      <CoKeys></CoKeys>
    </template>

    <template v-slot:after>
      <q-scroll-area class="h-[calc(100vh-50px)]">
      </q-scroll-area>
    </template>
  </q-splitter>
</template>
