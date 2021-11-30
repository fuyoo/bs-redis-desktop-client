<template>
  <el-dialog width="360px"
             :close-on-click-modal="false"
             :close-on-press-escape="false"
             v-model="visible" :title="mode === 'add' ? '添加新连接': '编辑连接'">
    <el-form label-width="80px">
      <el-form-item label="连接名称">
        <el-input v-model="form.name" placeholder="请输入连接名称"></el-input>
      </el-form-item>
      <el-form-item label="连接地址">
        <el-input v-model="form.address" placeholder="请输入连接地址"></el-input>
      </el-form-item>
      <el-form-item label="端口">
        <el-input v-model="form.port" placeholder="请输入端口"></el-input>
      </el-form-item>
      <el-form-item label="用户名">
        <el-input v-model="form.username" placeholder="请输入密码"></el-input>
      </el-form-item>
      <el-form-item label="密码">
        <el-input v-model="form.password" placeholder="请输入密码"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" :loading="loading" @click="submitFn">提交</el-button>
        <el-button type="primary" :loading="loading" @click="testFn">测试</el-button>
        <el-button type="primary" :loading="loading" @click="visible = false">取消</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>

<script>
import { invoke } from '@tauri-apps/api'

export default {
  name: 'Dialog',
  data() {
    return {
      visible: false,
      mode: 'add',
      form: {
        id: '',
        name: '',
        address: '',
        port: '',
        username: '',
        password: ''
      },

      loading: false
    }
  },
  methods: {
    show(mode, data) {
      this.mode = mode || 'add'
      this.form = {
        id: '',
        name: '',
        address: '',
        port: '',
        username: '',
        password: ''
      }
      this.visible = true
      if (mode === 'edit') {
        this.form = data
      }
    },
    submitFn() {
      if (this.mode === 'add') {
        this.loading = true
        invoke('connection_create', { map: this.form })
            .then(res => {
              if (res.code == 200) {
                this.alert.success('操作成功')
                this.visible = false
                this.$store.dispatch('getConnectionList')
              }
            })
            .finally(() => this.loading = false)
      }
      if (this.mode === 'edit') {
        this.loading = true
        invoke('connection_edit', { map: this.form })
            .then(res => {
              if (res.code == 200) {
                this.alert.success('修改成功')
                this.visible = false
                this.$store.dispatch('getConnectionList')
              }
            })
            .finally(() => this.loading = false)
      }
    },
    testFn() {
      this.loading = true
      invoke('test_redis_connection', this.form)
          .then(res => {
            if (res.code === 200) {
              this.alert.success(res.msg)
            } else {
              this.alert.warning(res.msg)
            }
          })
          .finally(() => this.loading = false)
    }
  }
}
</script>

<style scoped>

</style>
