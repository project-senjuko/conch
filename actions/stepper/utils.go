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
	"io"
	"os"
	"strconv"
	"strings"

	"senjuko-conch/cell"
)

func ReadReplaceAndWrite(p, d string, rp ...[2]string) (err error) {
	f, err := os.OpenFile(p, os.O_RDONLY, 0444)
	if err != nil {
		return errors.New("打开 " + d + " 读取流失败：" + err.Error())
	}
	defer f.Close()

	fb, err := io.ReadAll(f)
	if err != nil {
		return errors.New("读取 " + d + " 失败：" + err.Error())
	}

	f, err = os.OpenFile(p, os.O_WRONLY|os.O_TRUNC, 0200)
	if err != nil {
		return errors.New("打开 " + d + " 写入流失败：" + err.Error())
	}
	defer f.Close()

	b := string(fb)
	for _, v := range rp {
		b = strings.Replace(b, v[0], v[1], 1)
	}

	_, err = f.WriteString(b)
	if err != nil {
		return errors.New("写入 " + d + " 失败：" + err.Error())
	}
	return
}

func ReplaceCode(o, n *cell.VersionConf, prefix string) [2]string {
	return [2]string{
		prefix + strconv.FormatUint(o.Code, 10),
		prefix + strconv.FormatUint(n.Code, 10),
	}
}

func ReplaceAppId(o, n *cell.VersionConf, prefix string) [2]string {
	return [2]string{
		prefix + strconv.FormatUint(o.AppId, 10),
		prefix + strconv.FormatUint(n.AppId, 10),
	}
}
