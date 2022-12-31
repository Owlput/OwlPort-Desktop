import { createApp, ref } from "vue";
import "./style.css";
import "./components/overlays/overlays.css";
import App from "./App.vue";
import router from "./router/index";
import i18n from "./i18n";

let disabledPopupsTable = ref(new Map());
function setTable(k,v){
  disabledPopupsTable.value.set(k,v)
}
function deleteTable(k){
  disabledPopupsTable.value.delete(k)
}

createApp(App)
  .use(router)
  .use(i18n)
  .provide("disabledPopupsTable", 
    {disabledPopupsTable,setTable,deleteTable})
  .mount("#app");
