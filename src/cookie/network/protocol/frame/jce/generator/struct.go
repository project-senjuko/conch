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

type BasicReader interface {
	getApiVersion() string
	getKind() string
}

func (b *BasicSpec) getApiVersion() string { return b.ApiVersion }

func (b *BasicSpec) getKind() string { return b.Kind }

// === Spec

type ConfigSpec struct {
	*BasicSpec
	Spec ConfigKind `yaml:"spec"`
}

// === Kind

type ConfigKind struct {
	Source string `yaml:"source"`
	Output string `yaml:"output"`
}
