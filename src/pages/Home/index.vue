<script setup lang="ts">
import { css } from '@emotion/css';
import SideMenu from './components/SideMenu/index.vue'
import { useEmotion, useToken } from '@/utils/hooks/useEmotion';
const { token } = useToken()
const headerStyle = css({
    // textAlign: 'center',
    color: token.colorTextBase,
    // height: 64,
    // paddingInline: 50,
    // lineHeight: '64px',
});

const contentStyle = css({
    // textAlign: 'center',
    // minHeight: 120,
    // lineHeight: '120px',
    color: '#fff',
});

const siderStyle = useEmotion(() => ({
    color: token.colorTextBase,
    backgroundColor: token.colorBgContainer,

}))

const footerStyle = css({
    // textAlign: 'center',
    color: '#fff',
});

const siderWidth = "25%"
const contentWidth = "75%"

</script>

<template>
    <a-layout>
        <a-layout-sider :class="siderStyle" :width="siderWidth">
            <side-menu />
        </a-layout-sider>
        <a-layout :width="contentWidth">
            <a-layout-header :class="headerStyle">Header</a-layout-header>
            <a-layout-content :class="contentStyle">Content</a-layout-content>
            <a-layout-footer :class="footerStyle">Footer</a-layout-footer>
        </a-layout>
    </a-layout>
</template>

<style scoped></style>