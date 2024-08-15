<script setup lang='ts'>
import { useEmotion } from "@/utils/hooks/useEmotion";
import { css } from "@emotion/css";
import { invoke } from "@tauri-apps/api/tauri";
const data = ref<any[]>([])
const count = ref(0);

async function backendAdd() {
    count.value = await invoke("backend_add", { number: count.value });
}
const open_file_dialog = async () => {
    const value: any[] = await invoke("open_file_dialog", { number: 1 });
    data.value.push(...value.map(item => ({ size: item.size, file: item.name })))
}

data.value.push({ size: 123123, file: "123123" })

const fileItemClass = useEmotion((token) => ({
    color: token.colorTextBase
}))


</script>

<template>
    <a-flex justify="center">所有录音</a-flex>
    <a-button @click="open_file_dialog">click</a-button>
    <a-list item-layout="horizontal" :data-source="data">
        <template #renderItem="{ item }">
            <a-list-item :class="fileItemClass">
                {{ item.size }} - {{ item.file }}
            </a-list-item>
        </template>
    </a-list>
</template>

<style scoped></style>
