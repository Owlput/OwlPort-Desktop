import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import { Quasar,Notify } from "quasar";
import "@quasar/extras/material-icons/material-icons.css";
import "quasar/src/css/index.sass";
import route from "./router/index";

createApp(App)
  .use(Quasar, {
    plugins: {Notify},
  })
  .use(route())
  .mount("#app");
