<script setup lang="ts">
import axios from 'axios';
import { ref } from "vue";
import router from "@/router";
import { login } from "@/utils/auth";

const username = ref("");
const password = ref("");

function handleLogin() {
  axios
    .post("/api/login", { "username": username.value, "password" : password.value})
    .then((res) => {
      login(JSON.stringify(res.data));
      router.push({ path: "/protected"});
    })
    .catch((error) => {
      console.log("Error: ", error);
      if (error.response.status === 403) {
        alert("Invalid username/password.");
        return;
      }

      if (error.response.status === 400 && error.response.data) {
        alert(error.response.data);
        return;
      }

      alert("Could not login: unknown error.");
    });
}

function back() {
  router.push({ path: "/"});
}

</script>

<template>
  <v-container class="fill-height">
    <v-responsive
      class="align-center fill-height mx-auto"
      max-width="900"
    >
      <div class="text-center">
        <div class="text-h2 font-weight-bold">
          Login
        </div>
      </div>

      <v-form style="margin-top: 2rem">
        <v-row>
          <v-col cols="12">
            <v-text-field
              v-model="username"
              label="Username:"
              required
            />
          </v-col>

          <v-col cols="12">
            <v-text-field
              v-model="password"
              label="Password:"
              type="password"
              required
            />
          </v-col>

          <v-col
            cols="6"
            class="text-center"
          >
            <v-btn
              text="Login"
              @click="handleLogin"
            />
          </v-col>

          <v-col
            cols="6"
            class="text-center"
          >
            <v-btn
              text="Back"
              @click="back"
            />
          </v-col>
        </v-row>
      </v-form>
    </v-responsive>
  </v-container>
</template>

<style scoped>

</style>
