// @ts-check
// Note: type annotations allow type checking and IDEs autocompletion

const lightCodeTheme = require('prism-react-renderer/themes/github');
const darkCodeTheme = require('prism-react-renderer/themes/dracula');

/** @type {import('@docusaurus/types').Config} */
const config = {
  title: 'Conch 海螺 - 云原生中的 QQ 通讯',
  tagline: '基于 NATS 和 gRPC 的事件发布者',
  url: 'https://qianjunakasumi.github.io',
  baseUrl: '/senjuko-conch/',
  onBrokenLinks: 'throw',
  onBrokenMarkdownLinks: 'warn',

  organizationName: 'qianjunakasumi',
  projectName: 'senjuko-conch',

  i18n: {
    defaultLocale: 'zh-Hans',
    locales: ['zh-Hans'],
  },

  presets: [
    [
      'classic',
      /** @type {import('@docusaurus/preset-classic').Options} */
      ({
        docs: {
          sidebarPath: require.resolve('./sidebars.js'),
          editUrl: 'https://github.com/qianjunakasumi/senjuko-conch/tree/main/manual/',
        },
        blog: {
          showReadingTime: true,
          editUrl: 'https://github.com/qianjunakasumi/senjuko-conch/tree/main/manual/',
        },
        theme: {
          customCss: require.resolve('./src/css/custom.css'),
        },
      }),
    ],
  ],

  themeConfig:
    /** @type {import('@docusaurus/preset-classic').ThemeConfig} */
    ({
      navbar: {
        hideOnScroll: true,
        title: 'Conch 海螺',
        items: [
          {
            type: 'doc',
            docId: 'quick-start',
            position: 'left',
            label: '指南',
          },
          {to: '/blog', label: '博客', position: 'left'},
          {
            href: 'https://github.com/qianjunakasumi/senjuko-conch',
            position: 'right',
            className: 'header-github-link',
            'aria-label': 'GitHub 仓库',
          },
        ],
      },
      footer: {
        links: [
          {
            title: '文档',
            items: [
              {
                label: '指南',
                to: '/docs/quick-start',
              },
            ],
          },
          {
            title: '社区',
            items: [
              {
                label: 'GitHub Discussions',
                href: 'https://github.com/qianjunakasumi/senjuko-conch/discussions',
              },
            ],
          },
          {
            title: '更多',
            items: [
              {
                label: '博客',
                to: '/blog',
              },
              {
                label: 'GitHub',
                href: 'https://github.com/qianjunakasumi/senjuko-conch',
              },
            ],
          },
        ],
        copyright: `Copyright © ${new Date().getFullYear()} qianjunakasumi ❤ Docusaurus <br />本作品采用 <a rel="license" href="http://creativecommons.org/licenses/by-sa/4.0/">知识共享署名-相同方式共享 4.0 国际许可协议 <img alt="知识共享许可协议" style="border-width:0" src="https://i.creativecommons.org/l/by-sa/4.0/80x15.png" /></a> 进行许可`,
      },
      prism: {
        theme: lightCodeTheme,
        darkTheme: darkCodeTheme,
      },
    }),
};

module.exports = config;
