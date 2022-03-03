<template>
  <el-dialog width="360px" :title="lang().title" v-model="visible" :close-on-click-modal="false">
    <el-form ref="form" label-width="80px" :model="form">
      <el-form-item prop="t" :label="lang().form.type"
                    :rules="{required:true,message:lang().ruleMessage.type}">
        <el-select clearable style="width: 100%" v-model="form.t">
          <el-option label="string" value="string"></el-option>
          <el-option label="list" value="list"></el-option>
          <el-option label="hash" value="hash"></el-option>
          <el-option label="set" value="set"></el-option>
          <el-option label="zset" value="zset"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item prop="key" :label="lang().form.key" :rules="{required:true,message:lang().ruleMessage.key}">
        <el-input clearable v-model="form.key" :placeholder="lang().form.key"></el-input>
      </el-form-item>
      <el-form-item prop="field" :label="lang().form.field"
                    :rules="{required:true,message:lang().ruleMessage.field}" v-if="form.t === 'hash'">
        <el-input clearable v-model="form.field" :placeholder="lang().form.field"></el-input>
      </el-form-item>
      <el-form-item prop="member" :label="lang().form.member"
                    :rules="{required:true,message:lang().ruleMessage.member}" v-if="form.t === 'zset'">
        <el-input clearable v-model="form.member" :placeholder="lang().form.member"></el-input>
      </el-form-item>
      <el-form-item prop="value" :label="lang().form.value"
                    :rules="{required:true,message:lang().ruleMessage.value}">
        <el-input clearable v-model="form.value" :placeholder="lang().form.value"></el-input>
      </el-form-item>
      <el-form-item prop="time_type" :label="lang().form.timeType" v-if="form.t === 'string'">
        <el-select clearable v-model="form.timeType" style="width: 100%">
          <el-option :label="lang().form.timeTypeOption.s" value="ttl"></el-option>
          <el-option :label="lang().form.timeTypeOption.ms" value="pttl"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item prop="expire" :label="lang().form.expire"
                    v-if="form.t === 'string'">
        <el-input v-model="form.expire" clearable :placeholder="lang().form.expirePlaceHolder"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="submitFn">{{ lang().form.button.submit }}</el-button>
        <el-button type="danger" @click="visible = false">{{lang().form.button.cancel}}</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri';
import {mapGetters} from "vuex";

export default {
  name: 'newkey',
  data() {
    return {
      visible: false,
      key: '',
      form: {}
    }
  },
  computed: {
    ...mapGetters(['i18n'])
  },
  methods: {
    lang() {
      return this.i18n.mainPage.content.dataPage.dialog
    },
    initFormData() {
      this.form = {
        t: 'string',
        value: '',
        key: '',
        field: '',
        member: '',
        timeType: 'ttl',
        expire: ''
      }
    },
    show(key) {
      this.initFormData()
      this.key = key
      this.visible = true
      this.$nextTick(() => {
        this.$refs.form.clearValidate()
      })
    },
    submitFn() {
      this.$refs.form.validate(valid => {
        if (!valid) return
        invoke('set_new_key', this.form)
            .then(res => {
              if (res.code !== 200) {
                this.alert.error(res.msg)
              } else {
                this.$parent.autoFilterFn()
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