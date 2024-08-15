import { css } from "@emotion/css";
import { theme } from "ant-design-vue";
import { GlobalToken } from "ant-design-vue/es/theme/interface";

export const useEmotion = (fn: (theme: GlobalToken) => {}) => {
	const { useToken } = theme;
	const { token } = useToken();
	return css(fn(token.value));
};

export const useToken = () => {
	const { useToken } = theme;
	const { token } = useToken();
	return { token: token.value };
};
