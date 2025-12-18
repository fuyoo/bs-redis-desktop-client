<script setup lang="ts">
import { t } from "@/i18n";
import { type FormInst } from "naive-ui"
import { useReqStore } from "@/stores/req.ts";
const dialog = useDialog()
const { checkKeyIsExist } = useActions(dialog)
import { message } from "@/tools"
import { useActions } from "@/hooks/actions";
const form = reactive({
  key: "",
  expires: 0,
  data: "",
  score: "",
})
const formRef = shallowRef<FormInst | null>()
const rules = {
  key: {
    required: true,
    renderMessage: () => t("keyForm.msg.0")
  },
  data: {
    required: true,
    renderMessage: () => t("keyForm.msg.4")
  },
  score: {
    required: true,
    renderMessage: () => t("keyForm.msg.3")
  }
}

const req = useReqStore()
const submitFn = async () => {
  await checkKeyIsExist(form.key)
  await formRef.value?.validate()
  // ZADD key score1 member1 [score2 member2]
  await req.reqWithHost<boolean>({
    path: "/cmd",
    data: ['zadd', form.key, form.score, form.data]
  })
  if (form.expires > 0) {
    await req.reqWithHost<boolean>({
      path: "/cmd",
      data: ['expire', form.key, form.expires.toString()]
    })
  }

  dialog.destroyAll()
  message.success(t('tips.8'))
}
</script>

<template>
  <n-form class="pt-5" ref="formRef" label-placement="left" label-width="100px" size="small" :model="form"
    :rules="rules">
    <n-form-item :label="t('keyForm.label.0')" path="key" required>
      <n-input clearable v-model:value="form.key"></n-input>
    </n-form-item>
    <n-form-item :label="t('keyForm.label.5')" path="score" required>
      <n-input clearable v-model:value="form.score"></n-input>
    </n-form-item>
    <n-form-item :label="t('keyForm.label.6')" path="data" required>
      <n-input clearable type="textarea" v-model:value="form.data"></n-input>
    </n-form-item>

    <n-form-item :label="t('keyForm.label.2')">
      <n-input-number class="w-full" v-model:value="form.expires">
        <template #suffix>
          s
        </template>
      </n-input-number>
    </n-form-item>
    <n-form-item label=" ">
      <n-button type="primary" @click="submitFn">{{ t('actions.0') }}</n-button>
    </n-form-item>
  </n-form>
</template>

<style scoped lang="scss"></style>
