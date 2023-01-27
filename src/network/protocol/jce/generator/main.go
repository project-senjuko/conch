////////////////////////////////////////////////////////////////////////////////
// Copyright (c) 2022-2023 qianjunakasumi <i@qianjunakasumi.ren>               /
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
	"sort"
	"strings"
	"sync"
)

var (
	Con    *ConfigSpec
	ORGDir string
	Ver    *VersionSpec
	Wg     = sync.WaitGroup{}
	Rs     = make(map[string]string)
)

func main() {
	fmt.Print(`=== JceGenerator ===
`)
	fmt.Println("少女祈祷中...")

	Con = ReadConfigSpec()
	ORGDir = filepath.Dir(Con.Spec.Source)
	Ver = ReadVersionSpec(filepath.Join(ORGDir, "version.yml"))

	err := walk()
	if err != nil {
		panic("读取 Jce 文件失败：" + err.Error())
	}

	Wg.Wait()
	c := pack()
	Wg.Wait()

	fmt.Println("Done. 共计生成 ", c, " 个 JceStruct")
}

func walk() error {
	return filepath.WalkDir(filepath.Join(ORGDir, "blueprint"), func(p string, d fs.DirEntry, err error) error {
		if err != nil {
			fmt.Println("警告(逻辑) " + err.Error())
			return err
		}
		if d.IsDir() {
			return nil
		}

		Wg.Add(1)
		go func() {
			j := read(p)
			s := format(j)
			Rs[j.Metadata.Name] = s.String()
			Wg.Done()
		}()
		return nil
	})
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
		fmt.Println("信息(Jce) 请注意更新 " + p)
	}

	return j
}

func pack() int {
	keys := make([]string, 0)
	for k := range Rs {
		keys = append(keys, k)
	}
	sort.Strings(keys)

	s := strings.Builder{}
	s.WriteString(HEAD)
	for _, k := range keys {
		s.WriteString(`
////////////////////////////////////////////////////////////////////////////////
`)
		s.WriteString(Rs[k])
	}

	Wg.Add(1)
	go write(s)
	return len(keys)
}

func write(s strings.Builder) {
	fp := filepath.Join(filepath.Dir(Con.Spec.Output), "packet.rs")

	f, err := os.OpenFile(fp, os.O_WRONLY|os.O_TRUNC|os.O_CREATE, 0200)
	if err != nil && err == os.ErrExist {
		panic("打开 " + fp + " 写入流失败：" + err.Error())
	}

	_, err = f.WriteString(s.String())
	if err != nil {
		panic("写入 " + fp + " 失败：" + err.Error())
	}
	Wg.Done()
}
