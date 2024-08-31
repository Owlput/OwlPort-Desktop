import { createApp } from "vue";
import router from "./router";
import "./style.css";
import App from "./App.vue";

createApp(App).use(router).mount("#app");

/*
    Note: Errors from tauri cannot be handled using global error handler 
    provided by vue(`app.config.errorHandler`).
*/
