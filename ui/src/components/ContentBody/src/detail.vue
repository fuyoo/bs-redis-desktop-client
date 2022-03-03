<template>
  <div class="detail">

    <div class="key-container" ref="keyContainer" :style="{width: keyContainerWidth+'px'}">
      <div class="search">
        <input @keyup="autoFilterFn" v-model="query" type="text" class="s-i" :placeholder="lang().searchPlaceHolder">
        <button class="s-btn" @click="filterKeyFn">
          <icon name="search-line"></icon>
        </button>
      </div>
      <div class="keys" v-loading="keysLoading">
        <div style="line-height: 30px;font-size: 14px;margin-left: -5px">
          <icon name="database--line"></icon>&nbsp;
          <span style="font-size: 12px">{{lang().databaseList.title}}&nbsp;{{ clientInfo.db }}</span>
        </div>
        <el-tree-v2 @node-click="chooseKeyFn" width="100%" :height="treeHeight" icon="div"
                    ref="etv2" :empty-text="lang().databaseList.emptyTreeText"
                    :props="{label:'title'}"></el-tree-v2>
      </div>
      <div class="move-bar" @mousedown="mousedownFn"></div>
    </div>
    <div class="value-container">
      <div class="action-bar">
        <el-button size="mini" type="primary" @click="newkeyFn">
          <icon name="add-line"></icon>
          {{lang().actionBar.add}}
        </el-button>
        <el-button size="mini" v-if="key" type="primary" @click="renameFn">
          <icon name="edit--line"></icon>
          {{lang().actionBar.rename}}
        </el-button>
        <el-button size="mini" v-if="key" type="primary" @click="ttlFn">
          <icon name="time-line"></icon>
          {{lang().actionBar.ttl}}
        </el-button>
        <el-button size="mini" v-if="key" type="primary" @click="delFn">
          <icon name="delete-bin--line2"></icon>
          {{lang().actionBar.del}}
        </el-button>
        <el-button size="mini" type="primary" v-if="key" @click="refreshFn">
          <icon name="refresh-line"></icon>
          {{lang().actionBar.refresh}}
        </el-button>
      </div>
      <el-scrollbar class="info-container" v-loading="keyInfoLoading">
        <div class="wallpaper" v-if="key">
          <div class="key-base-info">
            <div class="kbi-title">{{lang().tableInfo.title}}</div>
            <div class="kbi-table">
              <div class="kbi-tr">
                <div class="kbi-td-1">{{lang().tableInfo.tHeader[0]}}</div>
                <div class="kbi-td-2">{{lang().tableInfo.tHeader[1]}}</div>
                <div class="kbi-td-3">{{lang().tableInfo.tHeader[2]}}</div>
                <div class="kbi-td-4">{{lang().tableInfo.tHeader[3]}}</div>
              </div>
              <div class="kbi-tr">
                <div class="kbi-td-1">{{ key }}</div>
                <div class="kbi-td-2">{{ keyInfo.t }}</div>
                <div class="kbi-td-3">{{ keyInfo.size }}</div>
                <div class="kbi-td-4">{{ keyInfo.ttl }}</div>
              </div>
            </div>
          </div>
          <component ref="dataCMP" :is="keyInfo.t" :k="key"></component>
        </div>
        <div class="none" v-else>
          <el-empty :description="lang().notChooseKey"></el-empty>
        </div>
      </el-scrollbar>
    </div>
    <newkey ref="newkey"></newkey>
    <rename ref="rename"></rename>
    <ttl ref="ttl"></ttl>
  </div>
</template>

<script>
import {invoke} from '@tauri-apps/api/tauri'
import {mapGetters} from 'vuex'
import newkey from ':/components/ContentBody/src/newkey';
import rename from ':/components/ContentBody/src/rename';
import ttl from ':/components/ContentBody/src/ttl';
import hash from ':/components/ContentBody/src/data-type/hash'
import list from ':/components/ContentBody/src/data-type/list'
import set from ':/components/ContentBody/src/data-type/set'
import string from ':/components/ContentBody/src/data-type/string'
import zset from ':/components/ContentBody/src/data-type/zset'
import {listen} from '@tauri-apps/api/event';
import {request} from ':/tools/invoke';

