<template>
  <div class="database-menu" :style="isShow">
    <div style="width: 199px; height:100%">
      <div class="new-connection">
        <el-button @click="newConnectionFn" style="width: 100%" type="primary" size="mini">
          <icon name="add-line"></icon>
          {{ i18n.mainPage.menu.button }}
        </el-button>
      </div>
      <div class="conn-list-container">
        <el-scrollbar height="100%">
          <el-collapse v-model="activeNames" accordion>
            <el-collapse-item :name="item.id" v-for="item in connections">
              <template #title>
                <div class="col-title">{{ item.name }} <span class="flag-on" v-if="info.id === item.id">on</span></div>
              </template>
              <div class="c-content">
                <div class="action">
                  <el-button type="primary" size="mini" @click="editConnectionFn(item)">
                    <icon name="edit-fill"></icon>
                    {{ i18n.mainPage.menu.listButton.edit }}
                  </el-button>
                  <el-button type="primary" size="mini" @click="deleteConnectionFn(item.id)">
                    <icon name="delete-bin--fill"></icon>
                    {{ i18n.mainPage.menu.listButton.delete }}
                  </el-button>
                  <el-button :loading="connectingLoading" type="primary" v-if="info.id === item.id "
                             @click="toggleConnectFn(item)" size="mini">
                    <icon name="link-unlink-m"></icon>
                    {{ i18n.mainPage.menu.listButton.disconnect }}
                  </el-button>
                  <el-button :loading="connectingLoading" type="primary" v-else size="mini"
                             @click="toggleConnectFn(item)">
                    <icon name="link-m"></icon>
                    {{ i18n.mainPage.menu.listButton.connect }}
                  </el-button>
                </div>
                <div class="conn-list" v-if="info.databases && info.id === item.id">
                  <div class="conn-list-item" :class="{active: info.id === item.id && info.db === index-1+''}"
                       v-for="index in Number(info.databases)" @click="chooseDatabaseFn(index - 1 +'')">
                  <span>
                    <icon name="database--fill"></icon> {{ i18n.mainPage.menu.listItemName }} {{ index - 1 }}
                  </span>
                    <span class="using"
                          v-if="info.id === item.id && info.db === index-1+''">{{ i18n.mainPage.menu.status }}</span>
                  </div>
                </div>
              </div>
            </el-collapse-item>
          </el-collapse>
        </el-scrollbar>
      </div>
    </div>
    <e-dialog ref="dialog"></e-dialog>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri'
import Dialog from './dialog'
import {computed} from 'vue'
import {useStore} from 'vuex'
import {mapGetters} from "vuex";

