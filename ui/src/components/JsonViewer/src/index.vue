<template>
  <div :class="['json-viewer', length ? 'closeable' : '']"
       :style="'font-size:' + fontSize+'px'+`;padding-left:${Number(fontSize)+4}px`">
    <span @click="toggleClose" :class="['angle', innerclosed ? 'closed' : '']" :style="arrowStyle" v-if="length"></span>
    <div class="content-wrap">
      <p class="first-line">
        <span v-if="jsonKey" class="json-key">"{{ jsonKey }}": </span>
        <span v-if="length">
          {{ prefix }}
          {{ innerclosed ? ('...' + subfix) : '' }}
          <span class="json-note">
           {{ innerclosed ? (' // count: ' + length) : '' }}
          </span>
        </span>
        <span v-if="!length">{{ isArray ? '[]' : '{}' }}</span>
      </p>
      <div v-if="!innerclosed && length" class="json-body">
        <template v-for="(item, index) in items">
          <json-viewer :closed="closed" v-if="item.isJSON"
                       :key="index" :json="item.value" :jsonKey="item.key"
                       :isLast="index === items.length - 1"></json-viewer>
          <p class="json-item" v-else>
            <span class="json-key">
                {{ (isArray ? '' : '"' + item.key + '"') }}
            </span>
            :
            <span :class="highlight(item.value)">
                {{
                item.value + (index ===
                items.length - 1 ? '' : ',')
              }}
            </span>
          </p>
        </template>
        <span v-show="!innerclosed" class="body-line"></span>
      </div>
      <p v-if="!innerclosed && length" class="last-line">
        <span>{{ subfix }}</span>
      </p>
    </div>
  </div>
</template>

<script>
export default {
  name: 'JsonViewer',
  props: {
    json: [Object, Array],
    jsonKey: {
      type: String,
      default: ''
    },
    closed: {
      type: Boolean,
      default: false
    },
    isLast: {
      type: Boolean,
      default: true
    },
    fontSize: {
      type: Number,
      default: 14
    }
  },
  created() {
    this.innerclosed = this.closed
    this.$watch('closed', () => {
      this.innerclosed = this.closed
    })
  },
  data() {
    return {
      innerclosed: true
    }
  },
  methods: {
    highlight(value) {
      let cls = 'number';
      if (/^"/.test(value)) {
        if (/:$/.test(value)) {
          cls = 'key';
        } else {
          cls = 'string';
        }
      } else if (/true|false/.test(value)) {
        cls = 'boolean';
      } else if (/null/.test(value)) {
        cls = 'null';
      }
      return cls;
    },
    isObjectOrArray(source) {
      const type = Object.prototype.toString.call(source)
      const res = type === '[object Array]' || type === '[object Object]'
      return res
    },
    toggleClose() {
      if (this.innerclosed) {
        this.innerclosed = false
      } else {
        this.innerclosed = true
      }
    }
  },
  computed: {
    arrowStyle(){
      return `width: ${Number(this.fontSize)+4}px;height:${Number(this.fontSize)+4}px`
    },
    isArray() {
      return Object.prototype.toString.call(this.json) === '[object Array]'
    },
    length() {
      return this.isArray ? this.json.length : Object.keys(this.json).length
    },
    subfix() {
      return (this.isArray ? ']' : '}') + (this.isLast ? '' : ',')
    },
    prefix() {
      return this.isArray ? '[' : '{'
    },
    items() {
      if (this.isArray) {
        return this.json.map(item => {
          const isJSON = this.isObjectOrArray(item)
          return {
            value: isJSON ? item : JSON.stringify(item),
            isJSON,
            key: ''
          }
        })
      }
      const json = this.json
      return Object.keys(json).map(key => {
        const item = json[key]
        const isJSON = this.isObjectOrArray(item)
        return {
          value: isJSON ? item : JSON.stringify(item),
          isJSON,
          key
        }
      })
    }
  },

}
</script>

<style lang="scss">

.json-viewer {
  position: relative;
  display: block;
  width: 100%;
  height: 100%;
  white-space: nowrap;
  padding-left: 30px;
  box-sizing: border-box;

  .json-note {
    color: #909399;
  }

  .json-key {
    color: rgb(147, 98, 15);
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
    color: red;
  }

  .json-item {
    margin: 0;
    padding-left: 30px;

  }

  .first-line {
    padding: 0;
    margin: 0;
  }

  .json-body {
    position: relative;
    padding: 0;
    margin: 0;

    .body-line {
      position: absolute;
      height: 100%;
      width: 0;
      border-left: dashed 1px #bbb;
      top: 0;
      left: 2px;
    }
  }

  .last-line {
    padding: 0;
    margin: 0;
  }

  .angle {
    position: absolute;
    display: flex;
    cursor: pointer;
    float: left;
    width: 30px;
    height: 30px;
    left: 0;
    justify-content: center;
    align-items: center;
    transition: 0.168s;
    transform-origin: center center;
    background: url("~:/asserts/icon/arrow-down.svg") center center / 100% no-repeat;

    &.closed {
      transform: rotate(-90deg);
    }
  }
}
</style>