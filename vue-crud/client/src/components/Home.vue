<template>
  <v-container class="fill-height">
    <v-responsive
      class="align-centerfill-height mx-auto"
      max-width="900">

      <v-sheet>
        <v-row>
          <v-col cols="12">
            <h2>A sample CRUD in Vue.js</h2>
          </v-col>
        </v-row>

        <v-row>
          <v-col cols="12">
            <v-data-table :headers="headers" :items="foos">
              <template v-slot:top>
                <v-toolbar flat>
                  <v-toolbar-title>
                    Registered items
                  </v-toolbar-title>
                  <v-spacer></v-spacer>
                  <v-btn prepend-icon="mdi-plus" text="New" @click="add"></v-btn>
                </v-toolbar>
              </template>

              <template v-slot:item.actions="{ item }">
                <div>
                  <v-icon icon="mdi-pencil" size="small" @click="edit(item)"></v-icon>
                  <v-icon icon="mdi-delete" size="small" @click="remove(item)"></v-icon>
                </div>
              </template>
            </v-data-table>
          </v-col>
        </v-row>
      </v-sheet>

      <v-dialog v-model="dialog" max-width="500">
        <v-card title="Foo registration">
          <template v-slot:text>
            <v-row>

              <v-col cols="12" v-if="foo.id">
                <v-text-field v-model="foo.id" label="Id" readonly></v-text-field>
              </v-col>

              <v-col cols="12">
                <v-text-field v-model="foo.description" label="Description"></v-text-field>
              </v-col>
            </v-row>
          </template>

          <v-divider></v-divider>

          <v-card-actions>
            <v-btn text="Cancel" @click="dialog = false"></v-btn>
            <v-spacer></v-spacer>
            <v-btn text="Save" @click="save"></v-btn>
          </v-card-actions>
        </v-card>
      </v-dialog>
    </v-responsive>
  </v-container>
</template>

<script setup lang="ts">
import type { Foo } from '@/models/Foo';
import axios from 'axios';
import { onMounted, ref, shallowRef } from 'vue';

const headers = [
  { title: 'Id', key: 'id', align: 'left' },
  { title: 'Description', key: 'description', align: 'left' },
  { title: 'Actions', key: 'actions', align: 'left', sortable: false },
]

const isEditing = shallowRef(false);
const EMPTY_FOO = { id: undefined, description: '' };
const foo = ref<Foo>(EMPTY_FOO);
const dialog = shallowRef(false);

let foos = ref<Foo[]>([]);

onMounted(() => {
  refresh();
});

function refresh() {
  axios
    .get("/api/foos")
    .then((res) => {
      foos.value = res.data;
    })
    .catch((error) => {
      console.log(error);
    });
}

function add() {
  isEditing.value = false;
  foo.value = EMPTY_FOO;
  dialog.value = true;
}

function edit(f: Foo) {
  isEditing.value = true;

  foo.value = f;
  dialog.value = true;
}

function remove(f: Foo) {
  axios
      .delete("/api/delete-foo", {data: f})
      .then((res) => {
        refresh();
        alert ("Record succesfully deleted!");
      })
      .catch((error) => {
        console.log(error);
    });
}

function save() {
  if (foo.value.id) {
    axios
      .patch("/api/update-foo", foo.value)
      .then((res) => {
        foo.value = EMPTY_FOO;
        dialog.value = false;
        refresh();
        alert("Foo succesfully saved!");
      })
      .catch((error) => {
        console.log(error);
    });
  } else {
    axios
      .post("/api/create-foo", foo.value)
      .then((res) => {
        foo.value = EMPTY_FOO;
        dialog.value = false;
        refresh();
        alert("Foo succesfully saved!");
      })
      .catch((error) => {
        console.log(error);
    });
  }
}

</script>
