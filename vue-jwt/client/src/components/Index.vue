<script setup lang="ts">
import router from "@/router";

function openLogin() {
  router.push({path : "/login"});
}

function openProtected() {
  router.push({path : "/protected"});
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
          Welcome!
        </div>
      </div>

      <v-row style="margin-top: 5rem">
        <v-col
          cols="6"
          style="text-align:center"
        >
          <v-btn
            text="Login"
            @click="openLogin"
          />
        </v-col>

        <v-col
          cols="6"
          style="text-align:center"
        >
          <v-btn
            text="Protected"
            @click="openProtected"
          />
        </v-col>
      </v-row>
    </v-responsive>
  </v-container>
</template>

<style scoped>

</style>
