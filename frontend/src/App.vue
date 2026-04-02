<script setup lang="ts">
import { useUserStore } from '@/stores/userStore';
import UserList from './components/UserList.vue';
import type { UserPayload } from './types';
import {ref} from 'vue';
const store = useUserStore();
const username = ref("");
const email = ref("");

const addUser = async() => {
  if (!username.value || !email.value) {
    console.log("nothing");
    return;
  }

  let payload: UserPayload =  {
    name: username.value,
    email: email.value,
  }
  store.addUser(payload);
  username.value = "";
  email.value = "";

}

</script>

<template >
  <div class="mx-auto lg:w-270  xl:w-370">
  <div class="mx-8 px-6 py-4 my-8 bg-amber-50 rounded-lg shadow-sm  flex max-sm:flex-col gap-4 justify-around`">
    <input v-model.trim="username" class="text-amber-800 placeholder:text-amber-800/70 w-full transition hover:-translate-0.5 hover:scale-105 hover:shadow hover:shadow-amber-700 py-2 px-3 mx-1 outline outline-amber-500 bg-amber-100 rounded-lg focus:outline-amber-500 focus:outline-2" placeholder="user name" />
    <input type="email" v-model.trim="email" class="text-amber-800 placeholder:text-amber-800/70 w-full transition hover:-translate-0.5 hover:scale-105 hover:shadow hover:shadow-amber-700 py-2 px-3 mx-1 outline outline-amber-500 bg-amber-100 rounded-lg focus:outline-amber-500 focus:outline-2"
      placeholder="email: xyz@abc.com" />
    <button @click="addUser" class="max-sm:w-full cursor-pointer transition hover:-translate-0.5 hover:scale-105 hover:bg-amber-700 w-100 bg-amber-700/90 text-white py-2 mx-1 rounded-xl"> Add user</button>
  </div>

  <UserList />
  </div>
</template>

<style></style>
