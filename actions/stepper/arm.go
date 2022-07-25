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
	"os"

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