let timer = -1
let refreshTimer = -1
export default {
  name: 'DetailInfo',
  components: {
    hash,
    list,
    set,
    string,
    zset,
    newkey,
    rename,
    ttl
  },
  data() {
    return {
      keysLoading: false,
      keyInfoLoading: false,
      autoRefresh: false,
      autoRefreshTime: 3,
      keys: [],
      bodyHeight: 0,
      query: '',
      keyContainerWidth: 205,
      resizeWidth: {
        start: 0,
        move: false,
        width: 205
      },
      key: '',
      keyInfo: {}
    }
  },
  computed: {
    treeHeight() {
      return this.bodyHeight - 95
    },
    ...mapGetters(['clientInfo','i18n'])
  },
  watch: {
    clientInfo() {
      this.query = ''
      this.key = ''
      this.filterKeyFn()
    },
    key(r) {
      if (r) {
        this.fetchKeyInfo(r)
      }
    }
  },
  created() {
    listen('sys_info_update', () => {
      this.getSysInfo()
    });
    window.onresize = () => {
      clearTimeout(timer)
      timer = setTimeout(() => {
        this.getGetBodyHeight()
      }, 50)
    }
    this.getGetBodyHeight()
    this.initResizeWidthEvt()
    this.getSysInfo()
  },
  mounted() {
    this.fetchKeysData('*')
  },
  methods: {
    lang(){
      return this.i18n.mainPage.content.dataPage
    },
    /**
     *
     */
    getSysInfo() {
      request('query_sys_info')
          .then(res => {
            const {autoRefresh, autoRefreshTime} = res.data
            this.autoRefresh = autoRefresh
            this.autoRefreshTime = autoRefreshTime
            if (autoRefresh) {
              clearTimeout(refreshTimer)
              this.autoRefreshKeyList()
            }
          })
    },
    /**
     * 自动刷新key列表
     */
    autoRefreshKeyList() {
      if (!this.autoRefresh) {
        return
      }
      refreshTimer = setTimeout(() => {
        this.fetchKeysData(this.query || '*')
            .finally(() => {
              this.autoRefreshKeyList()
            })
      }, this.autoRefreshTime * 1000)
    },
    /**
     * 获取body高度
     */
    getGetBodyHeight() {
      this.bodyHeight = document.body.clientHeight
    },
    /**
     * 根据key生成对应的命名空间数据
     * @param arr {array} keys列表
     */
    formatNamespace(arr) {
      let tmp = []
      let dg = (data, arrs, level, key, id) => {
        // 获取name
        let title = arrs.splice(0, 1)[0]
        // 获取是否存在root
        let root = data.find(e => e.title === title)
        // 如果arrs的长度为0，则到了最后一级
        if (arrs.length === 0) {
          //直接向data中push数据
          data.push(
              {
                id,
                title,
                type: 'key',
                key,
                level
              }
          )
          return
        }
        level += 1
        // root存在
        if (root) {
          // 如果没有children创建children
          if (!root.children) root.children = []
          // 然后加入children
          return dg(root.children, arrs, level, key, level + id)
        }
        // root 不存在代表是同级的，直接向data里面push数据
        root = {
          id,
          title,
          type: 'space',
          children: [],
          level
        }
        data.push(root)
        // 继续往children递归
        dg(root.children, arrs, level, key, level + id)
      }
      arr.forEach(e => {
        // 拆分命名空间
        let split = e.split(':')
        // 获取root的title名称
        let title = split[0]
        // 找一下tmp数组中是否已经存在root元素
        let root = tmp.find(e => e.title === title)
        // 如果不存在就插入一个根元素
        if (!root) {
          tmp.push({
            id: 'root-' + e,
            title,
            type: 'space',
            level: 0,
            children: []
          })
        }
        // 递归生成数据
        dg(tmp, split, 0, e, '0' + e)
      })
      return tmp
    },
    /**
     * 获取key数据
     */
    fetchKeysData(query) {
      return new Promise(resolve => {
        this.keysLoading = true
        invoke('get_redis_keys', {query})
            .then(res => {
              let space = []
              let keys = []
              res.data.forEach(e => {
                if (e.indexOf(':') > -1) {
                  space.push(e)
                } else {
                  keys.push({
                    id: e,
                    title: e,
                    type: 'key',
                    key: e,
                    level: 0
                  })
                }
              })
              let data = this.formatNamespace(space).concat(keys)
              if (data) {
                this.$refs.etv2.setData(data)
              }
            })
            .finally(() => {
              this.keysLoading = false
              resolve()
            })
      })
    },
    /**
     * 自动查询
     */
    autoFilterFn() {
      clearTimeout(timer)
      timer = setTimeout(() => {
        this.filterKeyFn()
      }, 200)
    },
    /**
     * 查询按钮点击
     */
    filterKeyFn() {
      if (!this.query) {
        return this.fetchKeysData('*')
      }
      this.fetchKeysData(this.query)
    },
    /**
     * 初始化key列表拖动调整宽度功能
     */
    initResizeWidthEvt() {
      document.addEventListener('mousemove', evt => {
        if (this.resizeWidth.move === true) {
          let x = evt.clientX - this.resizeWidth.start
          let width = this.resizeWidth.width + x
          if (width < 205) {
            width = 205
          }
          if (width > 600) {
            width = 600
          }
          this.keyContainerWidth = width
        }
      })
      document.addEventListener('mouseup', () => {
        this.resizeWidth.move = false
        document.body.style.cursor = 'default'
      })
    },
    /**
     * key列表拖动控制器鼠标按下事键
     * @param evt
     */
    mousedownFn(evt) {
      this.resizeWidth = {
        move: true,
        start: evt.clientX,
        width: this.$refs.keyContainer.clientWidth
      }
      document.body.style.cursor = 'col-resize'
    },
    /**
     * 选择key事件
     * @param data
     * @param node
     */
    chooseKeyFn(data, node) {
      if (node.isLeaf) {
        this.key = ''
        this.key = data.key
      }
    },
    /**
     * 获取key的信息
     * @param key
     */
    fetchKeyInfo(key) {
      if (!key) return
      this.keyInfoLoading = true
      invoke('get_redis_key_info', {key})
          .then(res => {
            if (res.code === 200 && res.data.status === true) {
              this.keyInfo = res.data
              this.$nextTick(() => {
                this.$refs.dataCMP.refresh()
              })
            } else {
              this.keyInfo = {}
              this.key = ''
            }
          })
          .finally(() => this.keyInfoLoading = false)
    },
    newkeyFn() {
      this.$refs.newkey.show(this.key)
    },
    renameFn() {
      this.$refs.rename.show(this.key)
    },
    ttlFn() {
      this.$refs.ttl.show(this.key)
    },
    delFn() {
      this.$confirm('确定要删除吗？')
          .then(() => {
            invoke('del_key', {key: this.key})
                .then(res => {
                  if (res.code === 200) {
                    this.key = ''
                    this.autoFilterFn()
                    this.alert.success(res.msg)
                  } else {
                    this.alert.error(res.msg)
                  }
                })
          })
    },
    refreshFn() {
      this.filterKeyFn()
      this.fetchKeyInfo(this.key)
      try {
        this.$refs.dataCMP.refresh()
      } catch (e) {

      }

    }
  }
}
</script>

