<script setup lang="ts">
import { onMounted, ref, watch } from "vue";
import { useRoute } from "vue-router";

import MainLayout from "@/components/MainLayout.vue";
import { useMainStore } from "@/stores/main";

import type { User as UserModel } from "@/models/api/user";
import axios from "axios";
import UserLink from "@/components/user/UserLink.vue";

const route = useRoute();

const mainStore = useMainStore();

const users = ref<UserModel[]>();
const error = ref<String|null>(null);

watch(route, () => {
  fetchUsers();
});

onMounted(async () => {
    await fetchUsers();
});

const fetchUsers = async () => {
  error.value = null;
  try {
      users.value = await mainStore.getAllPublicUsers();
  } catch (e) {
    if (axios.isAxiosError(e)) {
      if (e.response && e.response.status === 404) {
        error.value = 'No users found';
      } else {
        error.value = 'An unexpected error occurred';
      }
    }
  }
};

const formatDate = (isoString: string) => {
  return new Date(isoString).toLocaleString('en-NL', {
    dateStyle: 'medium',
    timeStyle: 'short',
  });
};

</script>

<template>
  <main>
    <MainLayout>
      <div v-if="users" class="content">
        <div class="title">All users</div>
        <table class="users-table">
          <thead>
            <tr>
              <th>ID</th>
              <th>Name</th>
              <th>Rank</th>
              <th>Join date</th>
              <th v-if="true">Actions</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="u of users" :key="u.id" class="tag">
              <td>
                {{ u.id }}
              </td>
              <td>
                <UserLink :name="u.name" :key="u.id" class="user">
                    <span class="username" :class="{ admin: u.rank > 0 }">
                      <span v-if="u.rank <= 0"><i class="fa-solid fa-user"></i></span>
                      <span v-if="u.rank > 0"><i class="fa-solid fa-crown"></i></span>
                      {{ u.name }}
                    </span>
                  </UserLink>
              </td>
              <td>
                {{ u.rank }}
              </td>
              <td>
                {{ formatDate(u.created_at) }}
              </td>
              <td>
                nothing yet
              </td>
            </tr>
          </tbody>
        </table>
      </div>
      <div v-if="error">
        {{ error }}
      </div>
    </MainLayout>
  </main>
</template>

<style scoped lang="scss">
main {
  padding: 1rem;
}

.users-table {
  border-spacing: 0;

  padding-top: 1rem;
  padding-bottom: 1rem;

  th {
    background-color: var(--color-list-header-background);

    padding: 0.4rem;

    text-align: left;
  }

  tr {
    background-color: var(--color-list-background);

    &:nth-child(even) {
      background-color: var(--color-list-alt-background);
    }
  }

  td {
    padding: 0.1rem 0.4rem;

    height: 1.6rem;
  }

  td:not(:first-child) {
    border-left: 0.2rem solid var(--color-table-divider);
  }
}
.title {
  font-size: 2rem;
}
</style>