export default {
  name: 'DatabaseMenu',
  components: {EDialog: Dialog},
  /**
   * 是否折叠连接栏
   */
  props: {
    show: {
      type: Boolean,
      default() {
        return true
      }
    }
  },
  data() {
    return {
      /**
       * 默认打开菜单
       */
      activeNames: '-1',
      /**
       * 连接信息
       */
      info: {
        id: '',
        db: '0'
      },
      connectingLoading: false
    }
  },
  watch: {
    info(r) {
      let clientInfo = Object.assign({}, r)
      if (clientInfo.id != -1) {
        clientInfo.status = true
      } else {
        clientInfo.status = false
      }
      this.$store.dispatch('setClientInfo', clientInfo)
    }
  },
  computed: {
    ...mapGetters(['i18n']),
    /**
     * 是否显示菜单
     * @returns {string}
     */
    isShow() {
      if (this.menuShow) {
        return `width:200px`
      }
      return `width:0px`
    }
  },
  mounted() {
    // 获取连接
    this.$store.dispatch('getConnectionList')
  },
  methods: {
    /**
     * 切换连接状态
     */
    toggleConnectFn(item) {
      if (item.id === this.info.id) {
        this.info = {id: '-1', db: '0'}
      } else {
        this.connectRedis(item)
      }
    },
    /**
     * 选择数据库
     */
    chooseDatabaseFn(db) {
      let obj = Object.assign({}, this.info)
      obj.db = db

      invoke('set_redis_config', obj)
          .then(async res => {
            if (res.code === 200) {
              this.info.db = db
              this.$store.dispatch('setClientInfo', Object.assign({status: true}, this.info))
            }
          })
    },
    /**
     * 创建一个新连接
     */
    newConnectionFn() {
      this.$refs.dialog.show()
    },
    /**
     * 编辑连接的方法
     * @param row
     */
    editConnectionFn(row) {
      this.$refs.dialog.show('edit', Object.assign({}, row))
    },
    /**
     * 删除连接
     * @param id
     */
    deleteConnectionFn(id) {
      this.$confirm('确定要删除吗？')
          .then(() => {
            invoke('connection_delete', {id})
                .then(res => {
                  if (res.code === 200) {
                    this.alert.success(res.msg)
                    this.$store.dispatch('getConnectionList')
                  } else {
                    this.alert.error(res.msg)
                  }
                })
          })
          .catch(() => {
          })
    },
    /**
     * 连接到redis
     */
    connectRedis(config) {
      if (!config.db) {
        config.db = '0'
      }
      this.connectingLoading = true
      // 设置配置信息
      invoke('set_redis_config', config)
          .then(async res => {
            if (res.code === 200) {
              return await invoke('get_redis_db')
            } else {
              return Promise.reject(res.msg)
            }
          })
          .then(res => {
            if (res.code === 200) {
              this.info = config
              this.info.databases = res.data
            } else {
              return Promise.reject(res.msg)
            }
          })
          .catch(e => {
            this.info = {id: '-1', db: '0'}
            this.alert.error(e.toString())
          })
          .finally(() => this.connectingLoading = false)
    }
  },
  setup() {
    const store = useStore()
    return {
      connections: computed(() => store.getters.connectionList),
      menuShow: computed(() => store.getters.menuShow)
    }
  }
}
</script>

<style scoped lang="scss">
.database-menu {
  width: 200px;
  border-right: 1px solid $border;
  height: 100vh;
  transition: width 0.168s ease-in-out;
  overflow: hidden;
  user-select: none;
  -webkit-user-select: none;
  flex-shrink: 0;

  .new-connection {
    display: flex;
    justify-content: center;
    align-items: center;
    padding: 15px 10px;
    border-bottom: 1px solid $border;
    position: relative;
    z-index: 2;
    background: #fff;
  }

  ::v-deep(.el-collapse-item__header) {
    height: 30px;
    padding-left: 10px;
  }

  ::v-deep(.el-collapse-item__content) {
    padding-bottom: 0;
  }

  .conn-list-container {
    height: calc(100% - 60px);
    position: relative;
    z-index: 1;
    margin-top: -2px;
    padding-bottom: 2px;
  }

  .col-title {
    width: 100%;
    height: 100%;
    line-height: 1;
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .flag-on {
    display: flex;
    justify-content: center;
    align-items: center;
    font-size: 10px;
    padding: 0px 4px;
    border: 1px solid $primary-color;
    color: $primary-color;
    border-radius: 3px;
    line-height: 1;
    margin-right: 4px;
  }

  .c-content {
    border-top: 1px solid $border;

    .action {
      display: flex;
      justify-content: space-between;
      align-items: center;
      padding: 10px;


      font-size: 10px;

      .el-button + .el-button {
        margin-left: 0;
      }

      ::v-deep(.el-button--mini) {
        font-size: 10px;
        min-height: 22px;
        padding: 0px 8px;
      }
    }

    .conn-list {
      border-top: 1px solid $border;
      padding: 10px 0;

      .conn-list-item {
        padding: 0 10px;
        line-height: 30px;
        transition: .168s;
        cursor: pointer;
        user-select: none;
        display: flex;
        justify-content: space-between;
        align-items: center;
        font-size: 12px;

        &:hover {
          color: $primary-color;
          background: $background-color;
        }

        .using {
          display: flex;
          justify-content: center;
          align-items: center;
          font-size: 10px;
          line-height: 1;
          border-radius: 3px;
          color: $primary-color;
        }
      }

      .active {
        background: $background-color;
        color: $primary-color;
      }
    }
  }

  .item {

  }
}
</style>
