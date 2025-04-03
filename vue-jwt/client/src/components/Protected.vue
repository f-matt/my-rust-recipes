<script setup lang="ts">
import router from "@/router";
import axios from "axios";
import {onMounted, ref} from "vue";
import {logout} from "@/utils/auth";

const serverTime = ref("");

function back() {
  router.push({path : "/"});
}

function handleLogout() {
  logout();
  router.push({ path: "/" });
}

onMounted(() => {
  axios
    .get("/api/protected")
    .then((res) => {
      if (res && res.status === 200 && res.data)
        serverTime.value = res.data;
    })
    .catch((error) => {
      if (error.status === 401) {
        logout();
        router.push({ path: "/login" });
        alert("Token expired, please login again.");
        return;
      }

      console.log("Error: ", error);
      alert("Error retrieving server time.");
    });
});
</script>

<template>
  <v-container class="fill-height">
    <v-responsive
      class="align-center fill-height mx-auto"
      max-width="900"
    >
      <div class="text-center">
        <div class="text-h2 font-weight-bold">
          Only authenticated users should access this page...
        </div>

        <div>
          Server time is: {{ serverTime }}.
        </div>
      </div>

      <v-row style="margin-top: 5rem">
        <v-col
          cols="6"
          style="text-align:center"
        >
          <v-btn
            text="Back"
            @click="back"
          />
        </v-col>

        <v-col
          cols="6"
          style="text-align:center"
        >
          <v-btn
            text="Logout"
            @click="handleLogout"
          />
        </v-col>
      </v-row>
    </v-responsive>
  </v-container>
</template>

<style scoped>

</style>
