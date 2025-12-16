<script setup lang="ts">
import { useI18n } from "vue-i18n";
import { type FormInst } from "naive-ui"
import { useReqStore } from "@/stores/req.ts";
import { dialog } from '@/tools'
import { message } from "@/tools"
const { t } = useI18n()
const form = reactive({
  key: "",
  expires: 0,
  field: "",
  data: ""
})
const formRef = shallowRef<FormInst | null>()
const rules = {
  key: {
    required: true,
    renderMessage: () => t("keyForm.msg.0")
  },
  data: {
    required: true,
    renderMessage: () => t("keyForm.msg.1")
  },
  field: {
    required: true,
    renderMessage: () => t("keyForm.msg.2")
  }
}

const req = useReqStore()
const submitFn = async () => {
  await formRef.value?.validate()
  await req.reqWithHost<boolean>({
    path: "/cmd",
    data: ['hset', form.key, form.field, form.data]
  })
  if (form.expires > 0) {
    await req.reqWithHost<boolean>({
      path: "/cmd",
      data: ['expire', form.key, form.expires.toString()]
    })
  }

  dialog.destroyAll()
  message.success("operate success")
}
</script>

<template>
  <n-form class="pt-5" ref="formRef" label-placement="left" label-width="100px" size="small" :model="form"
    :rules="rules">
    <n-form-item :label="$t('keyForm.label.0')" path="key" required>
      <n-input clearable v-model:value="form.key"></n-input>
    </n-form-item>
    <n-form-item :label="$t('keyForm.label.3')" path="field" required>
      <n-input clearable type="textarea" v-model:value="form.field"></n-input>
    </n-form-item>
    <n-form-item :label="$t('keyForm.label.1')" path="data" required>
      <n-input clearable type="textarea" v-model:value="form.data"></n-input>
    </n-form-item>
    <n-form-item :label="$t('keyForm.label.2')">
      <n-input-number class="w-full" v-model:value="form.expires">
        <template #suffix>
          s
        </template>
      </n-input-number>
    </n-form-item>
    <n-form-item label=" ">
      <n-button type="primary" @click="submitFn">{{ $t('actions.0') }}</n-button>
    </n-form-item>
  </n-form>
</template>

<style scoped lang="scss"></style>
