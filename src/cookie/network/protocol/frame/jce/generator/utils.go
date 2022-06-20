////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022 qianjunakasumi <i@qianjunakasumi.ren>                    /
//                     qianjunakasumi <qianjunakasumi@outlook.com>             /
//                     https://github.com/qianjunakasumi                       /
//                                                                             /
//     This Source Code Form is subject to the terms of the Mozilla Public     /
//     License, v. 2.0. If a copy of the MPL was not distributed with this     /
//     file, You can obtain one at http://mozilla.org/MPL/2.0/.                /
////////////////////////////////////////////////////////////////////////////////

package main

import (
	"bufio"
	"os"
	"strconv"
	"strings"

	"gopkg.in/yaml.v3"
)

func UniversalRead(s BasicReader) {
	a := s.getApiVersion()
	v, err := strconv.ParseUint(strings.TrimPrefix(a, "generator/v"), 10, 32)
	if err != nil {
		panic("解析通用 Spec 版本失败：" + err.Error())
	}

	if v > 10 {
		panic("Spec 版本不兼容，请更新生成器或检查 Spec 是否正确")
	}
}

func ReadConfigSpec() *ConfigSpec {
	f, err := os.OpenFile("config.yml", os.O_RDONLY, 0444)
	if err != nil {
		panic("加载配置文件失败：" + err.Error())
	}
	defer f.Close()

	a := ConfigSpec{}
	err = yaml.NewDecoder(bufio.NewReader(f)).Decode(&a)
	if err != nil {
		panic("解析配置文件失败：" + err.Error())
	}
	UniversalRead(&a)

	if a.Kind != "Config" {
		panic("解析配置文件失败：错误的 Spec 类型，请检查")
	}

	return &a
}
