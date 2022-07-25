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
	"errors"
	"fmt"
	"io"
	"os"
	"strconv"
	"strings"

	"gopkg.in/yaml.v3"
)

func updateVERSION(_ *VersionConf, n *VersionConf) (err error) {
	const d = "VERSION.yml"
	fmt.Println("正在更新 " + d)

	f, err := os.OpenFile("../../VERSION.yml", os.O_WRONLY|os.O_TRUNC, 0200)
	if err != nil {
		return errors.New("打开 " + d + " 写入流失败：" + err.Error())
	}

	o, err := yaml.Marshal(n)
	if err != nil {
		return errors.New("生成 " + d + " 失败：" + err.Error())
	}

	_, err = f.Write(o)
	if err != nil {
		return errors.New("写入 " + d + " 失败：" + err.Error())
	}
	return
}

func updateJceVersion(o *VersionConf, n *VersionConf) (err error) {
	const d = "jce version.yml"
	fmt.Println("正在更新 " + d)

	const path = "../../crates/cookie/src/network/protocol/struct/jce/version.yml"
	f, err := os.OpenFile(path, os.O_RDONLY, 0444)
	if err != nil {
		return errors.New("打开 " + d + " 读取流失败：" + err.Error())
	}
	defer f.Close()

	fs, err := io.ReadAll(f)
	if err != nil {
		return errors.New("读取 " + d + " 失败：" + err.Error())
	}

	f, err = os.OpenFile(path, os.O_WRONLY|os.O_TRUNC, 0200)
	if err != nil {
		return errors.New("打开 " + d + " 写入流失败：" + err.Error())
	}
	defer f.Close()

	_, err = f.WriteString(
		strings.Replace(
			string(fs),
			"  current: "+strconv.FormatUint(o.Code, 10),
			"  current: "+strconv.FormatUint(n.Code, 10),
			1,
		),
	)
	if err != nil {
		return errors.New("写入 " + d + " 失败：" + err.Error())
	}
	return
}
