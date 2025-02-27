# Mon Anniversaire

## prompt

```sh
cargo create-tauri-app -t vue-ts mon-anniversaire
```

prompt

```prompt
mon-anniversaire
功能描述：
1 创建纪念日：输入一个名称和一个日期，app提供将其保存为一个纪念日。
2 计算纪念日：每当再次打开这个纪念日，app计算距离当前日期间隔的天数，过去的使用橙色展示，未来的使用绿色。
3 计算日期：输入一个天数，选择过去或者未来，给出日期。
代码要求：
1 计算和存储均由Rust实现，TS单纯负责交互和UI。
2 给出每一个代码文件的全部内容，确保可以直接运行。
```

```sh
cd mon-anniversaire
yarn add -D @tauri-apps/cli@latest
yarn tauri dev
```
