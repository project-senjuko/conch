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

package cell

import (
	"bytes"
	"io"
	"os"
)

type VersionConf struct {
	Version string `yaml:"version"`
	Code    uint64 `yaml:"code"`
	AppId   uint64 `yaml:"appId"`
}

func ReadVersionConf() *VersionConf {
	f, err := os.OpenFile("../../VERSION.yml", os.O_RDONLY, 0444)
	if err != nil {
		panic("打开 VERSION.yml 文件失败：" + err.Error())
	}
	defer f.Close()
	fb, err := io.ReadAll(f)
	if err != nil {
		panic("读取 VERSION.yml 文件失败：" + err.Error())
	}

	a := VersionConf{}
	if err := yaml.NewDecoder(bytes.NewReader(fb)).Decode(&a); err != nil {
		panic("解析 VERSION.yml 文件失败：通用 Spec 解析失败" + err.Error())
	}
	return &a
}
