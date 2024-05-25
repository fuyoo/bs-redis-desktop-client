<script setup lang="ts">

import database, {type ConnectionImpl,type SettingsImpl} from '../database.ts'
import i18next from 'i18next'
import {ref} from 'vue'

const list = ref<ConnectionImpl[]>([])
const setting = ref<SettingsImpl[]>([])
const queryList = async () => {
  list.value = await database.connection.toArray()
  setting.value = await database.settings.toArray()
}
queryList()

const ad = async () => {
  await database.connection.add({
    name: 'string',
    uri: 'string | string[];',
    cluster: false
  })
  await queryList()
}
const lng = ref(i18next.language)
const change = () => {
  console.log(lng.value)
  i18next.changeLanguage(lng.value)
}
console.log(lng.value)
</script>

<template>
  <div>
    <button @click="ad()">{{ $t("name" as any) }}</button>
    <div class="flex">
      <ul>
        <li v-for="item in list" :key="item.id">{{ item.name }}</li>
      </ul>
      <ul>
        <li v-for="item in setting" :key="item.id">{{ item.name }}</li>
      </ul>
      <div class="h-[30px]">
        <select v-model="lng" @change="change()">
          <option value="en">english</option>
          <option value="zh-CN">中文</option>
        </select>
      </div>
    </div>

  </div>
</template>

<style scoped>

</style>