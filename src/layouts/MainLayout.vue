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
      <q-scroll-area class="w-full h-[calc(100vh-50px)]">
        <router-view />
      </q-scroll-area>
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { useI18n } from 'vue-i18n'
import WindowOperationButtonGroup from 'src/components/WindowOperationButtonGroup.vue'
import EssentialLink, { type EssentialLinkProps } from 'components/EssentialLink.vue'
const { t, locale } = useI18n()
const linksList = ref<EssentialLinkProps[]>([
  {
    title: t('menu[0][0]'),
    caption: t('menu[0][1]'),
    icon: 'storage',
    link: '/',
  },
  {
    title: t('menu[1][0]'),
    caption: t('menu[1][1]'),
    icon: 'settings',
    link: '/settings',
    badge: true,
  },
  {
    title: t('menu[2][0]'),
    caption: 'github.com/fuyoo/bs',
    icon: 'code',
    link: 'https://github.com/fuyoo/bs-redis-desktop-client',
    inner: 2,
  },
])
watch(locale, () => {
  linksList.value.forEach((v: any, i: number) => {
    v.title = t(`menu[${i}][0]`)
    v.caption = t(`menu[${i}][1]`)
  })
})
const leftDrawerOpen = ref(false)
function toggleLeftDrawer() {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
</script>
