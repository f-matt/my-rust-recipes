/**
 * main.ts
 *
 * Bootstraps Vuetify and other plugins then mounts the App`
 */

// Plugins
import { registerPlugins } from '@/plugins'
import axios from 'axios';

// Components
import App from './App.vue'

// Composables
import { createApp } from 'vue'

const app = createApp(App)

registerPlugins(app)

app.mount('#app')

axios.interceptors.request.use((config) => {
  console.log("Request interceptor");
  console.log(config);
  return config;
});

axios.interceptors.response.use((response) => {
  console.log("Response interceptor");
  return response;
}, (error)=> {
  console.log("Response interceptor error");
  return Promise.reject(error);
});
