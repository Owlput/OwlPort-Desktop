import { createApp } from "vue";
import router from "./router";
import "./style.css";
import App from "./App.vue";
import VueVirtualScroller from 'vue-virtual-scroller'
import 'vue-virtual-scroller/dist/vue-virtual-scroller.css'

createApp(App).use(router).use(VueVirtualScroller).mount("#app");

/*
    Note: Errors from tauri cannot be handled using global error handler 
    provided by vue(`app.config.errorHandler`).
*/
