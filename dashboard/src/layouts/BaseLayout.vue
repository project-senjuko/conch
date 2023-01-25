<!----------------------------------------------------------------------------->
<!-- Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>           -->
<!--                     qianjunakasumi <qianjunakasumi@outlook.com>         -->
<!--                     https://github.com/qianjunakasumi                   -->
<!--                                                                         -->
<!--     This Source Code Form is subject to the terms of the Mozilla Public -->
<!--     License, v. 2.0. If a copy of the MPL was not distributed with this -->
<!--     file, You can obtain one at http://mozilla.org/MPL/2.0/.            -->
<!----------------------------------------------------------------------------->

<template>
  <q-layout view="hHh Lpr lff">
    <q-header>
      <q-toolbar>
        <ConchIcon/>
        <q-toolbar-title>
          Conch Dashboard
        </q-toolbar-title>
      </q-toolbar>
    </q-header>

    <q-page-container>
      <router-view/>
    </q-page-container>
  </q-layout>
</template>

<script lang="ts" setup>
import ConchIcon from 'components/icons/ConchIcon.vue';</script>
