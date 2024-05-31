<template>
  <SettingsMenuRow>
      <template #label>
        <div><div class="i-cil:language"></div> {{$t("settings.lang" as any)}}</div>
      </template>
      <template #content>
        <div class="w-120px">
            <a-select v-model="lng">
                <a-option v-for="item in Languages" :value="item.value" :label="item.label"></a-option>
            </a-select>
        </div>
      </template>
  </SettingsMenuRow>
</template>

<script lang="ts" setup>
import SettingsMenuRow from "@/views/SettingsView/SettingsMenuRow.vue";
import {Languages} from "@/i18n";
import {ref, watch} from "vue";
import i18next from "i18next"
import database, {SettingsImpl} from "@/database.ts"
let data = {lang:i18next.language} as SettingsImpl
const lng = ref(data.lang )
database.settings.get(1)
    .then((v) => {
        data = v || {id:1}
    })
watch(() => lng.value,async (r)=>{
    await i18next.changeLanguage(r)
    try {
            data.lang = r
       database.settings.put(data,1)
    } catch (e) {
      console.log(e)
    }
})
</script>

<style scoped>

</style>