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
import {CLIENT_LOGIN_URL, getAccessToken, getRefreshToken, login, LOGIN_URL, REFRESH_URL} from "@/utils/auth";

const app = createApp(App)

registerPlugins(app)

app.mount('#app')

axios.interceptors.request.use((config) => {
  if (config.url && config.url == LOGIN_URL)
    return config;

  if (config.url && config.url == REFRESH_URL) {
    const refreshToken = getRefreshToken();
    if (refreshToken)
      config.headers.Authorization = `Bearer ${refreshToken}`;
  } else {
    const accessToken = getAccessToken();
    if (accessToken)
      config.headers.Authorization = `Bearer ${accessToken}`;
  }

  return config;
});

axios.interceptors.response.use(
  (response) => {
    return response;
  },
  async (error)=> {
    // Other response error, unrelated to token expiration
    if (error.response?.status !== 401)
      return Promise.reject(error);

    // Error trying to refresh the token, should logout and redirect to login page
    if (error.config.url === REFRESH_URL) {
      window.location.href = CLIENT_LOGIN_URL;
      alert("Token expired, please login again.");
      return Promise.reject(error);
    }

    // Save the previous config, to try it one more time if successful refresh
    const previousConfig = error.config;

    // If already retried once, abort
    if (previousConfig._retry)
      return Promise.reject(error);

    // Try to refresh the access token
    previousConfig._retry = true;

    try {
      const refreshResponse = await axios.post(REFRESH_URL);
      login(JSON.stringify(refreshResponse.data));
      return axios(previousConfig);
    } catch (refreshError) {
      return Promise.reject(refreshError);
    }
  }
);
