package main

import (
	"fmt"
	"os"

	"senjuko-conch/cell"
)

type Arm func(oldVersion, newVersion *cell.VersionConf) error

var arm = []Arm{updateVERSION, updateJceVersion, updateJceStructVersion, updateAppSetting}

func main() {
	v := cell.ReadVersionConf()
	nv := cell.FetchUpstreamVersion()

	if err := os.WriteFile("download_url", []byte(nv.DownloadURL), 0644); err != nil {
		fmt.Println("[ERR] ", err)
		return
	}
	if err := os.WriteFile("download_filename", []byte(nv.DownloadFileName), 0644); err != nil {
		fmt.Println("[ERR] ", err)
		return
	}

	fmt.Println("version: ", nv.Version, ", code: ", nv.Code, ", appid: ", nv.AppId)

	if nv.Code <= v.Code {
		fmt.Println("== 版本已同步，无需更新 ==")
		return
	}
	fmt.Println("== 开始更新 ==")

	onv := cell.VersionConf{
		Version: nv.Version,
		Code:    nv.Code,
		AppId:   nv.AppId,
	}

	for _, a := range arm {
		if err := a(v, &onv); err != nil {
			fmt.Println("[ERR] ", err)
		}
	}

	fmt.Println("== 更新完成 ==")
}