<style scoped lang="scss">
.detail {
  height: calc(100vh - 30px);
  display: flex;
  justify-content: flex-start;
  align-items: flex-start;

  .key-container {
    width: 205px;
    flex-shrink: 0;
    position: relative;

    user-select: none;
    -webkit-user-select: none;

    .search {
      display: flex;
      flex: 1;
      justify-content: flex-start;
      align-items: center;
      border-bottom: 1px solid $border;

      .s-i {
        border: none;
        outline: none;
        width: calc(100% - 70px);
        padding: 0 15px;
        line-height: 16px;
        border-right: 1px solid $border;
        height: 16px;
        font-size: 12px;
      }

      .s-btn {
        width: 40px;
        height: 30px;
        display: flex;
        justify-content: center;
        align-items: center;
        flex-shrink: 0;
        cursor: pointer;
        border: none;
        outline: none;
        font-size: 16px;
        background: #fff;

        &:active {
          color: gray;
        }
      }
    }

    .keys {
      overflow: hidden;
      height: calc(100vh - 62px);
      padding-left: 15px;

      ::v-deep(.el-tree-node__expand-icon) div {
        position: relative;
        display: inline-block;
        width: 100%;
        height: 100%;
        background: url("~:/asserts/icon/folder-close.svg") center center / 100% no-repeat;
      }

      ::v-deep(.el-tree-node__expand-icon) div::before {
        content: '';
        display: block;
        position: absolute;
        left: -6px;
        top: 7px;
        height: 0;
        width: 6px;
        border-bottom: 1px solid #666;
      }

      ::v-deep(.el-tree-node__expand-icon) div::after {
        content: '';
        display: block;
        position: absolute;
        left: -7px;
        top: -7px;
        height: 26px;
        width: 0;
        border-left: 1px solid #666;
      }

      ::v-deep(.el-tree-node__expand-icon.expanded) {
        transform: rotate(0);
        background: url("~:/asserts/icon/folder-open.svg") center center / 100% no-repeat;
      }

      ::v-deep(.el-tree-node__expand-icon.is-leaf) div {
        display: inline-block;
        width: 100%;
        height: 100%;
        background: url("~:/asserts/icon/key.svg") center center / 100% no-repeat;
      }

      ::v-deep(.el-tree-node) {
        border-left: 1px solid #666;
      }

      ::v-deep(.is-current) .el-tree-node__expand-icon.is-leaf + .el-tree-node__label {
        color: $primary-color;
      }

      ::v-deep(.is-current) .el-tree-node__expand-icon.is-leaf div {
        background: url("~:/asserts/icon/key-focus.svg") center center / 100% no-repeat;
      }
    }

    .move-bar {
      position: absolute;
      right: 0;
      top: 0;
      height: 100vh;
      width: 1px;
      background: $border;
      cursor: col-resize;
      z-index: 10;

      &:hover {
        width: 4px;
        right: -2px;
      }
    }
  }

  .value-container {
    flex: 1;
    height: 100%;

    .action-bar {
      height: 30px;
      background: #fff;
      border-bottom: 1px solid $border;
      display: flex;
      justify-content: flex-start;
      align-items: center;
      padding-left: 15px;

      ::v-deep(.el-button--mini) {
        font-size: 10px;
        min-height: 22px;
        padding: 0 12px;
      }
    }

    .info-container {
      height: calc(100vh - 62px);
      width: 100%;

      .wallpaper {
        padding: 10px;

        .key-base-info {
          -webkit-user-select: none;
          user-select: none;

          .kbi-title {
            font-size: 14px;
            height: 30px;
            padding: 0 10px;
            line-height: 30px;
            background: linear-gradient(to right, $border, #ffff);
          }

          .kbi-table {
            border: 1px solid $border;
            display: flex;
            flex-direction: column;

            .kbi-tr {
              display: flex;
              justify-content: space-between;
              align-items: center;

              .kbi-td-1 {
                flex: 1;
                height: 30px;
                line-height: 30px;
                padding: 0 10px;
                font-size: 12px;
                user-select: auto;
                -webkit-user-select: auto;
              }

              .kbi-td-2,
              .kbi-td-3,
              .kbi-td-4 {
                width: 80px;
                border-left: 1px solid $border;
                height: 30px;
                line-height: 30px;
                padding: 0 10px;
                font-size: 12px;
                text-align: center;
              }
            }

            .kbi-tr + .kbi-tr {
              border-top: 1px solid $border;
            }
          }
        }
      }

      .none {
        width: 100%;
        height: calc(100vh - 62px);
        display: flex;
        justify-content: center;
        align-items: center;
      }
    }
  }
}

</style>
