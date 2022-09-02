package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strings"

	"gopkg.in/yaml.v3"
)

type VersionConf struct {
	Version string `yaml:"version"`
	Code    uint64 `yaml:"code"`
	AppId   uint64 `yaml:"appId"`
}

type Arm func(oldVersion, newVersion *VersionConf) error

var arm = []Arm{updateVERSION, updateJceVersion, updateJceStructVersion, updateAppSetting}

func main() {
	v := readVersionConf()

	body := requestHTML()
	url := readDownloadURL(body)
	us := strings.Split(url, "/")

	if err := os.WriteFile("download_url", []byte(url), 0644); err != nil {
		fmt.Println("[ERR] ", err)
		return
	}
	if err := os.WriteFile("download_filename", []byte(us[len(us)-1]), 0644); err != nil {
		fmt.Println("[ERR] ", err)
		return
	}

	code, appId := parseDownloadURL(readDownloadURL(body))
	nv := VersionConf{
		Version: readVersion(body),
		Code:    code,
		AppId:   appId,
	}

	fmt.Println("version: ", nv.Version, ", code: ", nv.Code, ", appid: ", nv.AppId)

	if code <= v.Code {
		fmt.Println("== 版本已同步，无需更新 ==")
		return
	}
	fmt.Println("== 开始更新 ==")

	for _, a := range arm {
		if err := a(v, &nv); err != nil {
			fmt.Println("[ERR] ", err)
		}
	}

	fmt.Println("== 更新完成 ==")
}

func readVersionConf() *VersionConf {
	f, err := os.OpenFile("../../VERSION.yml", os.O_RDONLY, 0444)
	if err != nil {
		panic("打开 VERSION.yml 文件失败：" + err.Error())
	}
	defer f.Close()
	fb, err := io.ReadAll(f)
	if err != nil {
		panic("读取 VERSION.yml 文件失败：" + err.Error())
	}

	a := VersionConf{}
	if err := yaml.NewDecoder(bytes.NewReader(fb)).Decode(&a); err != nil {
		panic("解析 VERSION.yml 文件失败：通用 Spec 解析失败" + err.Error())
	}
	return &a
}
