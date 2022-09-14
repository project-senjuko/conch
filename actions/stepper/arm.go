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
	"io/fs"
	"os"
	"path/filepath"
	"sync"

	"gopkg.in/yaml.v3"
)

func updateVERSION(_, n *VersionConf) (err error) {
	const d = "VERSION.yml"
	fmt.Println("正在更新 " + d)

	f, err := os.OpenFile("../../VERSION.yml", os.O_WRONLY|os.O_TRUNC, 0200)
	if err != nil {
		return errors.New("打开 " + d + " 写入流失败：" + err.Error())
	}
	defer f.Close()

	err = yaml.NewEncoder(f).Encode(n)
	if err != nil {
		return errors.New("写入 " + d + " 失败：" + err.Error())
	}
	return
}

func updateJceVersion(o, n *VersionConf) (err error) {
	const d = "jce version.yml"
	fmt.Println("正在更新 " + d)

	return ReadReplaceAndWrite(
		"../../cookie/network/protocol/jce/struct/version.yml",
		d,
		ReplaceCode(o, n, "  current: "),
	)
}

func updateJceStructVersion(o, n *VersionConf) (err error) {
	wg := sync.WaitGroup{}
	err = filepath.WalkDir(
		"../../cookie/network/protocol/jce/struct/blueprint",
		func(p string, d fs.DirEntry, err error) error {
			if err != nil {
				return errors.New("[ERR][updateJceStructVersion] " + err.Error())
			}
			if d.IsDir() {
				return nil
			}

			wg.Add(1)
			go _updateJceStructVersion(p, o, n, &wg)
			return nil
		},
	)
	if err != nil {
		return errors.New("遍历 jce struct 失败：" + err.Error())
	}

	wg.Wait()
	return
}

func _updateJceStructVersion(p string, o, n *VersionConf, wg *sync.WaitGroup) {
	fmt.Println("正在更新 " + p)

	err := ReadReplaceAndWrite(p, p, ReplaceCode(o, n, "  upstreamVersion: "))
	if err != nil {
		fmt.Println(err)
	}
	wg.Done()
}

func updateAppSetting(o, n *VersionConf) (err error) {
	const d = "AppSetting"
	fmt.Println("正在更新 " + d)

	return ReadReplaceAndWrite(
		"../../cookie/config/app_setting.rs",
		d,
		ReplaceCode(o, n, "/// APP_ID: 版本 | "),
		ReplaceAppId(o, n, "pub const APP_ID: u32 = "),
	)
}
