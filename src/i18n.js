import { createI18n } from "vue-i18n";
import enUS from "./assets/i18n/en-US.json";
import zhCN from "./assets/i18n/zh-CN.json";

export const i18n = createI18n({
  locale: "en-US",
  globalInjection: true,
  messages:{
    en:enUS,
    zh:zhCN
  },
});
