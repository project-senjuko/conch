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

type BasicSpec struct {
	ApiVersion string `yaml:"apiVersion"`
	Kind       string `yaml:"kind"`
}

// === Spec

type ConfigSpec struct {
	Spec ConfigKind `yaml:"spec"`
}

type VersionSpec struct {
	Spec VersionKind `yaml:"spec"`
}

type JceSpec struct {
	Metadata struct {
		Name            string `yaml:"name"`
		UpstreamVersion uint32 `yaml:"upstreamVersion"`
		Source          string `yaml:"source"`
		Description     string `yaml:"description"`
	} `yaml:"metadata"`
	Spec JceKind `yaml:"spec"`
}

// === Kind

type ConfigKind struct {
	Source string `yaml:"source"`
	Output string `yaml:"output"`
}

type VersionKind struct {
	Current uint32 `yaml:"current"`
	Minimal uint32 `yaml:"minimal"`
}

type JceKind struct {
	StartTag uint8 `yaml:"startTag"`
	Field    []*struct {
		Name    string  `yaml:"name"`
		Type    string  `yaml:"type"`
		Tag     *uint8  `yaml:"tag"`
		Default *string `yaml:"default"`
	} `yaml:"field"`
}
