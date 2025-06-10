<script lang="ts" setup>
// adapt quasar dialog. custom component as quasar dialog content.
import { type ConnectionHost, db } from '@/db'
import { useDialogPluginComponent } from 'quasar'
import { reactive, shallowRef, toRaw } from 'vue'
defineEmits([
  ...useDialogPluginComponent.emits,
])
const props = defineProps<{
  data?: ConnectionHost
}>()
//why have below code? refrence https://quasar.dev/quasar-plugins/dialog#writing-the-custom-component
const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } = useDialogPluginComponent()
const formModel = reactive<ConnectionHost>({
  name: '',
  node: [
    {
      host: '',
    },
  ],
  cluster: false,
  ...(toRaw(props.data)),
})
const formRef = shallowRef<any>()

console.log(db.hosts)
const submitFn = async () => {
  const r = await formRef.value?.validate()
  if (!r) return
  await db.hosts.put(toRaw(formModel))
  onDialogOK(toRaw(formModel))
}
</script>
<template>
  <div>
    <q-dialog ref="dialogRef" @hide="onDialogHide">
      <q-card class="q-dialog-plugin">
        <q-bar>
          <span></span>
          <q-space />
          <q-btn dense flat icon="close" v-close-popup> </q-btn>
        </q-bar>

        <q-scroll-area class="h-460px">
          <q-form ref="formRef" class="q-gutter-md p-4">
            <q-input v-model="formModel.name" filled :label="$t('home.form.label[0]')" :hint="$t('home.form.hint[0]')"
              lazy-rules :rules="[(val) => (val && val.length > 0) || $t('home.form.rule[0]')]" />
            <q-input v-model="formModel.node[0].host" filled :label="$t('home.form.label[1]')"
              :hint="$t('home.form.hint[1]')" lazy-rules
              :rules="[(val) => (val && val.length > 0) || $t('home.form.rule[1]')]" />
            <q-input v-model="formModel.node[0].port" filled :label="$t('home.form.label[2]')"
              :hint="$t('home.form.hint[2]')" />
            <q-input v-model="formModel.node[0].db" filled :label="$t('home.form.label[3]')"
              :hint="$t('home.form.hint[3]')" />
            <q-input v-model="formModel.node[0].username" filled :label="$t('home.form.label[4]')"
              :hint="$t('home.form.hint[4]')" />
            <q-input v-model="formModel.node[0].password" filled :label="$t('home.form.label[5]')"
              :hint="$t('home.form.hint[5]')" />
            <q-toggle v-model="formModel.cluster" :label="$t('home.form.label[6]')" />
          </q-form>
        </q-scroll-area>
        <!-- buttons example -->
        <q-card-actions align="right">
          <q-btn color="primary" :label="$t('actions[0]')" @click="submitFn" />
          <q-btn color="primary" :label="$t('actions[1]')" @click="onDialogCancel" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>
<style lang="scss"></style>
