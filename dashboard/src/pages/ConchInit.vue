<template>
  <q-page class="flex flex-center">
    <q-card flat>
      <q-form
        class="flex column q-gutter-md"
        @submit="onSubmit"
      >
        <q-input
          v-model="name"
          :rules="[ (val) => (val && val.length > 0) || '请输入帐号' ]"
          autofocus
          class="input"
          filled
          label="帐号"
        >
          <template v-slot:prepend>
            <q-icon name="person"/>
          </template>
        </q-input>

        <q-input
          v-model="password"
          :rules="[ (val) => (val && val.length > 0) || '请输入密码' ]"
          :type="visible ? 'text' : 'password'"
          class="input"
          filled
          label="密码"
        >
          <template v-slot:prepend>
            <q-icon name="lock"/>
          </template>
          <template v-slot:append>
            <q-icon
              :name="visible ? 'visibility' : 'visibility_off'"
              class="cursor-pointer"
              @click="visible = !visible"
            />
          </template>
        </q-input>

        <q-btn
          class="q-py-sm"
          color="primary"
          label="登录"
          type="submit"
          unelevated
        />
      </q-form>
    </q-card>
  </q-page>
</template>

<script lang="ts" setup>
import {ref} from 'vue'

const visible = ref(false)
const name = ref('')
const password = ref('')

function onSubmit() {
  console.log("提交");
  // todo 发送登录请求至后端
}
</script>

<style scoped>
.input {
  width: 440px;
}
</style>
