<template>
  <q-layout view="lHh Lpr lFf">
    <q-header elevated>
      <q-toolbar data-tauri-drag-region>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
          class="mr-5"
        />
        <q-btn flat dense round icon="home" to="/"></q-btn>
        <q-space></q-space>
        <WindowOperationButtonGroup></WindowOperationButtonGroup>
      </q-toolbar>
    </q-header>

    <q-drawer v-model="leftDrawerOpen" show-if-above bordered>
      <q-list>
        <q-item-label header> BS <small>v2.0.0-dev</small> </q-item-label>
        <EssentialLink v-for="link in linksList" :key="link.title" v-bind="link" />
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { ref } from 'vue'
import WindowOperationButtonGroup from 'src/components/WindowOperationButtonGroup.vue'
import EssentialLink, { type EssentialLinkProps } from 'components/EssentialLink.vue'
const linksList: EssentialLinkProps[] = [
  {
    title: 'Host',
    caption: 'redis database host',
    icon: 'storage',
    link: '/',
  },
  {
    title: 'Settings',
    caption: 'app setting',
    icon: 'settings',
    link: '/settings',
    badge: true,
  },
  {
    title: 'Github',
    caption: 'github.com/fuyoo/bs',
    icon: 'code',
    link: 'https://github.com/fuyoo/bs-redis-desktop-client',
    inner: 2,
  },
]
const leftDrawerOpen = ref(false)
function toggleLeftDrawer() {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
</script>
