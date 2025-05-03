import { createApp } from "vue";
import router from "./router";
import "./style.css";
import "@mdi/font/css/materialdesignicons.css";
import Entry from "./Entry.vue";
import VueVirtualScroller from "vue-virtual-scroller";
import "vue-virtual-scroller/dist/vue-virtual-scroller.css";
import { createVuetify } from "vuetify";

let vuetify = createVuetify({
  theme: {
    defaultTheme: "light",
  },
});

createApp(Entry).use(router).use(VueVirtualScroller).use(vuetify).mount("#app");

/*
    Note: Errors from tauri cannot be handled using global error handler 
    provided by vue(`app.config.errorHandler`).
*/
