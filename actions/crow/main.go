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
	"io"
	"os"
	"strconv"

	"senjuko-conch/cell"
)

const CIV = "../cache/ci.version"

func main() {
	cv := readCV()
	uv := cell.FetchUpstreamVersion()

	if uv.Code <= cv {
		fmt.Println("=== 当前已同步最新版本信息，无需更新")
		return
	} else {
		fmt.Println("=== 开始更新最新版本缓存")
	}

	if err := os.WriteFile(
		"../cache/ci.version",
		[]byte(strconv.FormatUint(uv.Code, 10)),
		0644,
	); err != nil {
		panic("[ERR] " + err.Error())
	}

	if err := os.WriteFile(
		"../temp/old.version",
		[]byte(strconv.FormatUint(cv, 10)),
		0644,
	); err != nil {
		panic("[ERR] " + err.Error())
	}
	if err := os.WriteFile("../temp/file.name", []byte(uv.DownloadFileName), 0644); err != nil {
		panic("[ERR] " + err.Error())
	}
	if err := os.WriteFile("../temp/url", []byte(uv.DownloadURL), 0644); err != nil {
		panic("[ERR] " + err.Error())
	}

	fmt.Println("=== 更新完成")
}

func readCV() (cv uint64) {
	cvf, err := os.Open(CIV)
	if err != nil {
		panic("[ERR] 打开缓存版本文件失败：" + err.Error())
	}

	cvb, err := io.ReadAll(cvf)
	if err != nil {
		panic("[ERR] 读取缓存版本文件失败：" + err.Error())
	}

	cv, err = strconv.ParseUint(string(cvb), 10, 64)
	if err != nil {
		panic("[ERR] 解析缓存版本文件失败：" + err.Error())
	}
	return
}
