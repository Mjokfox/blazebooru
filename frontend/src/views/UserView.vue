<script setup lang="ts">
import { computed, onMounted, ref, toRefs, watch } from "vue";
import { useRoute, useRouter } from "vue-router";

import MainLayout from "@/components/MainLayout.vue";
import UserInfo from "@/components/user/UserInfo.vue";

import { useAuthStore } from "@/stores/auth";
import { useMainStore } from "@/stores/main";

import type { AdminUpdateUser, UserUpdateUser, User as UserModel } from "@/models/api/user";
import axios from "axios";

const props = defineProps<{
  name: string;
}>();

const { name } = toRefs(props);

const route = useRoute();
const router = useRouter();

const authStore = useAuthStore();
const mainStore = useMainStore();

const user = ref<UserModel>();
const error = ref<String|null>(null);

const can_edit_profile = computed(() => authStore.isAdmin || authStore.userProfile?.id === user.value?.id);

watch(route, () => {
  fetchUser();
});

onMounted(async () => {
  await fetchUser();
});

const fetchUser = async () => {
  error.value = null;
  try {
    await authStore.isInitialized();
    if (authStore.userProfile?.name === name.value) {
      user.value = authStore.userProfile;
    } else {
      user.value = await mainStore.getPublicUserProfile(name.value);
    }
  } catch (e) {
    if (axios.isAxiosError(e)) {
      if (e.response && e.response.status === 404) {
        error.value = 'User not found';
      } else {
        error.value = 'An unexpected error occurred';
      }
    }
  }
};

const updateUser = async (update_user: UserUpdateUser | AdminUpdateUser) => {
  if (can_edit_profile.value && user.value) {
    await mainStore.updateUser(user.value.id, update_user);
    if (user.value.name !== update_user.name) {
      router.push({ name: "user", params: { name: update_user.name } });
    } else{ 
      await authStore.getUserProfile();
      user.value = authStore.userProfile;
    }
  }
};

const deleteUser = async () => {
  if (authStore.isAdmin && user.value){
    await mainStore.deleteUser(user.value.id);
    // Navigate back to the browse view
    router.push({ name: "browse" });
  }
};

</script>

<template>
  <main>
    <MainLayout>
      <div v-if="user" class="content">
        <div class="title">{{ user.name }}</div>
        <UserInfo :user="user" :isAdmin="authStore.isAdmin" 
        :can_edit_profile="can_edit_profile" 
        @delete="deleteUser" @update="updateUser" />
      </div>
      <div v-if="error">
        {{ error }}
      </div>
    </MainLayout>
  </main>
</template>

<style scoped lang="scss">
main {
  padding: 2rem;
}

.content {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
}

.title {
  font-size: 2rem;
}
</style>
