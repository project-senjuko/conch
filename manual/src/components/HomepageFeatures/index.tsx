import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

type FeatureItem = {
  title: string;
  description: JSX.Element;
};

const FeatureList: FeatureItem[] = [
  {
    title: '易于使用',
    description: (
      <>
          Conch 海螺 采用全新设计思想，将通讯部分与您的逻辑代码分离并独立运行，充分发挥云上微服务优势。各微服务间采用 NATS
          消息队列和 GraphQL
          API 查询语言通讯，简化手动编写通讯代码，您只需要关注您的代码逻辑即可享受 Conch 海螺提供的各种便利。事件格式将精心设计，
          以满足大部分情况下优雅地处理事件。 Conch 海螺还将内置基于 Web 访问的 Dashboard，为您提供可视化的运维体验。
      </>
    ),
  },
  {
    title: '关注上云',
    description: (
      <>
        Conch 海螺 为云原生应用设计，提供基于 Jaeger 的分布式追踪、支持 Prometheus 的指标收集、采用通用的 OpenTelemetry 协议、符合
        CloudEvents 的事件格式。内置 Helm 和 Docker 支持，方便您在 Kubernetes 等容器编排系统中部署。采用 Rust 编写代码，
        提供优良的运行稳定性，支持交叉编译、多平台的强一致性。
      </>
    ),
  },
  {
    title: '社区共治',
    description: (
      <>
          Conch 海螺 是社区治理项目，我们高度重视项目的社区化，以吸纳更多建设性意见与批评。任何人都能积极参与至本项目，而不论贡献的何种或大小，
          推动 Conch 海螺的可持续化发展。如果您有意愿，请积极参与！访问 <a
          href={"https://github.com/project-senjuko/conch/blob/main/CONTRIBUTING.md"}> GitHub
          上的贡献部分说明 </a>
          以了解我们对您贡献的期望。目前 Conch 海螺 还是新生项目，众多功能丞待实现与完善，您的任何贡献都将推动我们大进步！
      </>
    ),
  },
];

function Feature({title, description}: FeatureItem) {
  return (
    <div className={clsx('col col--4')}>
      <div className="padding-horiz--md">
        <h3>{title}</h3>
        <p>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures(): JSX.Element {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
