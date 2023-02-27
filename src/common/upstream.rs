////////////////////////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-present qianjunakasumi <i@qianjunakasumi.ren>                                /
//                            project-senjuko/conch Contributors                                   /
//                                                                                                 /
//           https://github.com/qianjunakasumi                                                     /
//           https://github.com/project-senjuko/conch/graphs/contributors                          /
//                                                                                                 /
//   This Source Code Form is subject to the terms of the Mozilla Public                           /
//   License, v. 2.0. If a copy of the MPL was not distributed with this                           /
//   file, You can obtain one at http://mozilla.org/MPL/2.0/.                                      /
////////////////////////////////////////////////////////////////////////////////////////////////////

//! # QQ 上游版本控制

/// # APP ID
///
/// ## For Conch Action
/// @{id}=APP_ID
/// @{source}=com.tencent.common.config.AppSetting.f
/// @{version}=10155
pub const APP_ID: u32 = 537147618;

pub const APP_SHORT_VER: &str = "8.9.28";

/// # APP 提交哈希
///
/// ## For Conch Action
/// @{id}=APP_COMMIT
/// @{source}=com.tencent.common.config.AppSetting.c
/// @{version}=10155
pub const APP_COMMIT: &str = "79a4d4b7";

/// # APP 构建时间
///
/// ## For Conch Action
/// @{id}=BUILD_TIME
/// @{source}=oicq.wlogin_sdk.tools.C132433util.BUILD_TIME
/// @{version}=10000
pub const BUILD_TIME: u32 = 1669100372;

/// # SDK 版本
///
/// ## For Conch Action
/// @{id}=SDK_VERSION
/// @{source}=oicq.wlogin_sdk.tools.C132433util.SDK_VERSION
/// @{version}=10000
pub const SDK_VERSION: &str = "6.0.0.2530";

/// # Apk 包名
///
/// 原则上生命周期内不发生变更。
pub const APK_NAME: &str = "com.tencent.mobileqq";

/// # Apk 签名 MD5
///
/// 原则上生命周期内不发生变更。
pub const APK_SIGNATURE_MD5: [u8; 0x10] = [
    0xA6, 0xB7, 0x45, 0xBF, 0x24, 0xA2, 0xC2, 0x77, 0x52, 0x77, 0x16, 0xF6, 0xF3, 0x6E, 0xB6, 0x8D
];
