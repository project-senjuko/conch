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
	"io/fs"
	"os"
	"path/filepath"
	"strings"
	"sync"
)

var Ver *VersionSpec

func main() {
	fmt.Println("少女祈祷中...")

	con := ReadConfigSpec()
	dir := filepath.Dir(con.Spec.Source)
	Ver = ReadVersionSpec(filepath.Join(dir, "version.yml"))
	wg := sync.WaitGroup{}

	err := filepath.WalkDir(filepath.Join(dir, "struct"), func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			fmt.Println("警告[遍历途中] | " + err.Error())
			return err
		}
		if d.IsDir() {
			return nil
		}

		wg.Add(1)
		go func() { // 考虑协程池
			j := read(p)
			s := format(j)
			o := strings.ReplaceAll(p, filepath.Join(dir, "struct"), filepath.Dir(con.Spec.Output))
			o = o[:len(o)-3]
			write(s, o+"rs")
			wg.Done()
		}()

		return nil
	})
	if err != nil {
		fmt.Println("警告[遍历结束] | " + err.Error())
	}
	wg.Wait()

	// TODO 统计信息

	fmt.Println("Done.")
}

func read(p string) *JceSpec {
	j := ReadJceSpec(p)
	if j.Metadata.UpstreamVersion > Ver.Spec.Current {
		panic("请检查版本文件是否未更新：upstreamVersion > current")
	}
	if j.Metadata.UpstreamVersion < Ver.Spec.Minimal {
		panic("Jce 版本滞后于最低容忍版本，请更新 Jce")
	}
	if j.Metadata.UpstreamVersion != Ver.Spec.Current {
		fmt.Println("信息[Jce] | 请注意更新 " + p)
	}

	return j
}

func write(s strings.Builder, fp string) {
	err := os.MkdirAll(filepath.Dir(fp), 0200)
	if err != nil {
		panic("创建 " + fp + " 所属文件夹失败：" + err.Error())
	}

	f, err := os.OpenFile(fp, os.O_WRONLY|os.O_CREATE, 0200)
	if err != nil && err == os.ErrExist {
		panic("打开 " + fp + " 写入流失败：" + err.Error())
	}

	_, err = f.WriteString(s.String())
	if err != nil {
		panic("写入 " + fp + " 失败：" + err.Error())
	}
}
