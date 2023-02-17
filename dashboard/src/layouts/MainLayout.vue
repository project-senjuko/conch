<!------------------------------------------------------------------------------------------------->
<!-- Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                            -->
<!--                            project-senjuko/conch Contributors                               -->
<!--                                                                                             -->
<!--           https://github.com/qianjunakasumi                                                 -->
<!--           https://github.com/project-senjuko/conch/graphs/contributors                      -->
<!--                                                                                             -->
<!--   This Source Code Form is subject to the terms of the Mozilla Public                       -->
<!--   License, v. 2.0. If a copy of the MPL was not distributed with this                       -->
<!--   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                  -->
<!------------------------------------------------------------------------------------------------->

<template>
  <q-layout view="hHh Lpr lff">
    <q-header>
      <q-toolbar>
        <q-btn
          flat
          dense
          round
          icon="menu"
          aria-label="Menu"
          @click="toggleLeftDrawer"
        />

        <q-toolbar-title>
          Conch Dashboard
        </q-toolbar-title>
      </q-toolbar>
    </q-header>

    <q-drawer
      v-model="leftDrawerOpen"
      side="left"
      behavior="desktop"
      show-if-above
      bordered
    >
      <q-list>
      </q-list>
    </q-drawer>

    <q-page-container>
      <router-view />
    </q-page-container>
  </q-layout>
</template>

<script setup lang="ts">
import {ref} from 'vue';

const leftDrawerOpen = ref(false)
function toggleLeftDrawer() {
  leftDrawerOpen.value = !leftDrawerOpen.value
}
</script>
