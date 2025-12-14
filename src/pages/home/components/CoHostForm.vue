<script lang="ts" setup>
import { type ConnectionHost, db } from '@/db'
import { dialog } from '@/tools'
import { reactive, shallowRef, toRaw } from 'vue'
import type { FormInst, FormItemRule, FormRules } from 'naive-ui'
import { useI18n } from 'vue-i18n'
import { useTabStore } from '@/stores/tab.ts'
const { t } = useI18n()
const props = defineProps<{
  data?: ConnectionHost
}>()

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
const formRef = shallowRef<FormInst | null>(null)

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
const loading = reactive({
  test: false,
  submit: false,
})
const submitFn = async () => {
  await formRef.value?.validate()
  console.log(formModel)
  const v = JSON.parse(JSON.stringify(formModel.value))
  await db.hosts.put(v)
  resetForm()
}
const testConnection = async () => {
  await formRef.value?.validate()
  try {
    loading.test = true
  } finally {
    loading.test = false
  }
}
const doConnection = async () => {
  await formRef.value?.validate()
  try {
    loading.submit = true
  } finally {
    loading.submit = false
  }
}
const handlePositive = async (id: number) => {
  await db.hosts.delete(id)
}
const tab = useTabStore()
const connectTo = async () => {
  await tab.change({ id: ''+(props.data?.id || 0), name: props.data?.name })
  console.log('connect to', props.data)
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
      <n-input
        v-model:value="formModel.node[0].password"
        type="password"
        :placeholder="$t('home.form.hint[5]')"
      />
    </n-form-item>

    <!--    <n-form-item :label="$t('home.form.label[6]')">
      <n-switch v-model:value="formModel.cluster" />
    </n-form-item>-->
    <n-space justify="center">
      <n-button tertiary @click="testConnection">
        {{ $t('actions[7]') }}
      </n-button>
      <n-button @click="doConnection" v-if="props.data?.id" type="primary">
        {{ $t('actions[8]') }}
      </n-button>
      <n-button type="primary" @click="submitFn" v-if="props.data?.id">
        {{ $t('actions[3]') }}
      </n-button>
      <n-button type="primary" @click="submitFn" v-else>
        {{ $t('actions[0]') }}
      </n-button>
      <n-popconfirm @positive-click="() => handlePositive(props.data?.id)" v-if="props.data?.id">
        <template #trigger>
          <n-button @click="connectTo" type="warning">
            {{ $t('actions[2]') }}
          </n-button>
        </template>
        {{ $t('tips[3]') }}
      </n-popconfirm>
    </n-space>
  </n-form>
</template>
