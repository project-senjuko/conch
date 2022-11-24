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

	"senjuko-conch/cell"
)

func main() {
	cv := cell.ReadVersionConf()
	uv := cell.FetchUpstreamVersion()

	if uv.Code <= cv.Code {
		fmt.Println("=== 当前已同步最新版本信息")
	} else {
		fmt.Println("=== 开始写入最新版本信息")
	}

	writeFile("../temp/ci.version", []byte(strconv.FormatUint(uv.Code, 10)))
	writeFile("../temp/curr.version", []byte(strconv.FormatUint(cv.Code, 10)))
	writeFile("../temp/file.name", []byte(uv.DownloadFileName))
	writeFile("../temp/url", []byte(uv.DownloadURL))

	fmt.Println("=== 完成")
}

func writeFile(p string, b []byte) {
	if err := os.WriteFile(p, b, 0644); err != nil {
		panic("[ERR] " + err.Error())
	}
}
