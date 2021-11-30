<template>
  <el-dialog width="360px" title="添加新key" v-model="visible" :close-on-click-modal="false">
    <el-form ref="form" label-width="80px" :model="form">
      <el-form-item prop="t" label="类型" :rules="{required:true,message:'请选择类型'}">
        <el-select clearable style="width: 100%" v-model="form.t">
          <el-option label="string" value="string"></el-option>
          <el-option label="list" value="list"></el-option>
          <el-option label="hash" value="hash"></el-option>
          <el-option label="set" value="set"></el-option>
          <el-option label="zset" value="zset"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item prop="key" label="key" :rules="{required:true,message:'请输入key'}">
        <el-input clearable v-model="form.key" placeholder="请输入key"></el-input>
      </el-form-item>
      <el-form-item prop="field" label="字段" :rules="{required:true,message:'请输入字段'}" v-if="form.t === 'hash'">
        <el-input clearable v-model="form.field" placeholder="请输入字段"></el-input>
      </el-form-item>
      <el-form-item prop="member" label="成员" :rules="{required:true,message:'请输入成员'}" v-if="form.t === 'zset'">
        <el-input clearable v-model="form.member" placeholder="请输入成员"></el-input>
      </el-form-item>
      <el-form-item prop="value" label="值" :rules="{required:true,message:'请输入值'}">
        <el-input clearable v-model="form.value" placeholder="请输入值"></el-input>
      </el-form-item>
      <el-form-item prop="time_type" label="时间类型" v-if="form.t === 'string'">
        <el-select clearable v-model="form.timeType" style="width: 100%">
          <el-option label="秒" value="ttl"></el-option>
          <el-option label="毫秒" value="pttl"></el-option>
        </el-select>
      </el-form-item>
      <el-form-item prop="expire" label="过期时间"
                    v-if="form.t === 'string'">
        <el-input v-model="form.expire" clearable placeholder="默认不过期"></el-input>
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
  name: 'newkey',
  data() {
    return {
      visible: false,
      key: '',
      form: {}
    }
  },
  methods: {
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
      this.$nextTick(()=>{
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