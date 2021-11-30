<template>
  <el-dialog width="360px" title="设置时间" v-model="visible" :close-on-click-modal="false">
    <el-form ref="form" label-width="80px" :model="form">
      <el-form-item prop="key" label="key" :rules="{required:true,message:'请输入key'}">
        <el-input clearable v-model="form.key" :readonly="true" placeholder="请输入key"></el-input>
      </el-form-item>
      <el-form-item prop="t" label="时间类型" :rules="{required:true,message:'请选择时间类型'}">
        <el-select v-model="form.t" style="width:100%">
          <el-option value="ttl" label="秒"></el-option>
          <el-option value="pttl" label="毫秒"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item prop="expire" label="时间" :rules="{required:true,message:'请输入时间'}">
        <el-input clearable v-model="form.expire" placeholder="-1取消过期时间"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="submitFn">提交</el-button>
        <el-button type="danger" @click="visible = false">取消</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri';

export default {
  name: 'ttl',
  data() {
    return {
      visible: false,
      form: {}
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