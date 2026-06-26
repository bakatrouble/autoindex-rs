import { createApp } from 'vue'
import App from './App.vue'
import { VueQueryPlugin } from "@tanstack/vue-query";
import './style.css';



createApp(App)
    .use(VueQueryPlugin)
    .mount(document.body);
