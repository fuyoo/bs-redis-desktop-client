<template>
  <div class="detail">
    <el-scrollbar height="100%">
      <pre>
        <json-viewer :json="redisConfig"></json-viewer>
      </pre>
    </el-scrollbar>
  </div>
</template>

<script>
import {JsonViewer} from ":/components/JsonViewer";
import { invoke } from '@tauri-apps/api/tauri'

export default {
  name: 'detail',
  components:{
    JsonViewer
  },
  data() {
    return {
      redisConfig: {}
    }
  },
  created() {
    invoke('get_redis_info')
        .then(res => {
          if (res.code === 200) {
          }
          let cfg = res.data
          if (!cfg) return
          let tmp = {}
          cfg.split('#').map(e => e.split('\r\n').filter(v => v !== ''))
              .forEach(e => {
                if (e[ 0 ] == undefined) {
                  return
                }
                try {
                  let key = e[ 0 ].replace(/\s/, '')
                  tmp[ key ] = {}
                  e.slice(1).forEach(item => {
                    let data = item.split(':')
                    if (data[ 1 ].indexOf(',') > -1) {
                      let child = data[ 1 ].split(',')
                      let map = {}
                      child.map(val => {
                        let s = val.split('=')
                        map[ s[ 0 ] ] = s[ 1 ]
                      })
                      tmp[ key ][ data[ 0 ] ] = map
                      return
                    }
                    if (data[ 1 ].indexOf('=') > -1) {
                      let v = data[ 1 ].split('=')
                      tmp[ key ][ data[ 0 ] ] = { [ v[ 0 ] ]: v[ 1 ] }
                      return
                    }
                    tmp[ key ][ data[ 0 ] ] = data[ 1 ]
                  })
                } catch (e) {
                  console.error(e.message)
                }
              })
          this.redisConfig = tmp
        })
  },
}
</script>

<style lang="scss">
.detail {
  position: absolute;
  left: 0;
  top: 0;
  right: 0;
  bottom: 0;

  pre {
    padding: 5px;
    margin: 5px;
    line-height: 1.5;
    font-size: 18px;
  }

  .string {
    color: green;
  }

  .number {
    color: darkorange;
  }

  .boolean {
    color: blue;
  }

  .null {
    color: magenta;
  }

  .key {
    color: darkred;
    font-weight: bold;
  }
}
</style>
