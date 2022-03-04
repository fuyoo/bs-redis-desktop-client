<template>
  <el-dialog width="360px" :title="lang.title" v-model="visible" :close-on-click-modal="false">
    <el-form ref="form" label-width="80px" :model="form">
      <el-form-item prop="key" :label="lang.form.key" :rules="{required:true,message:lang.formRuleMsg.key}">
        <el-input clearable v-model="form.key" :readonly="true" :placeholder="lang.key"></el-input>
      </el-form-item>
      <el-form-item prop="t" :label="lang.form.timeType" :rules="{required:true,message:lang.formRuleMsg.timeType}">
        <el-select v-model="form.t" style="width:100%">
          <el-option value="ttl" :label="lang.form.timeTypeOption.s"></el-option>
          <el-option value="pttl" :label="lang.form.timeTypeOption.ms"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item prop="expire" :label="lang.form.expire" :rules="{required:true,message:lang.formRuleMsg.expire}">
        <el-input clearable v-model="form.expire" :placeholder="lang.form.expirePlaceHolder"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="submitFn">{{ lang.form.submit }}</el-button>
        <el-button type="danger" @click="visible = false">{{lang.form.cancel}}</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri';
import {mapGetters} from "vuex";

export default {
  name: 'ttl',
  data() {
    return {
      visible: false,
      form: {}
    }
  },
  computed: {
    ...mapGetters(["i18n"]),
    lang() {
      return this.i18n.mainPage.content.dataPage.ttl
    }
  },
  methods: {
    initFormData() {
      this.form = {
        expire: '',
        t: 'ttl',
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
        invoke('set_key_ttl', this.form)
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