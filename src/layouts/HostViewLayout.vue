<script setup lang="ts">
import { ref } from 'vue'
import { useRoute } from 'vue-router'

const route = useRoute()
const splitterModel = ref(50)
</script>
<template>
  <q-splitter style="height: 400px" v-model="splitterModel">
    <template v-slot:before>
      <div class="q-pa-md">
        <div class="text-h4 q-mb-md">Before</div>
        {{ route.params.id }}
      </div>
    </template>

    <template v-slot:after>
      <div class="q-pa-md">
        <div class="text-h4 q-mb-md">After</div>
        <div v-for="n in 20" :key="n" class="q-my-md">
          {{ n }}. Lorem ipsum dolor sit, amet consectetur adipisicing elit. Quis praesentium cumque
          magnam odio iure quidem, quod illum numquam possimus obcaecati commodi minima assumenda
          consectetur culpa fuga nulla ullam. In, libero.
        </div>
      </div>
    </template>
  </q-splitter>
</template>
