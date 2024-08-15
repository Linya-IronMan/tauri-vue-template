import router from "./router";
import { devtools } from "@vue/devtools";
import { createApp } from "vue";
import App from "./App.vue";
import "vfonts/RobotoSlab.css";
import "./assets/main.postcss";
import { Button, List, message, Layout, Flex } from "ant-design-vue";

if (process.env.NODE_ENV === "development") {
	devtools.connect("http://localhost", 8098);
}
const app = createApp(App).use(router);
app.use(Button);
app.use(Layout);
app.use(Flex);
app.use(List);

app.mount("#app");
app.config.globalProperties.$message = message;
