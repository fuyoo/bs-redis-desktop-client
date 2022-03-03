<template>
  <el-scrollbar style="height: calc(100vh - 30px)">
    <div class="base" v-loading="loading">
      <div class="list-item" v-for="item in some">
        <div class="icon">
          <icon :name="item.icon"></icon>
        </div>
        <div class="detail">
          <div class="row" v-for="item in item.list">
            <div class="col-key">{{ item.key }}</div>
            <div class="col-val">{{ item.val }}</div>
          </div>
        </div>
      </div>
      <div class="look-more">
        <el-button type="primary" @click="lookMoreFn">{{ i18n.mainPage.content.baseInfo.lookMore }}</el-button>
      </div>
    </div>
  </el-scrollbar>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri'
import {WebviewWindow} from '@tauri-apps/api/window'
import {mapGetters} from 'vuex'

export default {
  name: 'BaseInfo',
  data() {
    return {
      redisInfo: {},
      loading: true
    }
  },
  computed: {
    some() {
      try {
        function renderRow(type) {
          let keys = []
          switch (type) {
            case 'Server':
              keys = ['redis_version', 'redis_mode', 'config_file', 'uptime_in_days', 'tcp_port']
              break
            case 'Clients':
              keys = ['connected_clients', 'cluster_connections', 'maxclients', 'blocked_clients', 'clients_in_timeout_table']
              break
            case 'Memory':
              keys = [
                'used_memory_human',
                'used_memory_rss_human',
                'used_memory_peak_human',
                'total_system_memory_human',
                'maxmemory_human']
              break
            case 'Stats':
              keys = ['total_connections_received', 'total_commands_processed', 'instantaneous_ops_per_sec', 'total_net_input_bytes',
                'total_net_output_bytes', 'rejected_connections']
          }
          return keys
        }

        let res = [
          {type: 'Server', icon: 'server-fill'},
          {type: 'Clients', icon: 'window-fill'},
          {type: 'Memory', icon: 'cpu-line'},
          {type: 'Stats', icon: 'rss-fill'}
        ].map(item => {
          let list = renderRow(item.type).map(e => {
            return {
              key: e,
              val: this.redisInfo[item.type][e] || ''
            }
          })
          return {
            icon: item.icon,
            list
          }
        })
        return res
      } catch (e) {
        return false
      }
    },
    ...mapGetters(['clientInfo','i18n'])
  },
  watch: {
    clientInfo() {
      this.getRedisInfo()
    }
  },
  created() {
    this.getRedisInfo()
  },
  methods: {
    /**
     * 获取redis信息
     * @returns {{}}
     */
    getRedisInfo() {
      invoke('get_redis_info')
          .then(res => {
            if (res.code === 200) {
            }
            let cfg = res.data
            if (!cfg) return
            let tmp = {}
            cfg.split('#').map(e => e.split('\r\n').filter(v => v !== ''))
                .forEach(e => {
                  if (e[0] == undefined) {
                    return
                  }
                  try {
                    let key = e[0].replace(/\s/, '')
                    tmp[key] = {}
                    e.slice(1).forEach(item => {
                      let data = item.split(':')
                      if (data[1].indexOf(',') > -1) {
                        let child = data[1].split(',')
                        let map = {}
                        child.map(val => {
                          let s = val.split('=')
                          map[s[0]] = s[1]
                        })
                        tmp[key][data[0]] = map
                        return
                      }
                      if (data[1].indexOf('=') > -1) {
                        let v = data[1].split('=')
                        tmp[key][data[0]] = {[v[0]]: v[1]}
                        return
                      }
                      tmp[key][data[0]] = data[1]
                    })
                  } catch (e) {
                    console.error(e.message)
                  }
                })
            this.redisInfo = tmp
          })
          .finally(() => this.loading = false)
    },
    lookMoreFn() {
      new WebviewWindow('redis-config-details', {
        url: '/#/detail',
        title: this.i18n.mainPage.content.baseInfo.newWindowTitle,
        'center': true,
        skipTaskbar: true,
        alwaysOnTop: true
      })
    }
  }
}
</script>

<style scoped lang="scss">
.base {
  padding: 15px;

  .list-item {
    display: flex;
    justify-content: flex-start;
    align-items: center;
    height: 100%;
    background-color: #f8f8f8;
    border: 1px solid $border;
    border-radius: 4px;
    overflow: hidden;

    .icon {
      width: 148px;
      display: flex;
      justify-content: center;
      align-items: center;
      font-size: 60px;
      color: #333;
    }

    .detail {
      display: flex;
      flex-direction: column;
      flex: 1;
      background: $border;
      border-left: 1px solid $border;

      .row {
        border-spacing: 1px;
        flex: 1;
        display: flex;
        justify-content: flex-start;
        align-items: center;
        font-size: 12px;

        .col-key {
          width: 180px;
          height: 30px;
          padding: 0px 10px;
          margin-right: 1px;
          background: #fff;
          line-height: 30px;
        }

        .col-val {
          background: #fff;
          flex: 1;
          height: 30px;
          padding: 0px 10px;
          line-height: 30px;
        }
      }

      .row + .row {
        margin-bottom: 1px;
      }

      .row:first-child {
        margin-bottom: 1px;
      }

      .row:last-child {
        margin-bottom: 0;
      }
    }
  }

  .list-item + .list-item {
    margin-top: 15px;
  }

  .look-more {
    padding: 15px;
    text-align: center;
  }
}
</style>
