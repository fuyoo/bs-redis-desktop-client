<script setup lang="ts">
import useTabStore from '../store/tabs'

const tabStore = useTabStore()
const jumpTo = () => {
    const id = Math.ceil(Math.random() * 10e9)
    tabStore.addTab({
        id: id,
        path: `/tab/${id}`,
        name: "" + id
    })
}
</script>

<template>
    <div>
        <div class="flex bg-white h-50px sticky left-0 top-0 _box_shadow justify-between items-center"
             style="z-index: 2">
            <b class="ml-15px text-18px">
                <div class="i-tabler:server text-24px"></div>
                {{ $t("layout.host" as any) }}</b>

            <a-button type="primary" class="mr-15px" size="mini">
                <template #icon>
                    <div class="i-ic:round-plus"></div>
                </template>
                {{ $t("layout.add" as any) }}
            </a-button>

        </div>
        <div class="flex flex-wrap _main p-25px">
            <div @dblclick="jumpTo()"
                 class="select-none cursor-pointer relative flex p-10 w-280px rounded-lg items-center bg-white _host_items"
                 v-for="i in 31" :key="i">
                <div class="i-tabler:server-bolt text-40px"></div>
                <div class="flex ml-10px flex-col text-overflow justify-between">
                    <div class="font-bold w-230px text-overflow">10.35.11.21511111111111asdfasdfasdfasdfasdfasdf1111
                    </div>
                    <div class="text-gray w-230px text-overflow">password,cluster</div>
                </div>
                <div class="rounded-md w-20px h-20px absolute right-10px top-20px _edit bg-amber justify-center items-center hidden">
                    <div class="i-ic-round-edit"></div>
                </div>
            </div>
        </div>
    </div>
</template>

<style scoped lang="scss">
._main {
  gap: 15px;
}

._box_shadow {
  box-shadow: 1px 0 1px #0005;
}

._host_items {
  box-shadow: 1px 2px 4px #0001;

  &:hover > ._edit {
    display: flex;
  }
}
</style>