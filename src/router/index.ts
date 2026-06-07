import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import { useAuthStore } from '@/stores/auth'

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    name: 'LockScreen',
    component: () => import('@/views/LockScreen.vue'),
    meta: { requiresAuth: false },
  },
  {
    path: '/keys',
    name: 'KeyList',
    component: () => import('@/views/KeyList.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/keys/:id',
    name: 'KeyDetail',
    component: () => import('@/views/KeyDetail.vue'),
    meta: { requiresAuth: true },
  },
  {
    path: '/settings',
    name: 'Settings',
    component: () => import('@/views/Settings.vue'),
    meta: { requiresAuth: true },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
})

router.beforeEach((to) => {
  const auth = useAuthStore()

  // If route requires auth and app is locked, redirect to lock screen
  if (to.meta.requiresAuth !== false && !auth.isUnlocked) {
    return { name: 'LockScreen' }
  }

  // If already unlocked and going to lock screen, redirect to keys
  if (to.name === 'LockScreen' && auth.isUnlocked) {
    return { name: 'KeyList' }
  }
})

export default router
