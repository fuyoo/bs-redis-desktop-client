<template>
  <el-dialog width="360px" :title="lang().title" v-model="visible" :close-on-click-modal="false">
    <el-form ref="form" label-width="80px" :model="form">
      <el-form-item prop="key"
                    :label="lang().form.key" :rules="{required:true,message:lang().formRuleMsg.key}">
        <el-input clearable v-model="form.key" :readonly="true" :placeholder="lang().form.key"></el-input>
      </el-form-item>
      <el-form-item prop="newKey" :label="lang().form.newKey"
                    :rules="{required:true,message:lang().formRuleMsg.newKey}">
        <el-input clearable v-model="form.newKey" :placeholder="lang().form.newKey"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="submitFn">{{ lang().form.submit }}</el-button>
        <el-button type="danger" @click="visible = false">{{ lang().form.cancel }}</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri';
import {mapGetters} from "vuex";

export default {
  name: 'rename',
  data() {
    return {
      visible: false,
      form: {}
    }
  },
  computed: {
    ...mapGetters(['i18n'])
  },
  methods: {
    lang() {
      return this.i18n.mainPage.content.dataPage.renameDialog
    },
    initFormData() {
      this.form = {
        newKey: '',
        key: '',
      }
    },
    show(key) {
      this.initFormData()
      this.form.key = key
      this.visible = true
      this.$nextTick(() => {
        this.$refs.form.clearValidate()
      })
    },
    submitFn() {
      this.$refs.form.validate(valid => {
        if (!valid) return
        invoke('rename_key', this.form)
            .then(res => {
              if (res.code !== 200) {
                this.alert.error(res.msg)
              } else {
                this.$parent.autoFilterFn()
                this.$parent.key = this.form.newKey
                this.visible = false
                this.alert.success(res.msg)
              }
            })
      })
    }
  }
}
</script>

<style scoped>

</style>