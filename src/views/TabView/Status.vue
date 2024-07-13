<script setup lang="ts">
import database, {ConnectionImpl} from "@/database.ts";
import {onMounted, ref} from "vue";
import {status} from "@/api/backend.ts";
import {useTabStore} from "@/store/tabs.ts";

const tabs = useTabStore()
const isOk = ref(true)
let connInfo = ref<ConnectionImpl | undefined>()
let errorInfo = ref("")
const emit = defineEmits(['ok'])
onMounted(() => {
  database.connection.get(tabs.activeTab?.id)
      .then((data) => {
        connInfo.value = data
        status<any>(data)
            .then((e) => {
              console.log(e)
              emit("ok")
            })
            .catch((e) => {
              errorInfo.value = e
              isOk.value = false
            })

      })
      .finally(() => {

      })
})
const close = () => {
  tabs.delTab(tabs.activeTab?.id || -3)
}
</script>

<template>
  <div class="w-420  mx-auto pt-45px">
    <div class="flex justify-between">
      <div class="flex">
        <div class="p-8 rd-3 bg-black mr-4">
          <div class="i-tabler:server-bolt w-26 h-26 text-white"></div>
        </div>
        <div class="flex flex-col justify-center">
          <span class="font-700 pb-0.5 text-18px">{{ connInfo?.name }}</span>
          <span v-if="!connInfo?.cluster" class="text-12px text-gray">
             {{ $t("layout.connection.host") }} : {{ connInfo?.node[0].host }}
            </span>
          <span v-else>{{ $t("layout.cluster") }}</span>
        </div>
      </div>
    </div>
    <div class="flex-conn-progress relative my-4 h-30px" v-if="isOk">
      <div class="absolute left-15px right-15px h-8px h-8px top-11px bg-gray overflow-hidden">
        <div class="animation_bar h-full w-100 bg-black"></div>
      </div>
      <div class="absolute w-30 h-30 left-0 top-0 absolute">
        <div class="w-22 h-22 border-black border-4 border-solid rd-15px clip relative">
        </div>
        <div class="w-20 h-20 rd-22 bg-black top-5 left-5 absolute flex justify-center items-center">

          <div class="i-tabler:server-bolt text-white w-13 h-13">

          </div>
        </div>
      </div>
      <div class="absolute w-30 h-30 right-0 top-0 absolute">
        <div class="w-22 h-22 border-black border-4 border-solid rd-15px over relative">
        </div>
        <div class="w-20 h-20 rd-22 bg-black top-5 left-5 absolute flex justify-center items-center">

          <div class="i-material-symbols:power-plug text-white w-14 h-14">

          </div>
        </div>
      </div>
    </div>
    <div class="flex-conn-progress relative my-4 h-30px" v-else>
      <div class="absolute left-15px right-15px h-8px h-8px top-11px overflow-hidden bg-red">
      </div>
      <div class="absolute w-30 h-30 left-0 top-0 absolute">
        <div class="w-30 h-30 rd-22 bg-red top-0 left-0 absolute flex justify-center items-center">
          <div class="i-tabler:server-bolt text-white w-18 h-18">
          </div>
        </div>
      </div>
      <div class="absolute w-30 h-30 right-[-4px] top-0 absolute">
        <div class="w-20 h-20 rd-22 bg-red top-5 left-5 absolute flex justify-center items-center">
          <div class="i-material-symbols:power-plug text-white w-14 h-14"></div>
        </div>
      </div>
    </div>
    <div v-if="!isOk" class="scroller bg-black rd-2 text-red line-height-[20px]">
      {{ errorInfo }}
    </div>
    <div class="pt-5 flex" v-if="!isOk">
      <a-button type="primary" @click="close()" status="danger" size="mini">{{ $t("layout.connection.close") }}</a-button>
    </div>
    <div class="flex" v-else>
      <a-button  type="dashed"  status="normal" @click="close()" size="mini">{{ $t("layout.connection.close") }}</a-button>
    </div>
  </div>
</template>

<style scoped lang="scss">
@keyframes go_line {
  from {
    transform: translateX(0);
  }
  to {
    transform: translateX(420px);
  }
}

.animation_bar {
  animation: go_line 1s infinite linear;
}

@keyframes go_round {
  from {
    transform: rotateZ(0deg);
  }
  to {
    transform: rotateZ(360deg);
  }
}

.clip {
  background: transparent;
  clip-path: view-box circle(80% at 0% 10%);
  animation: linear infinite go_round 0.268s;
}

.over {
  border-color: transparent;
}

.scroller {
  overflow: auto;
  max-height: 50vh;
  border: 8px solid #000000;

  &::-webkit-scrollbar {
    width: 0;
    height: 0;
  }
}
</style>