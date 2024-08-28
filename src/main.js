import { createApp } from "vue";
import router from "./router";
import "./style.css";
import App from "./App.vue";
import { isBodylessHandler } from "./utils";

let app = createApp(App);
app.config.errorHandler((e,vm,info)=>{
    isBodylessHandler(e)
});
app.use(router).mount("#app");
