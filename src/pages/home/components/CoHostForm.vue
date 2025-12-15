<script lang="ts" setup>
import { type ConnectionHost, db } from '@/db'
import { dialog, message } from '@/tools'
import { reactive, shallowRef, toRaw } from 'vue'
import type { FormInst, FormItemRule, FormRules } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { request } from '@/api'
import { useTab } from '@/hooks/tab'
// i18n helper
const { t } = useI18n()
// props
const props = defineProps<{
  data?: ConnectionHost
}>()
// tab change handler
const { change } = useTab(t)
// form model
const formModel = ref<ConnectionHost>({
  name: '',
  node: [
    {
      host: '',
    },
  ],
  cluster: false,
  ...toRaw(props.data),
})
// get pure data, non proxy
const pureData = () => {
  return JSON.parse(JSON.stringify(formModel.value))
}
// reset form
const resetForm = () => {
  formModel.value = {
    name: '',
    node: [
      {
        host: '',
        port: '',
        db: '',
        username: '',
        password: '',
      },
    ],
    cluster: false,
  }
}
// listen data change,update form model
watch(
  () => props.data,
  (val) => {
    if (!val) {
      resetForm()
      return
    }
    formModel.value = { ...val }
    formRef.value?.restoreValidation()
  },
)
// form  ref
const formRef = shallowRef<FormInst | null>(null)
// rules
const rules: FormRules = {
  name: [
    {
      required: true,
      validator(rule: FormItemRule, value: string) {
        if (!value || value.length === 0) {
          return new Error(t('home.form.rule.0'))
        }
        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
  'node.0.host': [
    {
      required: true,
      validator(rule: FormItemRule, value: string) {
        if (!value || value.length === 0) {
          return new Error(t('home.form.rule.1'))
        }
        return true
      },
      trigger: ['input', 'blur'],
    },
  ],
}
// loading flag
const loading = reactive({
  test: false,
  submit: false,
  connect: false,
})

// submit new host
const submitFn = async () => {
  await formRef.value?.validate()
  const v = pureData()
  await db.hosts.put(v)
  // if create, reset form
  if (!props.data?.id) {
    resetForm()
  }
}

//checking connection status
const checkStatus = async () => {
  const v = pureData()
  const { code } = await request({
    path: `/status`,
    connectionInfo: v,
    data: '',
  })
  return code
}
// test connection handler
const testConnection = async () => {
  await formRef.value?.validate()
  try {
    loading.test = true
    const code = await checkStatus()
    if (code === 0) message.success("ok")
  } finally {
    loading.test = false
  }
}

// connect a host.
const connectTo = async () => {
  await formRef.value?.validate()
  try {
    loading.connect = true
    await checkStatus()
    const v = pureData()
    change(v)
  } finally {
    loading.connect = false
  }
}

// delete host handler
const handlePositive = async (id?: number) => {
  await db.hosts.delete(id)
}

</script>

<template>
  <n-form ref="formRef" :model="formModel" :rules="rules" label-placement="left" label-width="100">
    <n-form-item path="name" :label="$t('home.form.label[0]')">
      <n-input v-model:value="formModel.name" :placeholder="$t('home.form.hint[0]')" />
    </n-form-item>

    <n-form-item path="node.0.host" :label="$t('home.form.label[1]')">
      <n-input v-model:value="formModel.node[0].host" :placeholder="$t('home.form.hint[1]')" />
    </n-form-item>

    <n-form-item :label="$t('home.form.label[2]')">
      <n-input v-model:value="formModel.node[0].port" :placeholder="$t('home.form.hint[2]')" />
    </n-form-item>

    <n-form-item :label="$t('home.form.label[3]')">
      <n-input v-model:value="formModel.node[0].db" :placeholder="$t('home.form.hint[3]')" />
    </n-form-item>

    <n-form-item :label="$t('home.form.label[4]')">
      <n-input v-model:value="formModel.node[0].username" :placeholder="$t('home.form.hint[4]')" />
    </n-form-item>

    <n-form-item :label="$t('home.form.label[5]')">
      <n-input v-model:value="formModel.node[0].password" type="password" show-password-on="click"
        :placeholder="$t('home.form.hint[5]')" />
    </n-form-item>

    <!--    <n-form-item :label="$t('home.form.label[6]')">
      <n-switch v-model:value="formModel.cluster" />
    </n-form-item>-->
    <n-space justify="center">
      <n-button :loading="loading.test" tertiary @click="testConnection">
        {{ $t('actions[7]') }}
      </n-button>
      <n-button @click="connectTo" :loading="loading.connect" v-if="props.data?.id" type="primary">
        {{ $t('actions[8]') }}
      </n-button>
      <n-button type="primary" @click="submitFn" v-if="props.data?.id">
        {{ $t('actions[9]') }}
      </n-button>
      <n-button type="primary" @click="submitFn" v-else>
        {{ $t('actions[0]') }}
      </n-button>
      <n-popconfirm @positive-click="() => handlePositive(props.data?.id)" v-if="props.data?.id">
        <template #trigger>
          <n-button type="warning">
            {{ $t('actions[2]') }}
          </n-button>
        </template>
        {{ $t('tips[3]') }}
      </n-popconfirm>
    </n-space>
  </n-form>
</template>
