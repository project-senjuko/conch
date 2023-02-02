import {RouteRecordRaw} from 'vue-router';

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: () => import('layouts/MainLayout.vue'),
    children: [{path: '', component: () => import('pages/ErrorNotFound.vue')}],
  },
  {
    path: '/login',
    component: () => import('layouts/BaseLayout.vue'),
    children: [{path: '', component: () => import('pages/TotpLogin.vue')}],
  },
  {
    path: '/init',
    component: () => import('layouts/BaseLayout.vue'),
    children: [{path: '', component: () => import('pages/ConchInit.vue')}],
  },

  // Always leave this as last one,
  // but you can also remove it
  {
    path: '/:catchAll(.*)*',
    component: () => import('pages/ErrorNotFound.vue'),
  },
];

export default routes;
