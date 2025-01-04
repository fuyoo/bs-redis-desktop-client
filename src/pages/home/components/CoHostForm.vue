<script lang="ts" setup>
// adapt quasar dialog. custom component as quasar dialog content.
import { useDialogPluginComponent } from 'quasar'
import { reactive, shallowRef } from 'vue'
defineEmits([
  // REQUIRED; need to specify some events that your
  // component will emit through useDialogPluginComponent()
  ...useDialogPluginComponent.emits,
])
//why have below code? refrence https://quasar.dev/quasar-plugins/dialog#writing-the-custom-component
const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } = useDialogPluginComponent()
function onSubmit() {
  // on OK, it is REQUIRED to
  // call onDialogOK (with optional payload)
  //onDialogOK()
  // or with payload: onDialogOK({ ... })
  // ...and it will also hide the dialog automatically
  console.log(formModel)
}
const formModel = reactive({
  name: '',
  addr: '',
  node: [
    {
      host: '',
    },
  ] as { host: string; port?: string; db?: string; username?: string; password?: string }[],
  cluster: false,
})
const formRef = shallowRef<any>()
const submit = async () => {
  await formRef.value?.validate()
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
          <q-form ref="formRef" @submit="onSubmit" class="q-gutter-md p-4">
            <q-input
              v-model="formModel.name"
              filled
              :label="$t('home.form.lable[0]')"
              :hint="$t('home.form.hint[0]')"
              lazy-rules
              :rules="[(val) => (val && val.length > 0) || $t('home.form.rule[0]')]"
            />
            <q-input
              v-model="formModel.node[0].host"
              filled
              :label="$t('home.form.lable[1]')"
              :hint="$t('home.form.hint[1]')"
              lazy-rules
              :rules="[(val) => (val && val.length > 0) || $t('home.form.rule[1]')]"
            />
            <q-input
              v-model="formModel.node[0].port"
              filled
              :label="$t('home.form.lable[2]')"
              :hint="$t('home.form.hint[2]')"
            />
            <q-input
              v-model="formModel.node[0].db"
              filled
              :label="$t('home.form.lable[3]')"
              :hint="$t('home.form.hint[3]')"
            />
            <q-input
              v-model="formModel.node[0].username"
              filled
              :label="$t('home.form.lable[4]')"
              :hint="$t('home.form.hint[4]')"
            />
            <q-input
              v-model="formModel.node[0].password"
              filled
              :label="$t('home.form.lable[5]')"
              :hint="$t('home.form.hint[5]')"
            />
            <q-toggle v-model="formModel.cluster" :label="$t('home.form.lable[6]')" />
          </q-form>
        </q-scroll-area>
        <!-- buttons example -->
        <q-card-actions align="right">
          <q-btn color="primary" :label="$t('actions[0]')" @click="submit" />
          <q-btn color="primary" :label="$t('actions[1]')" @click="onDialogCancel" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>
<style lang="scss"></style>
