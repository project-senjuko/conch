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

func UniversalOpen(fp, dsc string) *bufio.Reader {
	f, err := os.OpenFile(fp, os.O_RDONLY, 0444)
	if err != nil {
		panic("加载" + dsc + "文件失败：" + err.Error())
	}
	defer f.Close()

	return bufio.NewReader(f)
}

func UniversalRead(s BasicReader, k, dsc string) {
	a := s.getApiVersion()
	v, err := strconv.ParseUint(strings.TrimPrefix(a, "generator/v"), 10, 32)
	if err != nil {
		panic("解析通用 Spec 版本失败：" + err.Error())
	}

	if v > 10 {
		panic("Spec 版本不兼容，请更新生成器或检查 Spec 是否正确")
	}
	if s.getKind() != k {
		panic("解析" + dsc + "文件失败：错误的 Spec 类型，请检查")
	}
}

func ReadConfigSpec() *ConfigSpec {
	const d = "配置"
	a := ConfigSpec{}
	if err := yaml.NewDecoder(UniversalOpen("config.yml", d)).Decode(&a); err != nil {
		panic("解析" + d + "文件失败：" + err.Error())
	}
	UniversalRead(&a, "Config", d)

	return &a
}

func ReadVersionSpec(fp string) *VersionSpec {
	const d = "版本"
	a := VersionSpec{}
	if err := yaml.NewDecoder(UniversalOpen(fp, d)).Decode(&a); err != nil {
		panic("解析" + d + "文件失败：" + err.Error())
	}
	UniversalRead(&a, "Version", d)

	if a.Spec.Current < a.Spec.Minimal {
		panic("解析" + d + "文件失败：current < minimal")
	}

	return &a
}
