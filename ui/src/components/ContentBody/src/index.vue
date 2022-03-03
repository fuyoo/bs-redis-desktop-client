<template>
  <div class="content-body" v-if="clientInfo.status">
    <div class="tab">
      <button class="tab-item btn" v-if="menuShow" @click="toggleMenuShow">
        <icon name="layout-column-fill"></icon>
      </button>
      <div class="tab-item btn" v-else @click="toggleMenuShow">
        <icon name="layout-column-line"></icon>
      </div>
      <button class="tab-item" :class="{'tab-active':tab === 'BaseInfo'}" @click="tab = 'BaseInfo'">
        {{ i18n.mainPage.content.tabBar.info }}
      </button>
      <button class="tab-item" :class="{'tab-active':tab === 'DetailInfo'}" @click="tab = 'DetailInfo'">
        {{ i18n.mainPage.content.tabBar.data }}
      </button>

      <button v-if="openPubSub" class="tab-item" :class="{'tab-active':tab === 'PubSub'}" @click="tab = 'PubSub'">
        Pub/Sub
      </button>
    </div>
    <component :is="tab"></component>
  </div>
  <div class="not-connect" v-else>
    <el-empty :description="i18n.mainPage.content.dataPage.notConnectText"></el-empty>
  </div>
</template>

<script>
import BaseInfo from './base'
import DetailInfo from './detail'
import {PubSub} from ':/components/PubSub';
import {useStore} from 'vuex'
import {computed} from 'vue'
import {listen} from '@tauri-apps/api/event';
import {request} from ':/tools/invoke';

export default {
  name: 'ContentBody',
  components: {
    BaseInfo,
    DetailInfo,
    PubSub
  },
  data() {
    return {
      tab: 'BaseInfo',
      openPubSub: false
    }
  },
  created() {
    listen('sys_info_update', () => {
      this.getSysInfo()
    })
    this.getSysInfo()
  },
  methods: {
    getSysInfo() {
      request('query_sys_info')
          .then(res => {
            const {pubsub} = res.data
            this.openPubSub = pubsub
            if (pubsub === false) {
              if (this.tab === 'PubSub') {
                this.tab = 'BaseInfo'
              }
              request('sub_stop_all')
            }
          })
    },
    toggleMenuShow() {
      this.$store.dispatch('toggleMenuShow')
    }
  },
  setup() {
    let store = useStore()
    return {
      menuShow: computed(() => store.state.menuShow),
      clientInfo: computed(() => store.state.clientInfo),
      i18n: computed(() => store.state.i18n)
    }
  }
}
</script>

<style scoped lang="scss">
.content-body {
  width: 100%;
  height: calc(100% - 30px);

  .tab {
    height: 30px;
    display: flex;
    justify-content: flex-start;
    align-items: center;
    background: #fff;
    border-bottom: 1px solid $border;
    user-select: none;

    .tab-item {
      padding: 0 30px;
      transition: .168s;
      font-size: 12px;
      user-select: none;
      display: flex;
      justify-content: center;
      align-items: center;
      height: 29px;
      position: relative;
      border: none;
      margin: 0;
      background: #fff;
    }

    .tab-item:hover {
      background: $background-color;
      cursor: pointer;
      color: $primary-color;
    }

    .tab-item:active {
      opacity: 0.6;
    }

    .tab-active {
      background-color: $background-color;
      color: $primary-color;
    }

    .tab-active::after {
      content: "";
      width: 100%;
      position: absolute;
      left: 0;
      bottom: -1px;
      border-bottom: 1px solid $primary-color;
    }

    .btn {
      font-size: 20px;
      padding: 0 8px;
    }

    .btn:hover {
      background: #fff;
    }
  }
}

.not-connect {
  width: 100%;
  height: 100%;
  display: flex;
  justify-content: center;
  align-items: center;
}
</style>
