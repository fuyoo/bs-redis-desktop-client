<template>
  <el-dialog width="360px"
             :close-on-click-modal="false"
             :close-on-press-escape="false"
             v-model="visible"
             :title="mode === 'add' ? lang().dialog.title.add : lang().dialog.title.edit">
    <el-form label-width="80px">
      <el-form-item :label="lang().dialog.form.name">
        <el-input v-model="form.name" :placeholder="lang().dialog.form.name"></el-input>
      </el-form-item>
      <el-form-item :label="lang().dialog.form.host">
        <el-input v-model="form.address" :placeholder="lang().dialog.form.host"></el-input>
      </el-form-item>
      <el-form-item :label="lang().dialog.form.port">
        <el-input v-model="form.port" :placeholder="lang().dialog.form.port"></el-input>
      </el-form-item>
      <el-form-item :label="lang().dialog.form.username">
        <el-input v-model="form.username" :placeholder="lang().dialog.form.username"></el-input>
      </el-form-item>
      <el-form-item :label="lang().dialog.form.password">
        <el-input v-model="form.password" :placeholder="lang().dialog.form.password"></el-input>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" :loading="loading" @click="submitFn">{{ lang().dialog.form.submit }}</el-button>
        <el-button type="primary" :loading="loading" @click="testFn">{{ lang().dialog.form.test }}</el-button>
        <el-button type="primary" :loading="loading" @click="visible = false">{{ lang().dialog.form.cancel }}</el-button>
      </el-form-item>
    </el-form>
  </el-dialog>
</template>

<script>
import {invoke} from '@tauri-apps/api'
import {mapGetters} from "vuex";

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
  computed:{
    ...mapGetters(['i18n'])
  },
  methods: {
    lang(){
      return this.i18n.mainPage.menu
    },
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
        invoke('connection_create', {map: this.form})
            .then(res => {
              if (res.code == 200) {
                this.alert.success(this.lang().dialog.AddSuccessMsg)
                this.visible = false
                this.$store.dispatch('getConnectionList')
              }
            })
            .finally(() => this.loading = false)
      }
      if (this.mode === 'edit') {
        this.loading = true
        invoke('connection_edit', {map: this.form})
            .then(res => {
              if (res.code == 200) {
                this.alert.success(this.lang().dialog.EditSuccessMsg)
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
              this.alert.success(this.lang().dialog.TestSuccessMsg)
            } else {
              this.alert.warning(this.lang().dialog.TestErrMsg)
            }
          })
          .finally(() => this.loading = false)
    }
  },
}
</script>

<style scoped>

</style>
