<script setup lang="ts">
import {ref, watch} from "vue";
import {DrawerMode} from "@/interface.ts";
import db, {ConnectionImpl, ConnectionInfo} from "@/database.ts";

const emits = defineEmits<{
  ok: [data: ConnectionImpl],
}>()

const props = defineProps<{
  mode: DrawerMode,
  data?: ConnectionImpl
}>()
const formRef = ref<any>()
const unProxy = (r: any): ConnectionImpl => {
  const d = {...(r || {name: '', node: [{} as ConnectionInfo]} as ConnectionImpl)}
  d.node = [...d.node].map(e => ({...e}))
  return d
}
const formModel = ref<ConnectionImpl>(unProxy(props.data))
watch(() => props.data, (data) => {
  formModel.value = unProxy(data)
  formRef.value?.clearValidate()
})
const submitFn = async (v: any) => {
  const data = unProxy(v)
  await db.connection.put(data)
  emits("ok", data)
}
const save = async () => {
  const res = await formRef.value?.validate()
  if (!res) {
    await submitFn(formModel.value)
  }
}
defineExpose({
  save
})
</script>

<template>
  <div class="p-25px">
    <a-form auto-label-width :model="formModel" @submit-success="submitFn" ref="formRef">
      <a-card class="rounded-lg" hoverable :body-style="{padding:'10px 15px 0 15px'}">
        <template #title>
          <span>{{ $t('layout.hostForm.name') }}</span>
        </template>
        <a-form-item required field="name" :rules="{message:$t('layout.hostForm.hostName.placeholder'),required:true}">
          <template #label>
            <div class="i-tabler:server-bolt text-24px"></div>
          </template>
          <a-input v-model="formModel.name" :placeholder="$t('layout.hostForm.hostName.placeholder')"></a-input>
        </a-form-item>
      </a-card>
      <br>
      <a-card :title="$t('layout.hostForm.normal')" hoverable>
        <a-form-item required field="node.0.host"
                     :rules="{message:$t('layout.hostForm.host.placeholder'),required:true}">
          <template #label>
            <div class="i-iconoir:network-left-solid"></div>
          </template>
          <a-input v-model="formModel.node[0].host" :placeholder="$t('layout.hostForm.host.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-material-symbols:power-plug"></div>
          </template>
          <a-input v-model="formModel.node[0].port" :placeholder="$t('layout.hostForm.port.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-ic:round-sd-storage"></div>
          </template>
          <a-input v-model="formModel.node[0].db" :placeholder="$t('layout.hostForm.db.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-ic:round-person"></div>
          </template>
          <a-input v-model="formModel.node[0].username"
                   :placeholder="$t('layout.hostForm.username.placeholder')"></a-input>
        </a-form-item>
        <a-form-item>
          <template #label>
            <div class="i-ic:round-lock"></div>
          </template>
          <a-input-password v-model="formModel.node[0].password"
                            :placeholder="$t('layout.hostForm.password.placeholder')"></a-input-password>
        </a-form-item>
      </a-card>
      <br>
      <a-button html-type="submit" type="primary">{{ $t('layout.hostForm.save') }}</a-button>
    </a-form>

  </div>
</template>

<style scoped lang="scss">
.p-25px {
  &::v-deep(.arco-form-item-label-col) {
    // display: none;
  }
}
</style>