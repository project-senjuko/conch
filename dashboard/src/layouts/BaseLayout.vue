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
