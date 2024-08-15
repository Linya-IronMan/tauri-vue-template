import { theme } from "ant-design-vue";
import { computed, ref } from "vue";

export const isDark = ref<boolean>(true);

export function syncTheme() {
	const item = localStorage.getItem("systemTheme");
	if (item) {
		isDark.value !== (item === "dark") && (isDark.value = item === "dark");
	}
}

export function setTheme() {
	isDark.value = !isDark.value;
	localStorage.setItem("systemTheme", isDark.value ? "dark" : "light");
}

export const toolTheme = computed(() => {
	return {
		algorithm: [
			isDark.value ? theme.darkAlgorithm : theme.defaultAlgorithm,
		],
		token: {
			colorPrimary: isDark.value ? "#01c89e" : "#03a05e",
			/**
			 * 工具图标激活状态色:#01C89E
			 */
			navBtnBgColor: "#01C89E",
			/**
			 * 工具图标移入状态色:#72EDC6
			 * */
			toolHover: "#72EDC6",
		},
	};
});
