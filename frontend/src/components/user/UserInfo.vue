<script setup lang="ts">
import { ref, toRefs } from "vue";

import ConfirmDialog from "@/components/common/ConfirmDialog.vue";
import { UserUpdateUser, AdminUpdateUser, User } from "@/models/api/user";

const props = defineProps<{
  user: User;
  isAdmin: boolean;
  can_edit_profile: boolean;
}>();

const emit = defineEmits<{
  (e: "delete"): void;
  (e: "update", request: UserUpdateUser | AdminUpdateUser): void;
}>();

const { user, isAdmin, can_edit_profile: can_edit_profile } = toRefs(props);

const confirmDelete = ref<typeof ConfirmDialog>();

const formatDate = (isoString: string) => {
  return new Date(isoString).toLocaleString('en-NL', {
    dateStyle: 'long',
    timeStyle: 'short',
  });
};

const editing = ref<User>();

const toggleEdit = () => {
  if (!editing.value) {
    editing.value = {
      id: user.value.id,
      created_at: user.value.created_at,
      name: user.value.name,
      rank: user.value.rank,
      biography: user.value.biography,
      css: user.value.css,
    };
  } else {
    cancelEdit();
  }
};

const cancelEdit = () => {
  editing.value = undefined;
};

const update = () => {
  const _editing = editing.value;
  if (!_editing) {
    return;
  }

  let update_user: AdminUpdateUser | UserUpdateUser;

  if (isAdmin.value) {
    update_user = {
      name: _editing.name,
      rank: _editing.rank,
      biography: _editing.biography,
      css: _editing.css
    };
  } else {
    update_user = {
      name: _editing.name,
      biography: _editing.biography,
      css: _editing.css
    };
  }

  emit("update", update_user);

  editing.value = undefined;
};

const deleteUser = () => {
  emit("delete");
};

</script>

<template>
  <div class="user-info">
    Joined: {{ formatDate(user.created_at) }} <br>
    id: {{ user.id }}, rank: {{ user.rank }}
    <hr />
    <h3>About me</h3>
    <div v-if="!(editing) && user.biography" class="user-bio">
      {{ user.biography }}
    </div>
    <div v-if="!user.biography" class="user-bio-empty">Nothing here yet..</div>
    <form v-if="can_edit_profile && editing" class="edit-form" @submit.prevent="update">
      <label>username</label>
      <input name="user_name" type="text" v-model="editing.name">
      <template v-if="isAdmin">
        <label>rank</label>
        <input name="user_rank" type="number" v-model="editing.rank"></input>
      </template>
      <label>About me</label>
      <textarea
        class="editing-field"
        name="user_biography"
        v-model="editing.biography"
        placeholder="About myself.."
        wrap="soft"
      ></textarea>

      <label>Custom css</label>
      <textarea
        class="editing-field"
        name="user_css"
        v-model="editing.css"
        placeholder=".title { color:red; }"
        wrap="soft"
      ></textarea>

      <div class="form-buttons">
        <input class="cancel-button" type="button" value="Cancel" @click="cancelEdit" />
        <input class="save-button" type="submit" value="Save" />
      </div>
    </form>
    <hr />
    <div class="actions">
      <button  class="edit-button link-button" @click="toggleEdit">
        <i class="fa-solid fa-pen-to-square"></i> Edit
      </button>
      <button v-if="isAdmin" class="delete-button link-button" @click="confirmDelete?.show()">
        <i class="fa-solid fa-trash"></i> Delete user
      </button>
    </div>
    <ConfirmDialog v-if="isAdmin" ref="confirmDelete" title="Confirm delete" @confirm="deleteUser">
      <template v-slot:confirm><i class="fa-solid fa-trash"></i> Delete</template>
      <div class="confirm-delete-dialog">Are you sure you want to delete the user "{{ user.name }}"?</div>
    </ConfirmDialog>
  </div>
</template>

<style scoped lang="scss">
.post-info {
  display: flex;
  flex-direction: column;
  gap: 0.4rem;
}

.actions {
  flex-grow: 1;

  display: flex;
  flex-direction: row;
  justify-content: end;
  gap: 1rem;
}

.title {
  font-size: 2rem;

  text-overflow: ellipsis;
  white-space: nowrap;
  overflow: hidden;
}

.editing {
  margin: 0.5rem 0;
}

.user-bio {
  min-height: 2em;
}

.user-bio-empty {
  color: gray
}

// --- Edit form ---
.edit-form {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.editing-field {
  resize: both;

  width: 30rem;
  height: 10rem;

  max-width: 100%;
}

.form-buttons {
  display: flex;
  flex-direction: row;
  align-self: end;
  gap: 1rem;
}

.submit-button {
  margin-top: 1rem;
}
</style>
