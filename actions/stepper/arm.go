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
	"io/fs"
	"os"
	"path/filepath"
	"strconv"
	"strings"
	"sync"

	"gopkg.in/yaml.v3"
)

func updateVERSION(_ *VersionConf, n *VersionConf) (err error) {
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

func updateJceVersion(o *VersionConf, n *VersionConf) (err error) {
	const d = "jce version.yml"
	fmt.Println("正在更新 " + d)

	const path = "../../crates/cookie/src/network/protocol/struct/jce/version.yml"
	f, err := os.OpenFile(path, os.O_RDONLY, 0444)
	if err != nil {
		return errors.New("打开 " + d + " 读取流失败：" + err.Error())
	}
	defer f.Close()

	fb, err := io.ReadAll(f)
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
			string(fb),
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

func updateJceStructVersion(o *VersionConf, n *VersionConf) (err error) {
	wg := sync.WaitGroup{}
	err = filepath.WalkDir(
		"../../crates/cookie/src/network/protocol/struct/jce/struct",
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

func _updateJceStructVersion(p string, o *VersionConf, n *VersionConf, wg *sync.WaitGroup) {
	fmt.Println("正在更新 " + p)

	f, err := os.OpenFile(p, os.O_RDONLY, 0444)
	if err != nil {
		fmt.Println("打开 "+p+" 读取流失败：", err.Error())
	}
	defer f.Close()

	fb, err := io.ReadAll(f)
	if err != nil {
		fmt.Println("读取 "+p+" 失败：", err.Error())
	}

	f, err = os.OpenFile(p, os.O_WRONLY|os.O_TRUNC, 0200)
	if err != nil {
		fmt.Println("打开 "+p+" 写入流失败：", err.Error())
	}
	defer f.Close()

	_, err = f.WriteString(
		strings.Replace(
			string(fb),
			"  upstreamVersion: "+strconv.FormatUint(o.Code, 10),
			"  upstreamVersion: "+strconv.FormatUint(n.Code, 10),
			1,
		),
	)
	if err != nil {
		fmt.Println("写入 "+p+" 失败：", err.Error())
	}
	wg.Done()
}
