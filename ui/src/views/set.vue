<template>
  <el-scrollbar style="height: 100vh">
    <el-form class="form" ref="form" :model="form" label-width="80px">
      <el-form-item label="自动刷新">
        <el-switch v-model="form.autoRefresh"></el-switch>
      </el-form-item>
      <el-form-item label="刷新时间" v-if="form.autoRefresh">
        <el-input type="number" v-model="form.autoRefreshTime" placeholder="默认3秒刷新一次"></el-input>
      </el-form-item>
      <el-form-item label="Pub/Sub">
        <el-switch v-model="form.pubsub"></el-switch>
      </el-form-item>
      <el-form-item>
        <el-button type="primary" @click="onSubmit">保存</el-button>
        <el-button @click="cancelFn">取消</el-button>
      </el-form-item>
    </el-form>
  </el-scrollbar>
</template>

<script>
import {request} from ':/tools/invoke';
import {getCurrent} from '@tauri-apps/api/window';

export default {
  name: 'set',
  data() {
    return {
      form: {},
    }
  },
  created() {
    getCurrent().show()
    this.getSysInfo()
  },
  mounted() {
  },
  methods: {
    getSysInfo() {
      request('query_sys_info')
          .then(res => {
            this.form = res.data
          })
    },
    onSubmit() {
      this.form.autoRefreshTime = Number( this.form.autoRefreshTime)
      request('update_sys_info', this.form)
          .then(res => {
            this.alert.success(res.msg)
          })
    },
    cancelFn() {
      getCurrent().close()
    }
  },
}
</script>

<style scoped>
.form {
  padding: 20px;
}
</style>