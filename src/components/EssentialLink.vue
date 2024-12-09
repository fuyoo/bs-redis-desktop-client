<template>
  <q-item clickable @click="jumpTo">
    <q-item-section v-if="icon" avatar class="relative">
      <q-icon :name="icon" />
      <q-badge floating color="red" v-if="badge" class="mr-5">new</q-badge>
    </q-item-section>

    <q-item-section>
      <q-item-label class="select-none">{{ title }}</q-item-label>
      <q-item-label caption class="select-none">{{ caption }}</q-item-label>
    </q-item-section>
  </q-item>
</template>

<script setup lang="ts">
import { open } from '@tauri-apps/plugin-shell'
import { useRouter } from 'vue-router'
export interface EssentialLinkProps {
  title: string
  caption?: string
  link?: string
  icon?: string
  inner?: 1 | 2
  badge?: boolean
}

const props = withDefaults(defineProps<EssentialLinkProps>(), {
  caption: '',
  link: '/',
  icon: '',
  inner: 1,
})
const router = useRouter()
const jumpTo = () => {
  if (props.inner == 2) {
    open(props.link)
    return
  }
  router.push(props.link)
}
</script>
