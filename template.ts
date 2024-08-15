import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";
export const useRust = () => {
	const count = ref(0);
	async function backendAdd() {
		count.value = await invoke("backend_add", { number: count.value });
	}
	return {
		count,
		backendAdd,
	};
};
