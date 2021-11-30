<template>
  <el-dialog width="360px" title="重命名key" v-model="visible" :close-on-click-modal="false">
    <el-form ref="form" label-width="80px" :model="form">
      <el-form-item prop="key" label="key" :rules="{required:true,message:'请输入key'}">
        <el-input clearable v-model="form.key" :readonly="true" placeholder="请输入key"></el-input>
      </el-form-item>
      <el-form-item prop="newKey" label="新key" :rules="{required:true,message:'请输入新key'}">
        <el-input clearable v-model="form.newKey" placeholder="请输入新key"></el-input>
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
  name: 'rename',
  data() {
    return {
      visible: false,
      form: {}
    }
  },
  methods: {
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
      this.$nextTick(()=>{
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