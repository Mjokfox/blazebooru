import { createRouter, createWebHistory } from "vue-router";

import BrowseView from "@/views/BrowseView.vue";
import LoginView from "@/views/LoginView.vue";
import PostView from "@/views/PostView.vue";
import RegisterView from "@/views/RegisterView.vue";
import TagsView from "@/views/TagsView.vue";
import UploadProgressView from "@/views/UploadProgressView.vue";
import UploadView from "@/views/UploadView.vue";
import UserView from "@/views/UserView.vue";
import UserListView from "@/views/UserListView.vue";
import WikiListView from "@/views/WikiListView.vue";
import WikiView from "@/views/WikiView.vue";

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  scrollBehavior(to, from, savedPosition) {
    if (savedPosition) {
      return new Promise((resolve) => {
        setTimeout(() => {
          resolve(savedPosition);
        }, 100);
      });
    }
  },
  routes: [
    {
      path: "/",
      redirect: { name: "browse" },
    },
    {
      path: "/login",
      name: "login",
      component: LoginView,
    },
    {
      path: "/register",
      name: "register",
      component: RegisterView,
    },
    {
      path: "/browse",
      name: "browse",
      component: BrowseView,
    },
    {
      path: "/post/:id",
      name: "post",
      component: PostView,
      props: (r) => {
        return {
          ...r.params,
          id: Number(r.params.id),
        };
      },
    },
    {
      path: "/user/:name",
      name: "user",
      component: UserView,
      props: (r) => {
        return {
          ...r.params,
          name: r.params.name,
        };
      },
    },
    {
      path: "/users",
      name: "users",
      component: UserListView,
    },
    {
      path: "/tags",
      name: "tags",
      component: TagsView,
    },
    {
      path: "/upload",
      name: "upload",
      component: UploadView,
    },
    {
      path: "/upload-progress",
      name: "upload-progress",
      component: UploadProgressView,
    },
    {
      path: "/wiki",
      name: "wiki-list",
      component: WikiListView,
    },
    {
      path: "/wiki/:name",
      name: "wiki",
      component: WikiView,
      props: (r) => {
        return {
          ...r.params,
          name: r.params.name,
        };
      },
    },
  ],
});

export default router;
