<script lang="ts" setup>
// adapt quasar dialog. custom component as quasar dialog content.
import { useDialogPluginComponent } from 'quasar'
import { reactive } from 'vue';
defineEmits([
  // REQUIRED; need to specify some events that your
  // component will emit through useDialogPluginComponent()
  ...useDialogPluginComponent.emits
])
//why have below code? refrence https://quasar.dev/quasar-plugins/dialog#writing-the-custom-component
const { dialogRef, onDialogHide, onDialogOK, onDialogCancel } = useDialogPluginComponent()
function onOKClick() {
  // on OK, it is REQUIRED to
  // call onDialogOK (with optional payload)
  //onDialogOK()
  // or with payload: onDialogOK({ ... })
  // ...and it will also hide the dialog automatically
}
const formModel = reactive({
  name: '',
  addr: '',
  cluster: false
})
</script>
<template>
  <div>
    <q-dialog ref="dialogRef" @hide="onDialogHide">

      <q-card class="q-dialog-plugin">
        <q-bar>
          <span></span>
          <q-space />
          <q-btn dense flat icon="close" v-close-popup>
          </q-btn>
        </q-bar>
        <q-form class="q-gutter-md  p-4">
          <q-input v-model="formModel.name" filled label="Your name *" hint="Name and surname" lazy-rules
            :rules="[val => val && val.length > 0 || 'Please type something']" />

          <q-input v-model="formModel.addr" filled type="number" label="Your age *" lazy-rules :rules="[
            val => val !== null && val !== '' || 'Please type your age',
            val => val > 0 && val < 100 || 'Please type a real age'
          ]" />
          <q-toggle v-model="formModel.cluster" label="I accept the license and terms" />
        </q-form>
        <!-- buttons example -->
        <q-card-actions align="right">
          <q-btn color="primary" :label="$t('actions[0]')" @click="onOKClick" />
          <q-btn color="primary" :label="$t('actions[1]')" @click="onDialogCancel" />
        </q-card-actions>
      </q-card>
    </q-dialog>
  </div>
</template>
<style lang="scss"></style>
