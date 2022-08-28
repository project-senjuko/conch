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
	"fmt"
	"os"
	"strconv"
	"strings"

	"gopkg.in/yaml.v3"
)

func UniversalRead(fp, k, dsc string) (r *os.File) {
	f, err := os.OpenFile(fp, os.O_RDONLY, 0444)
	if err != nil {
		panic("打开 " + dsc + " 文件失败：" + err.Error())
	}
	defer f.Close()
	r, _ = os.OpenFile(fp, os.O_RDONLY, 0444)

	a := BasicSpec{}
	if err := yaml.NewDecoder(f).Decode(&a); err != nil {
		panic("解析 " + dsc + " 文件失败：通用 Spec 解析失败" + err.Error())
	}

	v, err := strconv.ParseUint(strings.ReplaceAll(a.ApiVersion, "generator/v", ""), 10, 32)
	if err != nil {
		panic("解析 " + dsc + " 文件失败：通用 Spec 版本解析失败：" + err.Error())
	}

	if v > 10 { // v1.0
		panic("解析 " + dsc + " 文件失败：Spec 版本不兼容，请更新生成器或检查 Spec 是否正确")
	}
	if a.Kind != k {
		panic("解析 " + dsc + " 文件失败：错误的 Spec 类型，请检查是否正确")
	}
	return
}

func ReadConfigSpec() *ConfigSpec {
	const d = "配置"
	a := ConfigSpec{}

	_, err := os.Stat("config.yml")
	if err != nil && os.IsNotExist(err) {
		fmt.Println("配置文件不存在，将使用相对路径") // 默认项目根目录
		a.Spec.Source = "../struct/"
		a.Spec.Output = "../struct/"
		return &a
	}

	f := UniversalRead("config.yml", "Config", d)
	if err := yaml.NewDecoder(f).Decode(&a); err != nil {
		panic("解析 " + d + " 文件失败：" + err.Error())
	}
	defer f.Close()

	return &a
}

func ReadVersionSpec(fp string) *VersionSpec {
	const d = "版本"
	a := VersionSpec{}
	f := UniversalRead(fp, "Version", d)
	if err := yaml.NewDecoder(f).Decode(&a); err != nil {
		panic("解析 " + d + " 文件失败：" + err.Error())
	}
	defer f.Close()

	if a.Spec.Current < a.Spec.Minimal {
		panic("解析 " + d + " 文件失败：current < minimal")
	}

	return &a
}

func ReadJceSpec(fp string) *JceSpec {
	const d = "Jce"
	a := JceSpec{}
	f := UniversalRead(fp, "Jce", d)
	if err := yaml.NewDecoder(f).Decode(&a); err != nil {
		panic("解析" + d + "文件失败：" + err.Error())
	}
	defer f.Close()

	return &a
}
