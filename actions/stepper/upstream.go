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
	"io"
	"net/http"
	"strconv"
	"strings"
)

func requestHTML() string {
	req, _ := http.NewRequest("GET", "https://im.qq.com/mobileqq/touch/android?arch=arm64", nil)
	req.Header.Set("user-agent", "Mozilla/5.0 (Linux; Android 13.0) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.5060.114 Mobile Safari/537.36 Edg/103.0.1264.62")

	res, err := http.DefaultClient.Do(req)
	if err != nil {
		panic("请求下载链接失败：" + err.Error())
	}
	defer res.Body.Close()

	bytes, err := io.ReadAll(res.Body)
	if err != nil {
		panic("读取下载页面失败：" + err.Error())
	}

	return string(bytes)
}

func readDownloadURL(b string) string {
	n := strings.LastIndex(b, "androidUrl64")
	b = b[n:]

	n = strings.Index(b, ":")
	b = b[n+2:] // :"

	n = strings.Index(b, `"`)
	b = b[:n]

	return strings.ReplaceAll(b, "\\u002F", "/")
}

func readVersion(b string) string {
	n := strings.LastIndex(b, "version64")
	b = b[n:]

	n = strings.Index(b, ":")
	b = b[n+2:] // :"

	n = strings.Index(b, `"`)
	return b[:n]
}

func parseDownloadURL(u string) (uint64, uint64) {
	s := strings.LastIndex(u, "/")
	e := strings.LastIndex(u, "_")
	us := strings.Split(u[s+1:e], "_")

	c, err := strconv.ParseUint(us[1][strings.LastIndex(us[1], ".")+1:], 10, 32)
	if err != nil {
		panic("解析 code 失败：" + err.Error())
	}
	a, err := strconv.ParseUint(us[2], 10, 64)
	if err != nil {
		panic("解析 appid 失败：" + err.Error())
	}

	return c, a
}
